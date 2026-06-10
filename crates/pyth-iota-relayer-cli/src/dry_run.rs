use std::cmp::Reverse;

use anyhow::{Context, Result, anyhow};
use iota_sdk_graphql_client::{Client, PaginationFilter};
use iota_sdk_types::Address;
use move_bindgen_runtime::PtbBuilder;

const INSPECT_GAS_BUDGET: u64 = 50_000_000;

/// Read-only startup `inspect()` calls don't need gas estimation.
/// Configure a concrete gas payment so we avoid the extra dry-run
/// behind `with_auto_gas()`.
pub async fn configure_inspect_gas(
    client: &Client,
    sender: Address,
    ptb: &mut PtbBuilder,
) -> Result<()> {
    let mut gas_candidates = client
        .gas_coins(sender, PaginationFilter::default())
        .await
        .with_context(|| format!("list gas coins for inspect sender {sender}"))?
        .data;
    gas_candidates.sort_by_key(|coin| Reverse(coin.balance()));

    let gas_coin = gas_candidates
        .iter()
        .find(|coin| coin.balance() >= INSPECT_GAS_BUDGET)
        .or_else(|| gas_candidates.first())
        .ok_or_else(|| anyhow!("no gas coins available for inspect sender {sender}"))?;
    let gas_object = client
        .object(*gas_coin.id(), None)
        .await
        .with_context(|| format!("fetch inspect gas coin object {}", gas_coin.id()))?
        .ok_or_else(|| anyhow!("inspect gas coin {} disappeared", gas_coin.id()))?;
    let gas_price = client
        .reference_gas_price(None)
        .await
        .context("fetch reference gas price for inspect")?
        .ok_or_else(|| anyhow!("reference gas price unavailable for inspect"))?;

    ptb.gas([gas_object.object_ref()]);
    ptb.gas_price(gas_price);
    ptb.gas_budget(INSPECT_GAS_BUDGET);
    Ok(())
}
