//! Pretty-print helpers shared between subcommand handlers.
//!
//! Just formatters — never prints. Callers decide *where* output
//! lands.
//!
//! Unit convention used everywhere in this binary:
//!
//! ```text
//! nanoIOTA  (nIOTA)  0.000_000_001 IOTA              ← chain-side base unit
//! microIOTA (μIOTA)  0.000_001     IOTA  =        1_000 nIOTA
//! milliIOTA (mIOTA)  0.001         IOTA  =    1_000_000 nIOTA
//! IOTA                                  =  1_000_000_000 nIOTA
//! ```
//!
//! All on-chain amounts and CLI flag values are `u64` of **nIOTA**.

use iota_sdk_types::ObjectId;
use owo_colors::OwoColorize;

use crate::cli::Network;

/// 1 IOTA = 1_000_000_000 nIOTA. The chain's base unit.
pub const NIOTA_PER_IOTA: u64 = 1_000_000_000;

/// `5_000_000_000` → `"5 IOTA"`,
/// `1_234_567_890` → `"1.23456789 IOTA"`,
/// `100`           → `"0.0000001 IOTA"`,
/// `0`             → `"0 IOTA"`.
///
/// Strips trailing zeros and the decimal point when the value is a
/// whole number. Integer math only — no `f64` rounding.
pub fn iota(niota: u64) -> String {
    let whole = niota / NIOTA_PER_IOTA;
    let frac = niota % NIOTA_PER_IOTA;
    if frac == 0 {
        return format!("{whole} IOTA");
    }
    let mut frac_str = format!("{frac:09}");
    while frac_str.ends_with('0') {
        frac_str.pop();
    }
    format!("{whole}.{frac_str} IOTA")
}

/// Variant of [`iota`] padded to a fixed number of decimals — useful
/// for columnar tables where every value should occupy the same
/// width.
pub fn iota_fixed(niota: u64, decimals: u32) -> String {
    let whole = niota / NIOTA_PER_IOTA;
    let frac = niota % NIOTA_PER_IOTA;
    let decimals = decimals.min(9) as usize;
    if decimals == 0 {
        // Round half-up to whole IOTA.
        let rounded = whole + (frac >= NIOTA_PER_IOTA / 2) as u64;
        return format!("{rounded} IOTA");
    }
    let frac_padded = format!("{frac:09}");
    format!("{whole}.{} IOTA", &frac_padded[..decimals])
}

/// `0xb96280…0b7ed` — 6 leading hex chars + ellipsis + 4 trailing.
/// Useful in log lines where a full 64-char id wastes space.
pub fn short_id(id: ObjectId) -> String {
    let s = id.to_string();
    if s.len() > 14 {
        format!("{}…{}", &s[..6], &s[s.len() - 4..])
    } else {
        s
    }
}

/// Colour used to flag the active network in summary blocks.
pub fn network_color(network: Network) -> comfy_table::Color {
    match network {
        Network::Testnet => comfy_table::Color::Yellow,
        Network::Mainnet => comfy_table::Color::Green,
    }
}

/// `plan: <line>` with the prefix in cyan. Subcommands print this
/// before kicking off a tx so the user sees what's about to happen.
pub fn print_plan(line: &str) {
    println!("{} {}", "plan:".cyan().bold(), line);
}

/// `ok <op> · tx digest <digest>` with green `ok`.
pub fn print_ok(op: &str, digest: &str) {
    println!(
        "{} {} · tx digest {}",
        "ok".green().bold(),
        op,
        digest.bold(),
    );
}

/// Cosmetic dust threshold for the `coins list` colour cue — pick the
/// median balance / 100 so genuine outliers get dimmed without
/// requiring a flag.
pub fn dust_threshold_hint(balances: &[u64]) -> u64 {
    if balances.is_empty() {
        return 1;
    }
    let mut sorted: Vec<u64> = balances.to_vec();
    sorted.sort_unstable();
    let median = sorted[sorted.len() / 2];
    (median / 100).max(1)
}
