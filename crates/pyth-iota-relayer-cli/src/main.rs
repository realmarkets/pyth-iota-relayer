//! `pyth-iota-relayer` — top-level binary.
//!
//! Parses CLI args, sets up the GraphQL client + signing key for the
//! requested network, and dispatches to `start` or `coins {…}`.

mod cli;
mod coins;
mod feeds;
mod fmt;
mod network;
mod on_chain;
mod pyth_update;
mod relayer_coins;
mod retry;
mod signer;
mod start;
mod submit;
mod trigger;

use crate::cli::{Cli, Cmd, Network};
use anyhow::Result;
use clap::Parser;
use iota_sdk_graphql_client::Client;

#[tokio::main]
async fn main() -> Result<()> {
    init_tracing();

    let cli = Cli::parse();
    let client = build_client(cli.network, cli.rpc.as_deref())?;
    let sender = resolve_sender(cli.key.as_deref())?;

    match cli.cmd {
        Cmd::Start {
            max_feeds_per_tx,
            gas_coin_target,
            gas_coin_min,
        } => {
            let opts = start::Options {
                max_feeds_per_tx,
                gas_coin_target_niota: iota_from_f64(gas_coin_target),
                gas_coin_min_niota: iota_from_f64(gas_coin_min),
            };
            start::run(client, sender, cli.network, cli.key, &cli.feeds, opts).await
        }
        Cmd::Coins { cmd } => coins::run(cmd, client, sender, cli.network, cli.key).await,
    }
}

fn iota_from_f64(v: f64) -> u64 {
    (v * 1_000_000_000.0).round() as u64
}

fn init_tracing() {
    use tracing_subscriber::{fmt, EnvFilter};
    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("pyth_iota_relayer=info"));
    fmt().with_env_filter(filter).with_target(false).init();
}

fn build_client(network: Network, rpc_override: Option<&str>) -> Result<Client> {
    if let Some(url) = rpc_override {
        return Client::new(url).map_err(|e| anyhow::anyhow!("invalid --rpc URL {url}: {e}"));
    }
    Ok(match network {
        Network::Testnet => Client::new_testnet(),
        Network::Mainnet => Client::new_mainnet(),
    })
}

/// Resolve the relayer's address from the configured key. For
/// read-only subcommands the key is still required — the binary has no
/// other way to know whose pool to inspect. If the user wants to peek
/// at someone else's pool, they can run with a throwaway key matching
/// that address, or we add an `--address` flag later.
fn resolve_sender(key: Option<&str>) -> Result<iota_sdk_types::Address> {
    let key = signer::require(key)?;
    let (_, sender) = signer::load(key)?;
    Ok(sender)
}
