//! Chain-side reads for the Pyth relayer.
//!
//! Two pieces:
//!
//! - [`resolve_price_info_ids`] — one-shot at startup: dry-runs
//!   `state::get_price_info_object_id` per feed (in parallel) to map
//!   each feed id to its on-chain `PriceInfoObject` id.
//! - [`OnChainReader::snapshot`] — per tick: batch-fetches every
//!   `PriceInfoObject`, BCS-decodes it, and returns the current
//!   `(price, expo, publish_time)` per feed. Used by the trigger so
//!   "should I publish?" is a pure function of chain state — multiple
//!   relayer instances see the same view and converge.

use std::collections::HashMap;
use std::sync::Arc;

use anyhow::{Context, Result};
use contracts_rs::price_info::PriceInfoObject;
use contracts_rs::state as pyth_state;
use iota_sdk_graphql_client::Client;
use iota_sdk_types::{Address, ObjectId, ObjectReference};
use move_bindgen_runtime::{ClientExt, PackageRegistry, PtbBuilder};
use pyth_hermes_client::FeedId;
use tracing::debug;

use crate::feeds::FeedConfig;
use crate::network::Contracts;
use crate::retry::with_retry;

pub type PriceInfoIds = Arc<HashMap<FeedId, ObjectId>>;

/// `(price, expo, publish_time)` for one feed, as read from its
/// on-chain `PriceInfoObject` at the current indexer tip.
#[derive(Debug, Clone, Copy)]
pub struct OnChainPrice {
    pub price: i64,
    pub expo: i32,
    pub publish_time: u64,
}

pub async fn resolve_price_info_ids(
    client: &Client,
    sender: Address,
    contracts: &Contracts,
    feeds: &[FeedConfig],
    gas_coin: ObjectReference,
) -> Result<PriceInfoIds> {
    let resolved: Vec<(FeedId, ObjectId)> =
        futures::future::try_join_all(feeds.iter().map(|cfg| async {
            let label = format!("resolve price-info-id for {}", cfg.alias);
            let id = with_retry(&label, || {
                resolve_one(client, sender, contracts, cfg, gas_coin)
            })
            .await?;
            anyhow::Ok((cfg.id, id))
        }))
        .await?;
    Ok(Arc::new(resolved.into_iter().collect()))
}

async fn resolve_one(
    client: &Client,
    sender: Address,
    contracts: &Contracts,
    cfg: &FeedConfig,
    gas_coin: ObjectReference,
) -> Result<ObjectId> {
    let mut ptb = PtbBuilder::new(sender)
        .with_client(client.clone())
        .with_package::<contracts_rs::Package>(contracts.pyth_package)
        .with_auto_gas();
    ptb.gas([gas_coin]);
    let arg =
        pyth_state::get_price_info_object_id(&mut ptb, contracts.pyth_state, cfg.id.to_vec()).await;
    let result = ptb
        .inspect()
        .await
        .with_context(|| format!("resolve price info id for {}", cfg.alias))?;
    let id: ObjectId = result
        .decode(arg)
        .with_context(|| format!("decode price info id for {}", cfg.alias))?;
    Ok(id)
}

pub struct OnChainReader {
    client: Client,
    addrs: PackageRegistry,
    ids: PriceInfoIds,
    feed_by_obj: HashMap<ObjectId, FeedId>,
}

impl OnChainReader {
    pub fn new(client: Client, contracts: &Contracts, ids: PriceInfoIds) -> Self {
        let addrs = PackageRegistry::at::<contracts_rs::Package>(contracts.pyth_package);
        let feed_by_obj = ids.iter().map(|(f, o)| (*o, *f)).collect();
        Self {
            client,
            addrs,
            ids,
            feed_by_obj,
        }
    }

    pub async fn snapshot(&self) -> Result<HashMap<FeedId, OnChainPrice>> {
        let object_ids: Vec<ObjectId> = self.ids.values().copied().collect();
        let objs: Vec<PriceInfoObject> = self
            .client
            .get_objects(&object_ids, &self.addrs)
            .await
            .context("fetch PriceInfoObjects")?;
        let mut out = HashMap::with_capacity(objs.len());
        for obj in objs {
            let oid: ObjectId = obj.id.id.bytes.into();
            let Some(&feed_id) = self.feed_by_obj.get(&oid) else {
                debug!(object = %oid, "ignoring PriceInfoObject not in our map");
                continue;
            };
            let p = obj.price_info.price_feed.price;
            let price = signed_i64(p.price.negative, p.price.magnitude);
            let expo = signed_i64(p.expo.negative, p.expo.magnitude) as i32;
            out.insert(
                feed_id,
                OnChainPrice {
                    price,
                    expo,
                    publish_time: p.timestamp,
                },
            );
        }
        Ok(out)
    }
}

fn signed_i64(negative: bool, magnitude: u64) -> i64 {
    if negative {
        -(magnitude as i64)
    } else {
        magnitude as i64
    }
}
