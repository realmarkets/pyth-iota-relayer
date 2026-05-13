//! Helpers shared between `coins/{split,merge,transfer}.rs`.
//!
//! Mostly translates a user-supplied `--source <id>` (an `Option`)
//! into either [`iota_coin_pool::Source::Gas`] when omitted or
//! [`Source::Coin(entry)`] when present.

use anyhow::{Context, Result};
use iota_coin_pool::{CoinPool, Source};
use iota_sdk_types::ObjectId;

/// `None` → [`Source::Gas`] (use the PTB's gas coin — bootstrap mode,
/// works with as few as 1 coin in the pool).
/// `Some(id)` → [`Source::Coin`] pointing at the named entry. Errors
/// if the id isn't in the pool.
pub fn pick_source<'a>(pool: &'a CoinPool, source: Option<ObjectId>) -> Result<Source<'a>> {
    match source {
        None => Ok(Source::Gas),
        Some(id) => {
            let entry = pool
                .find(id)
                .with_context(|| format!("coin {id} not in the sender's pool"))?;
            Ok(Source::Coin(entry))
        }
    }
}
