//! `coins merge` — collapse dust into a target coin.

use anyhow::Result;
use iota_coin_pool::{CoinEntry, CoinPool, Source};
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
    dust_below: u64,
    into: Option<ObjectId>,
) -> Result<()> {
    let (signing_key, _) = signer::load(signer::require(key.as_deref())?)?;
    let dest = pick_source(pool, into)?;

    // Coins we'll consume into the destination. Excludes the dest itself
    // (when it's a specific coin); `Source::Gas` is already not in the
    // pool's enumeration, so no extra exclusion needed for that case.
    let dest_id = match dest {
        Source::Coin(c) => Some(c.id),
        Source::Gas => None,
    };
    let to_merge: Vec<CoinEntry> = pool
        .iter()
        .filter(|c| c.balance <= dust_below && Some(c.id) != dest_id)
        .cloned()
        .collect();

    if to_merge.is_empty() {
        println!(
            "{} no coin has balance ≤ {} — nothing to merge.",
            "·".dimmed(),
            fmt::iota(dust_below),
        );
        return Ok(());
    }

    let recovered: u64 = to_merge.iter().map(|c| c.balance).sum();
    let target = match dest {
        Source::Gas => "gas coin".to_string(),
        Source::Coin(c) => fmt::short_id(c.id),
    };
    fmt::print_plan(&format!(
        "merge {} coin(s) into {} (+{} recovered)",
        to_merge.len().to_string().bold(),
        target,
        fmt::iota(recovered).bold(),
    ));

    let ptb = CoinPool::build_merge_ptb(sender, dest, &to_merge);
    submit::execute(client, signing_key, ptb, "merge").await
}
