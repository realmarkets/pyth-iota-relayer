//! Pyth Hermes REST client.
//!
//! Hermes is Pyth's public price-update endpoint. The `latest` call
//! returns two things in one response:
//!
//! - `update_data`: one or more accumulator blobs (base64-decoded into
//!   raw bytes) to be passed to the on-chain
//!   `pyth::pyth::create_price_infos_hot_potato` Move entry.
//! - `parsed`: the same prices in a human-/state-machine-readable form
//!   (`price`, `conf`, `expo`, `publish_time`) so callers can run
//!   deviation/heartbeat logic without parsing the binary blob.
//!
//! The numeric fields come over the wire as JSON strings; this crate
//! deserialises them into native integers.

use std::time::Duration;

use anyhow::{anyhow, Context, Result};
use base64::Engine;
use serde::{Deserialize, Deserializer};

pub mod accumulator;
pub use accumulator::extract_vaa;

/// 32-byte Pyth price-feed identifier.
pub type FeedId = [u8; 32];

pub const DEFAULT_BASE_URL: &str = "https://hermes.pyth.network";

#[derive(Debug, Clone)]
pub struct PriceUpdate {
    /// Accumulator blobs to be forwarded to the on-chain Pyth contract.
    /// Hermes typically returns a single bundled blob covering all
    /// requested feeds.
    pub update_data: Vec<Vec<u8>>,
    /// Parsed price per feed, in the same order the upstream returns.
    pub parsed: Vec<ParsedPrice>,
}

#[derive(Debug, Clone)]
pub struct ParsedPrice {
    pub id: FeedId,
    pub price: i64,
    pub conf: u64,
    pub expo: i32,
    pub publish_time: u64,
}

pub struct HermesClient {
    base: String,
    http: reqwest::Client,
}

impl HermesClient {
    pub fn new() -> Result<Self> {
        Self::with_base_url(DEFAULT_BASE_URL.to_string())
    }

    pub fn with_base_url(base: String) -> Result<Self> {
        let http = reqwest::Client::builder()
            .timeout(Duration::from_secs(15))
            .build()
            .context("build reqwest client")?;
        let base = base.trim_end_matches('/').to_string();
        Ok(Self { base, http })
    }

    /// `GET /v2/updates/price/latest?ids[]=<hex>&encoding=base64`.
    pub async fn latest(&self, ids: &[FeedId]) -> Result<PriceUpdate> {
        if ids.is_empty() {
            return Err(anyhow!("at least one feed id required"));
        }
        let url = format!("{}/v2/updates/price/latest", self.base);
        let mut query: Vec<(&str, String)> =
            ids.iter().map(|id| ("ids[]", hex::encode(id))).collect();
        query.push(("encoding", "base64".to_string()));

        let resp = self
            .http
            .get(&url)
            .query(&query)
            .send()
            .await
            .with_context(|| format!("GET {url}"))?;
        let status = resp.status();
        let body = resp.text().await.context("read response body")?;
        if !status.is_success() {
            return Err(anyhow!("hermes {status}: {body}"));
        }
        let parsed: LatestResponse =
            serde_json::from_str(&body).with_context(|| format!("decode hermes response: {body}"))?;

        let engine = base64::engine::general_purpose::STANDARD;
        let update_data = parsed
            .binary
            .data
            .iter()
            .map(|s| engine.decode(s).context("decode hermes binary.data base64"))
            .collect::<Result<Vec<_>>>()?;

        let parsed_prices = parsed
            .parsed
            .into_iter()
            .map(|p| {
                let id_hex = p.id.strip_prefix("0x").unwrap_or(&p.id);
                let bytes = hex::decode(id_hex).context("decode hermes parsed.id hex")?;
                let id: FeedId = bytes
                    .try_into()
                    .map_err(|_| anyhow!("hermes parsed.id is not 32 bytes"))?;
                Ok(ParsedPrice {
                    id,
                    price: p.price.price,
                    conf: p.price.conf,
                    expo: p.price.expo,
                    publish_time: p.price.publish_time,
                })
            })
            .collect::<Result<Vec<_>>>()?;

        Ok(PriceUpdate {
            update_data,
            parsed: parsed_prices,
        })
    }
}

#[derive(Deserialize)]
struct LatestResponse {
    binary: BinaryBlock,
    parsed: Vec<ParsedItem>,
}

#[derive(Deserialize)]
struct BinaryBlock {
    #[allow(dead_code)]
    encoding: String,
    data: Vec<String>,
}

#[derive(Deserialize)]
struct ParsedItem {
    id: String,
    price: PriceFields,
}

#[derive(Deserialize)]
struct PriceFields {
    #[serde(deserialize_with = "de_i64_str")]
    price: i64,
    #[serde(deserialize_with = "de_u64_str")]
    conf: u64,
    expo: i32,
    publish_time: u64,
}

fn de_i64_str<'de, D: Deserializer<'de>>(d: D) -> Result<i64, D::Error> {
    use serde::de::Error;
    let s = String::deserialize(d)?;
    s.parse::<i64>().map_err(D::Error::custom)
}

fn de_u64_str<'de, D: Deserializer<'de>>(d: D) -> Result<u64, D::Error> {
    use serde::de::Error;
    let s = String::deserialize(d)?;
    s.parse::<u64>().map_err(D::Error::custom)
}
