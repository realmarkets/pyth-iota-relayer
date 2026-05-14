//! `start` — the relay daemon. Runs until SIGINT or SIGTERM.
//!
//! Loop body: fetch on-chain `PriceInfoObject`s + Hermes parsed
//! prices, run the (stateless) hybrid trigger, then for each firing
//! chunk submit a Pyth update PTB *in parallel*. Each chunk gets its
//! own hot gas coin so the txs don't version-collide. Pool warm-up
//! happens once at startup; rebalance triggers after a tick if any
//! hot coin fell below the configured minimum.

use std::path::Path;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use anyhow::{anyhow, Result};
use iota_sdk_crypto::ed25519::Ed25519PrivateKey;
use iota_sdk_graphql_client::Client;
use iota_sdk_types::{Address, ObjectReference};
use pyth_hermes_client::{FeedId, HermesClient};
use tokio::signal::unix::{signal, SignalKind};
use tokio::time::interval;
use tracing::{debug, info, warn};

use crate::cli::Network;
use crate::feeds;
use crate::network::Contracts;
use crate::on_chain::{resolve_price_info_ids, OnChainReader};
use crate::pyth_update::PythUpdater;
use crate::relayer_coins::{self, GasPoolConfig};
use crate::signer;
use crate::trigger::{self, Fire, Reason};

const TICK: Duration = Duration::from_secs(1);

#[derive(Debug, Clone, Copy)]
pub struct Options {
    pub max_feeds_per_tx: usize,
    pub gas_coin_target_niota: u64,
    pub gas_coin_min_niota: u64,
}

pub async fn run(
    client: Client,
    sender: Address,
    network: Network,
    key: Option<String>,
    feeds_path: &Path,
    opts: Options,
) -> Result<()> {
    let contracts = Contracts::for_network(network);
    let cfgs = feeds::load(feeds_path)?;
    let hermes = HermesClient::new()?;
    let ids: Vec<_> = cfgs.iter().map(|c| c.id).collect();
    let key = signer::require(key.as_deref())?;
    let (signing_key, _) = signer::load(key)?;

    let gas_cfg = GasPoolConfig {
        n: cfgs.len().div_ceil(opts.max_feeds_per_tx).max(1),
        target_niota: opts.gas_coin_target_niota,
        min_niota: opts.gas_coin_min_niota,
    };

    info!(
        sender = %sender,
        network = network.as_str(),
        pyth_state = %contracts.pyth_state,
        wormhole_state = %contracts.wormhole_state,
        feeds = cfgs.len(),
        tick_ms = TICK.as_millis() as u64,
        max_feeds_per_tx = opts.max_feeds_per_tx,
        hot_coins = gas_cfg.n,
        "relayer starting",
    );

    relayer_coins::warm_up(&client, &signing_key, sender, gas_cfg).await?;

    // Pick the largest pool coin to pin as the gas coin for every
    // startup dry-run. These are `inspect()` calls (read-only); they
    // never consume the coin, so reusing the same one across feeds is
    // safe. Avoids `with_auto_gas` accidentally picking a dust coin
    // from an unsorted indexer response.
    let startup_pool = relayer_coins::fetch_pool(&client, sender).await?;
    let dry_run_gas = startup_pool
        .largest()
        .ok_or_else(|| anyhow!("sender has no IOTA coins after warm-up"))?
        .object_ref;
    info!(coin = %dry_run_gas.object_id(), "pinned dry-run gas coin");

    let price_info_ids =
        resolve_price_info_ids(&client, sender, &contracts, &cfgs, dry_run_gas).await?;
    let reader = OnChainReader::new(client.clone(), &contracts, price_info_ids.clone());
    let updater =
        PythUpdater::new(&client, sender, contracts, price_info_ids, dry_run_gas).await?;

    let mut ticker = interval(TICK);
    ticker.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Skip);
    let mut sigint = signal(SignalKind::interrupt())?;
    let mut sigterm = signal(SignalKind::terminate())?;

    loop {
        tokio::select! {
            biased;
            _ = sigint.recv() => {
                info!(signal = "SIGINT", "shutdown signal received, exiting");
                return Ok(());
            }
            _ = sigterm.recv() => {
                info!(signal = "SIGTERM", "shutdown signal received, exiting");
                return Ok(());
            }
            _ = ticker.tick() => {}
        }
        let result = tokio::select! {
            biased;
            _ = sigint.recv() => {
                info!(signal = "SIGINT", "shutdown signal received, exiting");
                return Ok(());
            }
            _ = sigterm.recv() => {
                info!(signal = "SIGTERM", "shutdown signal received, exiting");
                return Ok(());
            }
            r = tick(&hermes, &ids, &cfgs, &client, &signing_key, sender, &reader, &updater, opts, gas_cfg) => r,
        };
        if let Err(e) = result {
            warn!(error = %e, "tick failed");
        }
    }
}

#[allow(clippy::too_many_arguments)]
async fn tick(
    hermes: &HermesClient,
    ids: &[FeedId],
    cfgs: &[feeds::FeedConfig],
    client: &Client,
    signer: &Ed25519PrivateKey,
    sender: Address,
    reader: &OnChainReader,
    updater: &PythUpdater,
    opts: Options,
    gas_cfg: GasPoolConfig,
) -> Result<()> {
    let (snapshot, on_chain) = tokio::try_join!(hermes.latest(ids), reader.snapshot())?;
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    let fires = trigger::fire(now, cfgs, &on_chain, &snapshot.parsed);
    debug!(
        feeds = cfgs.len(),
        hermes = snapshot.parsed.len(),
        on_chain = on_chain.len(),
        fires = fires.len(),
        "evaluated snapshot",
    );
    if fires.is_empty() {
        return Ok(());
    }

    let pool = relayer_coins::fetch_pool(client, sender).await?;
    let gas_coins = pool.top_n_refs(gas_cfg.n);

    let chunks: Vec<&[Fire<'_>]> = fires.chunks(opts.max_feeds_per_tx).collect();
    if gas_coins.len() < chunks.len() {
        return Err(anyhow!(
            "have {} hot coin(s) but need {} for parallel submission — rebalance will run after this tick",
            gas_coins.len(),
            chunks.len(),
        ));
    }

    let submissions = chunks.iter().zip(gas_coins.iter()).map(|(chunk, gas)| {
        submit_chunk(hermes, client, signer.clone(), sender, updater, chunk, *gas)
    });
    let results = futures::future::join_all(submissions).await;

    let mut any_err = None;
    for r in results {
        if let Err(e) = r {
            warn!(error = %e, "chunk submit failed");
            any_err.get_or_insert(e);
        }
    }

    relayer_coins::maybe_rebalance(client, signer, sender, gas_cfg).await?;

    match any_err {
        Some(e) => Err(e),
        None => Ok(()),
    }
}

async fn submit_chunk(
    hermes: &HermesClient,
    client: &Client,
    signer: Ed25519PrivateKey,
    sender: Address,
    updater: &PythUpdater,
    chunk: &[Fire<'_>],
    gas_coin: ObjectReference,
) -> Result<()> {
    let symbols: Vec<&str> = chunk.iter().map(|f| f.cfg.alias.as_str()).collect();
    let scoped_ids: Vec<FeedId> = chunk.iter().map(|f| f.cfg.id).collect();
    for f in chunk {
        info!(
            alias = %f.cfg.alias,
            reason = reason_label(f.reason),
            stale_s = f.stale_s,
            deviation_pct = format!("{:.3}", f.deviation_pct),
            "publishing",
        );
    }
    let scoped = hermes.latest(&scoped_ids).await?;
    let digest = updater
        .submit(client, signer, sender, scoped.update_data, chunk, gas_coin)
        .await?;
    info!(symbols = ?symbols, digest = %digest, "pyth update confirmed");
    Ok(())
}

fn reason_label(r: Reason) -> &'static str {
    match r {
        Reason::Heartbeat => "heartbeat",
        Reason::Deviation => "deviation",
    }
}
