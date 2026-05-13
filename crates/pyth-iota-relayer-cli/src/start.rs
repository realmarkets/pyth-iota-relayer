//! `start` — the relay daemon. Runs until SIGINT or SIGTERM.
//!
//! Loop body: fetch the on-chain `PriceInfoObject` snapshot + the
//! latest Hermes snapshot, run the (stateless) hybrid trigger, build a
//! Pyth update PTB for the firing subset, and submit.

use std::path::Path;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use anyhow::Result;
use iota_sdk_graphql_client::Client;
use iota_sdk_types::Address;
use pyth_hermes_client::HermesClient;
use tokio::signal::unix::{signal, SignalKind};
use tokio::time::interval;
use tracing::{debug, info, warn};

use crate::cli::Network;
use crate::feeds;
use crate::network::Contracts;
use crate::on_chain::{resolve_price_info_ids, OnChainReader};
use crate::pyth_update::PythUpdater;
use crate::signer;
use crate::trigger::{self, Reason};

const TICK: Duration = Duration::from_secs(1);

pub async fn run(
    client: Client,
    sender: Address,
    network: Network,
    key: Option<String>,
    feeds_path: &Path,
) -> Result<()> {
    let contracts = Contracts::for_network(network);
    let cfgs = feeds::load(feeds_path)?;
    let hermes = HermesClient::new()?;
    let ids: Vec<_> = cfgs.iter().map(|c| c.id).collect();
    let key = signer::require(key.as_deref())?;
    let (signing_key, _) = signer::load(key)?;

    info!(
        sender = %sender,
        network = network.as_str(),
        pyth_state = %contracts.pyth_state,
        wormhole_state = %contracts.wormhole_state,
        feeds = cfgs.len(),
        tick_ms = TICK.as_millis() as u64,
        "relayer starting",
    );

    let price_info_ids = resolve_price_info_ids(&client, sender, &contracts, &cfgs).await?;
    let reader = OnChainReader::new(client.clone(), &contracts, price_info_ids.clone());
    let updater = PythUpdater::new(&client, sender, contracts, price_info_ids).await?;

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
            r = tick(&hermes, &ids, &cfgs, &client, &signing_key, sender, &reader, &updater) => r,
        };
        if let Err(e) = result {
            warn!(error = %e, "tick failed");
        }
    }
}

#[allow(clippy::too_many_arguments)]
async fn tick(
    hermes: &HermesClient,
    ids: &[pyth_hermes_client::FeedId],
    cfgs: &[feeds::FeedConfig],
    client: &Client,
    signer: &iota_sdk_crypto::ed25519::Ed25519PrivateKey,
    sender: Address,
    reader: &OnChainReader,
    updater: &PythUpdater,
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
    for f in &fires {
        info!(
            alias = %f.cfg.alias,
            reason = reason_label(f.reason),
            on_chain_publish_time = f.on_chain.publish_time,
            hermes_publish_time = f.hermes.publish_time,
            "publishing",
        );
    }
    updater
        .submit(client, signer.clone(), sender, snapshot.update_data, &fires)
        .await
}

fn reason_label(r: Reason) -> &'static str {
    match r {
        Reason::Heartbeat => "heartbeat",
        Reason::Deviation => "deviation",
    }
}
