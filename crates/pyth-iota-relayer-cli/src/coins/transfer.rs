//! `coins transfer` — split off `amount` and send to a recipient.

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
    to: Address,
    amount: u64,
    source: Option<ObjectId>,
) -> Result<()> {
    let (signing_key, _) = signer::load(signer::require(key.as_deref())?)?;
    let src = pick_source(pool, source)?;

    let from = match src {
        Source::Gas => "gas coin".to_string(),
        Source::Coin(c) => fmt::short_id(c.id),
    };
    fmt::print_plan(&format!(
        "transfer {} from {} to {}",
        fmt::iota(amount).bold(),
        from,
        to.to_string().bold(),
    ));

    let ptb = CoinPool::build_transfer_ptb(sender, src, to, amount);
    submit::execute(client, signing_key, ptb, "transfer").await
}
