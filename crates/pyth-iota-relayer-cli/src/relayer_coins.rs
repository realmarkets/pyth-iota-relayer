//! Hot-coin pool policy for parallel-tick submission.
//!
//! Each tick submits `ceil(fires / max_feeds_per_tx)` PTBs in
//! parallel; each PTB needs its own owned `Coin<IOTA>` as gas (two
//! parallel txs can't share one without version-conflicting). This
//! module maintains a small set of *hot* gas coins at a target value:
//!
//! - [`warm_up`] — at startup, if the pool has fewer than `n` coins
//!   at or above the target, rebalance once.
//! - [`maybe_rebalance`] — after a tick settles, if any of the top
//!   `n` coins fell below the min threshold, rebalance once.
//!
//! The actual merge+split PTB is built by
//! [`iota_coin_pool::CoinPool::build_rebalance_ptb`]; this module
//! handles the "when" (threshold checks) and the submit wrapper.

use anyhow::{anyhow, Context, Result};
use iota_coin_pool::CoinPool;

use crate::retry::with_retry;
use iota_sdk_crypto::ed25519::Ed25519PrivateKey;
use iota_sdk_graphql_client::Client;
use iota_sdk_types::execution_status::ExecutionStatus;
use iota_sdk_types::Address;
use move_bindgen_runtime::{ClientExt, WaitOptions};
use tracing::{info, warn};

#[derive(Debug, Clone, Copy)]
pub struct GasPoolConfig {
    /// Number of hot coins to maintain — sized to fit
    /// `ceil(feeds / max_feeds_per_tx)` parallel chunks.
    pub n: usize,
    /// Target value per hot coin, in nIOTA.
    pub target_niota: u64,
    /// Refill threshold — if any of the top `n` coins drops below
    /// this, the tick triggers a rebalance.
    pub min_niota: u64,
}

pub async fn fetch_pool(client: &Client, sender: Address) -> Result<CoinPool> {
    with_retry("fetch coin pool", || async {
        CoinPool::fetch_all(client, sender)
            .await
            .context("fetch coin pool")
    })
    .await
}

pub async fn warm_up(
    client: &Client,
    signer: &Ed25519PrivateKey,
    sender: Address,
    cfg: GasPoolConfig,
) -> Result<()> {
    let pool = fetch_pool(client, sender).await?;
    let usable = pool
        .iter()
        .filter(|c| c.balance >= cfg.target_niota)
        .count();
    if usable >= cfg.n {
        info!(have = usable, want = cfg.n, "gas-coin pool already warm");
        return Ok(());
    }
    info!(
        have = usable,
        want = cfg.n,
        target_niota = cfg.target_niota,
        "warming up gas-coin pool",
    );
    rebalance(client, signer.clone(), sender, &pool, cfg).await
}

pub async fn maybe_rebalance(
    client: &Client,
    signer: &Ed25519PrivateKey,
    sender: Address,
    cfg: GasPoolConfig,
) -> Result<()> {
    let pool = fetch_pool(client, sender).await?;
    let needs = pool.iter().take(cfg.n).any(|c| c.balance < cfg.min_niota);
    if !needs {
        return Ok(());
    }
    info!(min_niota = cfg.min_niota, "rebalancing gas-coin pool");
    rebalance(client, signer.clone(), sender, &pool, cfg).await
}

async fn rebalance(
    client: &Client,
    signer: Ed25519PrivateKey,
    sender: Address,
    pool: &CoinPool,
    cfg: GasPoolConfig,
) -> Result<()> {
    let (ptb, split_count) = pool
        .build_rebalance_ptb(sender, cfg.target_niota, cfg.n)
        .context("build rebalance PTB")?;
    if split_count < cfg.n {
        warn!(
            split = split_count,
            want = cfg.n,
            "balance below target — splitting fewer hot coins than configured",
        );
    }
    let ptb = ptb
        .with_client(client.clone())
        .with_signer(signer)
        .with_auto_gas();
    let (effects, _) = ptb
        .execute()
        .await
        .map_err(|e| anyhow!("rebalance submit: {e}"))?;
    let v1 = effects.as_v1();
    match v1.status() {
        ExecutionStatus::Success => {
            let digest = v1.transaction_digest;
            client
                .wait_for_effects(&effects, WaitOptions::default())
                .await
                .context("wait for indexer after rebalance")?;
            info!(digest = %digest, hot_coins = split_count, "gas-coin pool rebalanced");
            Ok(())
        }
        ExecutionStatus::Failure { error, command } => Err(anyhow!(
            "rebalance aborted (digest={}, command={:?}): {:?}",
            v1.transaction_digest,
            command,
            error,
        )),
        other => Err(anyhow!("rebalance unknown status: {other:?}")),
    }
}
