//! Pyth update PTB construction + submission.
//!
//! Flow per submit:
//!
//! 1. Extract the Wormhole VAA from the Hermes accumulator blob and
//!    pass it to `wormhole::vaa::parse_and_verify`.
//! 2. `pyth::create_authenticated_price_infos_using_accumulator(state,
//!    accumulator_bytes, verified_vaa, &Clock)` →
//!    `HotPotatoVector<PriceInfo>`.
//! 3. Split N fee coins of `base_update_fee` nIOTA off the gas coin
//!    (one per fire).
//! 4. For each fire: `pyth::update_single_price_feed(state, hot_potato,
//!    price_info_obj, fee_coin, &Clock)` — threads the hot potato.
//! 5. `hot_potato_vector::destroy<PriceInfo>` to consume the potato.
//!
//! Price-info-object id resolution lives in [`crate::on_chain`]. This
//! module only takes the id map as a constructor argument and probes
//! the per-update fee at startup.

use anyhow::{anyhow, Context, Result};
use contracts_rs::{hot_potato_vector, price_info::PriceInfo, pyth};
use iota_sdk_crypto::ed25519::Ed25519PrivateKey;
use iota_sdk_graphql_client::Client;
use iota_sdk_transaction_builder::unresolved::{Command, SplitCoins};
use iota_sdk_types::execution_status::ExecutionStatus;
use iota_sdk_types::{Address, Digest, ObjectId, ObjectReference};
use move_bindgen_runtime::{Argument, ClientExt, PtbBuilder, WaitOptions};
use pyth_hermes_client::extract_vaa;
use tracing::info;

use crate::network::Contracts;
use crate::on_chain::PriceInfoIds;
use crate::trigger::Fire;

pub struct PythUpdater {
    contracts: Contracts,
    price_info_ids: PriceInfoIds,
    base_update_fee: u64,
}

impl PythUpdater {
    pub async fn new(
        client: &Client,
        sender: Address,
        contracts: Contracts,
        price_info_ids: PriceInfoIds,
        dry_run_gas: ObjectReference,
    ) -> Result<Self> {
        let mut ptb = PtbBuilder::new(sender)
            .with_client(client.clone())
            .with_package::<contracts_rs::Package>(contracts.pyth_package)
            .with_auto_gas();
        ptb.gas([dry_run_gas]);
        let fee_arg = pyth::get_total_update_fee(&mut ptb, contracts.pyth_state, 1u64).await;
        let inspect = ptb.inspect().await.context("probe pyth update fee")?;
        let base_update_fee: u64 = inspect.decode(fee_arg).context("decode update fee")?;

        info!(
            feeds = price_info_ids.len(),
            base_update_fee_niota = base_update_fee,
            "pyth updater initialised",
        );

        Ok(Self {
            contracts,
            price_info_ids,
            base_update_fee,
        })
    }

    pub async fn submit(
        &self,
        client: &Client,
        signer: Ed25519PrivateKey,
        sender: Address,
        update_data: Vec<Vec<u8>>,
        fires: &[Fire<'_>],
        gas_coin: ObjectReference,
    ) -> Result<Digest> {
        if fires.is_empty() {
            return Err(anyhow!("submit called with empty fires"));
        }
        let count = fires.len();
        let clock = ObjectId::CLOCK;

        let mut ptb = PtbBuilder::new(sender)
            .with_client(client.clone())
            .with_signer(signer)
            .with_package::<contracts_rs::Package>(self.contracts.pyth_package)
            .with_package::<wormhole_rs::Package>(self.contracts.wormhole_package)
            .with_auto_gas();
        ptb.gas([gas_coin]);

        let blob = update_data
            .into_iter()
            .next()
            .ok_or_else(|| anyhow!("hermes returned no update blobs"))?;
        let vaa_bytes = extract_vaa(&blob).context("extract VAA from accumulator")?;
        let verified_vaa = wormhole_rs::vaa::parse_and_verify(
            &mut ptb,
            self.contracts.wormhole_state,
            vaa_bytes,
            clock,
        )
        .await;

        let mut hot_potato = pyth::create_authenticated_price_infos_using_accumulator(
            &mut ptb,
            self.contracts.pyth_state,
            blob,
            verified_vaa,
            clock,
        )
        .await;

        let fee_amounts: Vec<Argument> = (0..count)
            .map(|_| ptb.inner.pure(self.base_update_fee))
            .collect();
        let split_result = ptb.inner.command(Command::SplitCoins(SplitCoins {
            coin: Argument::Gas,
            amounts: fee_amounts,
        }));
        let split_idx = match split_result {
            Argument::Result(i) => i,
            other => return Err(anyhow!("split_coins returned {other:?}")),
        };
        let fee_coins: Vec<Argument> = (0..count as u16)
            .map(|i| Argument::NestedResult(split_idx, i))
            .collect();

        for (i, f) in fires.iter().enumerate() {
            let pio = self
                .price_info_ids
                .get(&f.cfg.id)
                .copied()
                .ok_or_else(|| anyhow!("no price info object id for feed {}", f.cfg.alias))?;
            hot_potato = pyth::update_single_price_feed(
                &mut ptb,
                self.contracts.pyth_state,
                hot_potato,
                pio,
                fee_coins[i],
                clock,
            )
            .await;
        }

        hot_potato_vector::destroy::<PriceInfo>(&mut ptb, hot_potato).await;

        let (effects, _cache) = ptb
            .execute()
            .await
            .map_err(|e| anyhow!("pyth update submit: {e}"))?;
        let v1 = effects.as_v1();
        match v1.status() {
            ExecutionStatus::Success => {
                let digest = v1.transaction_digest;
                client
                    .wait_for_effects(&effects, WaitOptions::default())
                    .await
                    .context("wait for indexer to ingest pyth update")?;
                Ok(digest)
            }
            ExecutionStatus::Failure { error, command } => Err(anyhow!(
                "pyth update aborted (digest={}, command={:?}): {:?}",
                v1.transaction_digest,
                command,
                error,
            )),
            other => Err(anyhow!("pyth update unknown status: {other:?}")),
        }
    }
}
