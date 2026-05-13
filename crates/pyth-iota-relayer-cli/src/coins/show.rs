//! `coins show` — boxed summary of the gas-coin pool.

use anyhow::Result;
use comfy_table::modifiers::UTF8_ROUND_CORNERS;
use comfy_table::presets::UTF8_FULL;
use comfy_table::{Attribute, Cell, Color, ContentArrangement, Table};
use iota_coin_pool::CoinPool;
use iota_sdk_types::Address;

use crate::cli::Network;
use crate::fmt;

pub fn run(pool: &CoinPool, sender: Address, network: Network, dust_below: u64) -> Result<()> {
    let total = pool.total_balance();
    let count = pool.count();
    let largest = pool.largest().map(|c| c.balance).unwrap_or(0);
    let smallest = pool.smallest().map(|c| c.balance).unwrap_or(0);
    let dust_count = pool.iter().filter(|c| c.balance <= dust_below).count();

    let mut t = Table::new();
    t.load_preset(UTF8_FULL)
        .apply_modifier(UTF8_ROUND_CORNERS)
        .set_content_arrangement(ContentArrangement::Dynamic)
        .set_header(vec![
            Cell::new(" pyth-iota-relayer · gas-coin pool ")
                .add_attribute(Attribute::Bold)
                .fg(Color::Cyan),
            Cell::new(""),
        ]);
    t.add_row(vec![Cell::new("owner"), Cell::new(sender.to_string())]);
    t.add_row(vec![
        Cell::new("network"),
        Cell::new(network.as_str()).fg(fmt::network_color(network)),
    ]);
    t.add_row(vec![
        Cell::new("total"),
        Cell::new(fmt::iota(total)).add_attribute(Attribute::Bold),
    ]);
    t.add_row(vec![Cell::new("coin count"), Cell::new(count.to_string())]);
    t.add_row(vec![Cell::new("largest"), Cell::new(fmt::iota(largest))]);
    t.add_row(vec![Cell::new("smallest"), Cell::new(fmt::iota(smallest))]);
    t.add_row(vec![
        Cell::new(format!("dust ≤ {}", fmt::iota(dust_below))),
        Cell::new(dust_count.to_string()).fg(if dust_count > 0 {
            Color::Yellow
        } else {
            Color::Reset
        }),
    ]);
    println!("{t}");
    Ok(())
}
