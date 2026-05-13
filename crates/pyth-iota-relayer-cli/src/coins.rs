//! `coins` subcommand dispatch. The per-subcommand handlers live in
//! sibling files under `coins/`; this module just wires them up + owns
//! the shared `fetch_pool` step.

mod common;
mod list;
mod merge;
mod show;
mod split;
mod transfer;

use anyhow::{Context, Result};
use iota_coin_pool::CoinPool;
use iota_sdk_graphql_client::Client;
use iota_sdk_types::Address;

use crate::cli::{CoinsCmd, Network};

pub async fn run(
    cmd: CoinsCmd,
    client: Client,
    sender: Address,
    network: Network,
    key: Option<String>,
) -> Result<()> {
    let pool = CoinPool::fetch_all(&client, sender)
        .await
        .with_context(|| format!("fetching gas-coin pool for {sender}"))?;

    match cmd {
        CoinsCmd::Show { dust_below } => show::run(&pool, sender, network, dust_below),
        CoinsCmd::List => list::run(&pool),
        CoinsCmd::Split {
            amount,
            count,
            source,
        } => split::run(&pool, sender, client, key, amount, count, source).await,
        CoinsCmd::Merge { dust_below, into } => {
            merge::run(&pool, sender, client, key, dust_below, into).await
        }
        CoinsCmd::Transfer {
            to,
            amount,
            source,
        } => transfer::run(&pool, sender, client, key, to, amount, source).await,
    }
}
