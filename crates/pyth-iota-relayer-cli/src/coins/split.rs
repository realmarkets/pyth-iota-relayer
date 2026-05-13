//! `coins split` — slice a coin into N fixed-size outputs.

use anyhow::Result;
use iota_coin_pool::{CoinPool, Source};
use iota_sdk_graphql_client::Client;
use iota_sdk_types::{Address, ObjectId};
use owo_colors::OwoColorize;

use super::common::pick_source;
use crate::{fmt, signer, submit};

pub async fn run(
    pool: &CoinPool,
    sender: Address,
    client: Client,
    key: Option<String>,
    amount: u64,
    count: usize,
    source: Option<ObjectId>,
) -> Result<()> {
    let (signing_key, _) = signer::load(signer::require(key.as_deref())?)?;
    let src = pick_source(pool, source)?;

    let total = amount.saturating_mul(count as u64);
    let from = match src {
        Source::Gas => "gas coin".to_string(),
        Source::Coin(c) => fmt::short_id(c.id),
    };
    fmt::print_plan(&format!(
        "split {} × {} = {} off {}",
        count.to_string().bold(),
        fmt::iota(amount).bold(),
        fmt::iota(total).bold(),
        from,
    ));

    let ptb = CoinPool::build_split_ptb(sender, src, amount, count);
    submit::execute(client, signing_key, ptb, "split").await
}
