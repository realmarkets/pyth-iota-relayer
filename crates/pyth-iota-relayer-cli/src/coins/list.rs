//! `coins list` — one-row-per-coin table.

use anyhow::Result;
use comfy_table::modifiers::UTF8_ROUND_CORNERS;
use comfy_table::presets::UTF8_FULL;
use comfy_table::{Attribute, Cell, CellAlignment, Color, ContentArrangement, Table};
use iota_coin_pool::CoinPool;
use owo_colors::OwoColorize;

use crate::fmt;

pub fn run(pool: &CoinPool) -> Result<()> {
    let dust_threshold =
        fmt::dust_threshold_hint(&pool.coins.iter().map(|c| c.balance).collect::<Vec<_>>());

    let mut t = Table::new();
    t.load_preset(UTF8_FULL)
        .apply_modifier(UTF8_ROUND_CORNERS)
        .set_content_arrangement(ContentArrangement::Dynamic)
        .set_header(vec![
            Cell::new("object_id").add_attribute(Attribute::Bold),
            Cell::new("balance")
                .add_attribute(Attribute::Bold)
                .set_alignment(CellAlignment::Right),
            Cell::new("version")
                .add_attribute(Attribute::Bold)
                .set_alignment(CellAlignment::Right),
        ]);
    for c in &pool.coins {
        let dust = c.balance <= dust_threshold;
        let mut id_cell = Cell::new(c.id.to_string());
        let mut bal_cell = Cell::new(fmt::iota(c.balance)).set_alignment(CellAlignment::Right);
        let mut ver_cell = Cell::new(c.object_ref.version().to_string())
            .set_alignment(CellAlignment::Right);
        if dust {
            id_cell = id_cell.fg(Color::DarkGrey);
            bal_cell = bal_cell.fg(Color::DarkGrey);
            ver_cell = ver_cell.fg(Color::DarkGrey);
        }
        t.add_row(vec![id_cell, bal_cell, ver_cell]);
    }
    println!("{t}");
    if pool.coins.iter().any(|c| c.balance <= dust_threshold) {
        println!(
            "{} rows in dimmed grey are ≤ {} (cosmetic dust cue)",
            "·".dimmed(),
            fmt::iota(dust_threshold),
        );
    }
    Ok(())
}
