//! Feed config loader. The YAML is a flat list:
//!
//! ```yaml
//! - alias: BTC/USD
//!   id: e62df6c8b4a85fe1a67db44dc12de5db330f7ac66b72dc658afedf0f4a415b43
//!   time_difference: 10        # seconds (heartbeat)
//!   price_deviation: 0.5       # percent
//!   confidence_ratio: 1        # percent — skip publish above this
//! ```

use std::path::Path;

use anyhow::{Context, Result};
use pyth_hermes_client::FeedId;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct FeedConfig {
    pub alias: String,
    #[serde(deserialize_with = "de_feed_id_hex")]
    pub id: FeedId,
    pub time_difference: u64,
    pub price_deviation: f64,
    pub confidence_ratio: f64,
}

pub fn load(path: &Path) -> Result<Vec<FeedConfig>> {
    let raw = std::fs::read_to_string(path)
        .with_context(|| format!("read feeds config {}", path.display()))?;
    let feeds: Vec<FeedConfig> = serde_yaml::from_str(&raw)
        .with_context(|| format!("parse feeds config {}", path.display()))?;
    if feeds.is_empty() {
        anyhow::bail!("feeds config {} is empty", path.display());
    }
    Ok(feeds)
}

fn de_feed_id_hex<'de, D>(d: D) -> Result<FeedId, D::Error>
where
    D: serde::Deserializer<'de>,
{
    use serde::de::Error;
    let s = String::deserialize(d)?;
    let s = s.strip_prefix("0x").unwrap_or(&s);
    let bytes = hex::decode(s).map_err(D::Error::custom)?;
    let arr: FeedId = bytes
        .try_into()
        .map_err(|_| D::Error::custom("feed id must be 32 bytes"))?;
    Ok(arr)
}
