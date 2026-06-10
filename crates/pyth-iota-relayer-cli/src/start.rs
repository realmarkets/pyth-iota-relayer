//! `start` — the relay daemon. Runs until SIGINT or SIGTERM.
//!
//! Loop body: fetch on-chain `PriceInfoObject`s + Hermes parsed
//! prices, run the (stateless) hybrid trigger, then for each firing
//! chunk submit a Pyth update PTB *in parallel*. Each chunk gets its
//! own hot gas coin so the txs don't version-collide. Pool warm-up
//! happens once at startup; rebalance triggers after a tick if any
//! hot coin fell below the configured minimum.

use std::collections::{HashMap, HashSet};
use std::path::Path;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use anyhow::{anyhow, Result};
use iota_sdk_crypto::ed25519::Ed25519PrivateKey;
use iota_sdk_graphql_client::Client;
use iota_sdk_types::ObjectId;
use iota_sdk_types::{Address, ObjectReference};
use pyth_hermes_client::{FeedId, HermesClient};
use tokio::signal::unix::{signal, SignalKind};
use tokio::time::interval;
use tracing::{debug, info, warn};

use crate::cli::Network;
use crate::feeds;
use crate::network::Contracts;
use crate::on_chain::{resolve_price_info_ids, OnChainReader, PriceInfoIds};
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

#[derive(Default)]
struct FeedAvailability {
    active_ids: PriceInfoIds,
    unavailable: HashSet<FeedId>,
    retry_attempts: HashMap<FeedId, u64>,
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

    let mut availability = FeedAvailability::default();
    refresh_missing_feeds(&client, sender, &contracts, &cfgs, &mut availability).await?;
    let updater = PythUpdater::new(&client, sender, contracts).await?;

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
            r = tick(
                &hermes,
                &cfgs,
                &client,
                &signing_key,
                sender,
                &contracts,
                &updater,
                opts,
                &mut availability,
            ) => r,
        };
        if let Err(e) = result {
            warn!(error = %e, "tick failed");
        }
    }
}

#[allow(clippy::too_many_arguments)]
async fn tick(
    hermes: &HermesClient,
    cfgs: &[feeds::FeedConfig],
    client: &Client,
    signer: &Ed25519PrivateKey,
    sender: Address,
    contracts: &Contracts,
    updater: &PythUpdater,
    opts: Options,
    availability: &mut FeedAvailability,
) -> Result<()> {
    refresh_missing_feeds(client, sender, contracts, cfgs, availability).await?;

    let active_cfgs = active_cfgs(cfgs, &availability.active_ids);
    if active_cfgs.is_empty() {
        return Ok(());
    }
    let ids: Vec<_> = active_cfgs.iter().map(|c| c.id).collect();
    let reader = OnChainReader::new(client.clone(), contracts, availability.active_ids.clone());
    let (snapshot, chain_snapshot) = tokio::try_join!(hermes.latest(&ids), reader.snapshot())?;
    for feed_id in chain_snapshot.missing {
        mark_unavailable(
            cfgs,
            availability,
            feed_id,
            "price disappeared on-chain; deactivating feed",
        );
    }
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    let fires = trigger::fire(now, &active_cfgs, &chain_snapshot.prices, &snapshot.parsed);
    debug!(
        feeds = active_cfgs.len(),
        hermes = snapshot.parsed.len(),
        on_chain = chain_snapshot.prices.len(),
        fires = fires.len(),
        "evaluated snapshot",
    );
    if fires.is_empty() {
        return Ok(());
    }

    let gas_cfg = GasPoolConfig {
        n: active_cfgs.len().div_ceil(opts.max_feeds_per_tx).max(1),
        target_niota: opts.gas_coin_target_niota,
        min_niota: opts.gas_coin_min_niota,
    };
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
        submit_chunk(
            hermes,
            client,
            signer.clone(),
            sender,
            updater,
            &availability.active_ids,
            chunk,
            *gas,
        )
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
    price_info_ids: &PriceInfoIds,
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
        .submit(
            client,
            signer,
            sender,
            price_info_ids,
            scoped.update_data,
            chunk,
            gas_coin,
        )
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

async fn refresh_missing_feeds(
    client: &Client,
    sender: Address,
    contracts: &Contracts,
    cfgs: &[feeds::FeedConfig],
    availability: &mut FeedAvailability,
) -> Result<()> {
    let missing_cfgs: Vec<_> = cfgs
        .iter()
        .filter(|cfg| !availability.active_ids.contains_key(&cfg.id))
        .cloned()
        .collect();
    if missing_cfgs.is_empty() {
        return Ok(());
    }

    for cfg in &missing_cfgs {
        if availability.unavailable.contains(&cfg.id) {
            let attempt = availability.retry_attempts.entry(cfg.id).or_default();
            *attempt += 1;
            warn!(
                alias = %cfg.alias,
                attempt = *attempt,
                "re-checking unavailable feed"
            );
        }
    }

    let resolved = resolve_price_info_ids(client, sender, contracts, &missing_cfgs).await?;
    for cfg in missing_cfgs {
        match resolved.get(&cfg.id).copied() {
            Some(id) => {
                let was_unavailable = availability.unavailable.remove(&cfg.id);
                availability.retry_attempts.remove(&cfg.id);
                let was_active = availability.active_ids.insert(cfg.id, id);
                if was_unavailable {
                    info!(alias = %cfg.alias, "price became available on-chain; reactivating feed");
                } else if was_active.is_none() {
                    info!(alias = %cfg.alias, "price available on-chain; activating feed");
                }
            }
            None => {
                if availability.unavailable.insert(cfg.id) {
                    availability.retry_attempts.insert(cfg.id, 0);
                    warn!(
                        alias = %cfg.alias,
                        feed_id = %hex::encode(cfg.id),
                        "price currently unavailable on-chain; skipping feed"
                    );
                }
            }
        }
    }
    Ok(())
}

fn active_cfgs(
    cfgs: &[feeds::FeedConfig],
    active_ids: &HashMap<FeedId, ObjectId>,
) -> Vec<feeds::FeedConfig> {
    cfgs.iter()
        .filter(|cfg| active_ids.contains_key(&cfg.id))
        .cloned()
        .collect()
}

fn mark_unavailable(
    cfgs: &[feeds::FeedConfig],
    availability: &mut FeedAvailability,
    feed_id: FeedId,
    message: &'static str,
) {
    availability.active_ids.remove(&feed_id);
    if availability.unavailable.insert(feed_id) {
        availability.retry_attempts.insert(feed_id, 0);
        if let Some(cfg) = cfgs.iter().find(|cfg| cfg.id == feed_id) {
            warn!(alias = %cfg.alias, feed_id = %hex::encode(feed_id), "{message}");
        } else {
            warn!(feed_id = %hex::encode(feed_id), "{message}");
        }
    }
}
