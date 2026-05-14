//! Owner-side gas-coin pool + PTB-fee helpers for IOTA bots and relayers.
//!
//! The pool model:
//! - The biggest coin (above [`Allocation::gas_min`]) is the **gas
//!   coin** for any PTB built by [`Allocation::apply`].
//! - The next-largest coins are **fee coins** — distinct objects bound
//!   as `Input::ImmutableOrOwned` so generated calls can pass them to
//!   `Coin<IOTA>` parameters by value (e.g. Pyth's per-update fee).
//!   Distinct objects matter: the chain locks the gas coin for billing
//!   and rejects txs that also consume it as a regular input, so the
//!   fee path must come from a different object.
//!
//! Maintenance helpers ([`CoinPool::build_split_ptb`],
//! [`build_merge_ptb`], [`build_transfer_ptb`]) build PTBs but never
//! sign — the caller submits.

use iota_sdk_graphql_client::{query_types::ObjectFilter, Client, PaginationFilter};
use iota_sdk_transaction_builder::unresolved::{
    Argument, Command, MergeCoins, SplitCoins, TransferObjects,
};
use iota_sdk_types::{
    framework::Coin, Address, Identifier, Input, ObjectId, ObjectReference, StructTag, TypeTag,
};
use move_bindgen_runtime::PtbBuilder;

/// 0x2 — the IOTA framework address.
const IOTA_FRAMEWORK: Address = {
    let mut bytes = [0u8; 32];
    bytes[31] = 2;
    Address::new(bytes)
};

/// A snapshot of every `Coin<IOTA>` owned by an address. Sorted by
/// balance descending.
#[derive(Debug, Clone, Default)]
pub struct CoinPool {
    pub coins: Vec<CoinEntry>,
}

#[derive(Debug, Clone)]
pub struct CoinEntry {
    pub id: ObjectId,
    pub balance: u64,
    pub object_ref: ObjectReference,
}

/// Result of pulling a gas coin + N fee coins out of the pool, ready to
/// be bound onto a [`PtbBuilder`] via [`Allocation::apply`].
#[derive(Debug, Clone)]
pub struct Allocation {
    pub gas: CoinEntry,
    pub fees: Vec<CoinEntry>,
}

#[derive(Debug, thiserror::Error)]
pub enum PoolError {
    #[error("client backend: {0}")]
    Backend(String),
    #[error("pool empty for owner {owner}")]
    Empty { owner: Address },
    #[error("no coin has balance >= {needed} (largest: {largest})")]
    NoCoinBigEnough { needed: u64, largest: u64 },
    #[error(
        "need {fee_count} fee coin(s) of >= {fee_min}, only {available} in the pool after \
         reserving the gas coin"
    )]
    InsufficientFeeCoins {
        fee_count: usize,
        fee_min: u64,
        available: usize,
    },
    #[error("source coin {0} not in pool")]
    SourceNotInPool(ObjectId),
    #[error("pool has no IOTA coins to rebalance")]
    NothingToRebalance,
    #[error("total balance {have} nIOTA is below one target coin of {need} nIOTA")]
    InsufficientBalance { have: u64, need: u64 },
}

impl CoinPool {
    /// Walk every `Coin<0x2::iota::IOTA>` owned by `owner`, paginated.
    /// Sorted balance-descending so the gas coin is always `coins[0]`.
    pub async fn fetch_all(client: &Client, owner: Address) -> Result<Self, PoolError> {
        let coin_type = format!(
            "{}::coin::Coin<{}::iota::IOTA>",
            IOTA_FRAMEWORK, IOTA_FRAMEWORK,
        );
        let filter = ObjectFilter {
            type_: Some(coin_type),
            owner: Some(owner),
            object_ids: None,
        };

        let mut coins = Vec::new();
        let mut cursor: Option<String> = None;
        loop {
            let page = client
                .objects(
                    filter.clone(),
                    PaginationFilter {
                        cursor: cursor.clone(),
                        ..Default::default()
                    },
                )
                .await
                .map_err(|e| PoolError::Backend(e.to_string()))?;
            for obj in &page.data {
                let coin = Coin::try_from_object(obj)
                    .map_err(|e| PoolError::Backend(format!("decode coin: {e}")))?;
                coins.push(CoinEntry {
                    id: *coin.id(),
                    balance: coin.balance(),
                    object_ref: obj.object_ref(),
                });
            }
            if !page.page_info.has_next_page {
                break;
            }
            cursor = page.page_info.end_cursor;
        }

        coins.sort_by_key(|c| std::cmp::Reverse(c.balance));
        Ok(Self { coins })
    }

    pub fn total_balance(&self) -> u64 {
        self.coins.iter().map(|c| c.balance).sum()
    }

    pub fn count(&self) -> usize {
        self.coins.len()
    }

    pub fn largest(&self) -> Option<&CoinEntry> {
        self.coins.first()
    }

    pub fn smallest(&self) -> Option<&CoinEntry> {
        self.coins.last()
    }

    pub fn iter(&self) -> impl Iterator<Item = &CoinEntry> {
        self.coins.iter()
    }

    /// Object references for the top-`n` coins by balance. Caller
    /// uses these as the explicit gas coin for parallel
    /// transactions (one coin per tx — they can't share without
    /// version-conflicting).
    pub fn top_n_refs(&self, n: usize) -> Vec<ObjectReference> {
        self.coins.iter().take(n).map(|c| c.object_ref).collect()
    }

    pub fn find(&self, id: ObjectId) -> Option<&CoinEntry> {
        self.coins.iter().find(|c| c.id == id)
    }

    /// Pull a gas coin + `fee_count` fee coins out of the pool.
    ///
    /// `gas_min` / `fee_min` are minimum-balance gates so the allocator
    /// doesn't hand back coins that'll fail the planned tx. Picks
    /// largest-first for both slots, the gas coin always coming from
    /// `coins[0]`.
    pub fn allocate(
        &self,
        gas_min: u64,
        fee_count: usize,
        fee_min: u64,
    ) -> Result<Allocation, PoolError> {
        let largest = self.coins.first().ok_or(PoolError::Empty {
            owner: Address::ZERO,
        })?;
        if largest.balance < gas_min {
            return Err(PoolError::NoCoinBigEnough {
                needed: gas_min,
                largest: largest.balance,
            });
        }
        let gas = largest.clone();

        // Skip index 0 (gas coin), take next N >= fee_min.
        let fees: Vec<CoinEntry> = self
            .coins
            .iter()
            .skip(1)
            .filter(|c| c.balance >= fee_min)
            .take(fee_count)
            .cloned()
            .collect();
        if fees.len() < fee_count {
            return Err(PoolError::InsufficientFeeCoins {
                fee_count,
                fee_min,
                available: fees.len(),
            });
        }
        Ok(Allocation { gas, fees })
    }

    /// Build a PTB that splits `source` into `count` outputs of
    /// exactly `amount_each` MIST. Outputs are transferred back to
    /// `sender` so they show up as fresh owned `Coin<IOTA>` objects in
    /// the pool. The remainder stays in `source`.
    ///
    /// Source selection rules:
    /// - [`Source::Gas`] (no `--source`): the chain's gas coin pays
    ///   for itself *and* is sliced — works with as few as 1 coin in
    ///   the pool, the bootstrap path.
    /// - [`Source::Coin(entry)`]: the named coin is declared as an
    ///   explicit input; the helper registers it in the
    ///   `PtbBuilder`'s cache so `with_auto_gas` skips it and picks
    ///   the next-largest coin for gas.
    pub fn build_split_ptb(
        sender: Address,
        source: Source<'_>,
        amount_each: u64,
        count: usize,
    ) -> PtbBuilder {
        let mut ptb = PtbBuilder::new(sender);
        let source_arg = bind_source(&mut ptb, source);
        let mut amounts = Vec::with_capacity(count);
        for _ in 0..count {
            amounts.push(ptb.inner.pure(amount_each));
        }
        let split_result = ptb.inner.command(Command::SplitCoins(SplitCoins {
            coin: source_arg,
            amounts,
        }));
        let split_args: Vec<Argument> = (0..count as u16)
            .map(|i| Argument::NestedResult(extract_result_idx(split_result), i))
            .collect();
        let recipient = ptb.inner.pure(sender);
        ptb.inner.command(Command::TransferObjects(TransferObjects {
            objects: split_args,
            address: recipient,
        }));
        ptb
    }

    /// Build a PTB that merges every coin in `to_merge` into `into`.
    /// All `to_merge` coins are consumed; their balances land in
    /// `into`. The merge-source coins are registered in the cache so
    /// `with_auto_gas` doesn't pick one of them for gas.
    pub fn build_merge_ptb(
        sender: Address,
        into: Source<'_>,
        to_merge: &[CoinEntry],
    ) -> PtbBuilder {
        let mut ptb = PtbBuilder::new(sender);
        let into_arg = bind_source(&mut ptb, into);
        let source_args: Vec<Argument> = to_merge
            .iter()
            .map(|c| {
                // Tell auto-gas to skip this coin — it's about to get
                // consumed by the merge command.
                ptb.register_owned(c.id, c.object_ref);
                ptb.inner.input(Input::ImmutableOrOwned(c.object_ref))
            })
            .collect();
        ptb.inner.command(Command::MergeCoins(MergeCoins {
            coin: into_arg,
            coins_to_merge: source_args,
        }));
        ptb
    }

    /// Build a "rebalance" PTB: merge every IOTA coin into the
    /// largest (which becomes the tx's gas coin), then slice
    /// `min(target_count, total / target_niota)` fresh coins of
    /// `target_niota` off it and transfer them back to `sender`.
    /// The remainder stays in the gas coin.
    ///
    /// Use to keep a hot pool of N owned coins at a target size for
    /// parallel-tx workloads where each tx needs its own gas coin.
    /// Errors if the pool is empty.
    pub fn build_rebalance_ptb(
        &self,
        sender: Address,
        target_niota: u64,
        target_count: usize,
    ) -> Result<(PtbBuilder, usize), PoolError> {
        let Some(gas) = self.largest() else {
            return Err(PoolError::NothingToRebalance);
        };
        let total: u64 = self.iter().map(|c| c.balance).sum();
        let split_count = std::cmp::min(target_count as u64, total / target_niota.max(1)) as usize;
        if split_count == 0 {
            return Err(PoolError::InsufficientBalance {
                have: total,
                need: target_niota,
            });
        }

        let mut ptb = PtbBuilder::new(sender);
        ptb.gas([gas.object_ref]);
        let others: Vec<Argument> = self
            .iter()
            .filter(|c| c.id != gas.id)
            .map(|c| {
                ptb.register_owned(c.id, c.object_ref);
                ptb.inner.input(Input::ImmutableOrOwned(c.object_ref))
            })
            .collect();
        if !others.is_empty() {
            ptb.inner.command(Command::MergeCoins(MergeCoins {
                coin: Argument::Gas,
                coins_to_merge: others,
            }));
        }
        let amounts: Vec<Argument> = (0..split_count)
            .map(|_| ptb.inner.pure(target_niota))
            .collect();
        let split_result = ptb.inner.command(Command::SplitCoins(SplitCoins {
            coin: Argument::Gas,
            amounts,
        }));
        let split_idx = extract_result_idx(split_result);
        let split_args: Vec<Argument> = (0..split_count as u16)
            .map(|i| Argument::NestedResult(split_idx, i))
            .collect();
        let to = ptb.inner.pure(sender);
        ptb.inner.command(Command::TransferObjects(TransferObjects {
            objects: split_args,
            address: to,
        }));
        Ok((ptb, split_count))
    }

    /// Build a PTB that splits `amount` MIST off `source`, transfers
    /// it to `recipient`, leaves the remainder with `sender`. Same
    /// source-selection rules as [`Self::build_split_ptb`].
    pub fn build_transfer_ptb(
        sender: Address,
        source: Source<'_>,
        recipient: Address,
        amount: u64,
    ) -> PtbBuilder {
        let mut ptb = PtbBuilder::new(sender);
        let source_arg = bind_source(&mut ptb, source);
        let amount_arg = ptb.inner.pure(amount);
        let split_result = ptb.inner.command(Command::SplitCoins(SplitCoins {
            coin: source_arg,
            amounts: vec![amount_arg],
        }));
        let payment = Argument::NestedResult(extract_result_idx(split_result), 0);
        let to = ptb.inner.pure(recipient);
        ptb.inner.command(Command::TransferObjects(TransferObjects {
            objects: vec![payment],
            address: to,
        }));
        ptb
    }
}

/// Which coin acts as the *source* (or destination, for merge) for a
/// pool maintenance op. See [`CoinPool::build_split_ptb`] /
/// [`CoinPool::build_merge_ptb`] / [`CoinPool::build_transfer_ptb`].
#[derive(Debug, Clone, Copy)]
pub enum Source<'a> {
    /// The PTB's gas coin. The chain handles "gas pays for itself +
    /// gets sliced" natively; no need to declare an explicit input.
    /// Works with a single-coin pool.
    Gas,
    /// Use the named coin as an explicit input. The helper registers
    /// it in the `PtbBuilder` cache so `with_auto_gas` won't try to
    /// pick the same object for gas.
    Coin(&'a CoinEntry),
}

/// Resolve a [`Source`] into the `Argument` the SDK builder needs.
/// For [`Source::Coin`], also registers the coin in the cache so
/// downstream `with_auto_gas` skips it.
fn bind_source(ptb: &mut PtbBuilder, source: Source<'_>) -> Argument {
    match source {
        Source::Gas => Argument::Gas,
        Source::Coin(c) => {
            ptb.register_owned(c.id, c.object_ref);
            ptb.inner.input(Input::ImmutableOrOwned(c.object_ref))
        }
    }
}

impl Allocation {
    /// Bind the gas coin and fee coins onto `ptb`. Sets the gas slot,
    /// adds each fee coin as `Input::ImmutableOrOwned`, and returns the
    /// per-fee `Argument`s in the same order as [`Self::fees`] so
    /// callers can pass them to `Coin<IOTA>` parameters.
    pub fn apply(&self, ptb: &mut PtbBuilder) -> Vec<Argument> {
        ptb.gas([self.gas.object_ref]);
        self.fees
            .iter()
            .map(|c| ptb.inner.input(Input::ImmutableOrOwned(c.object_ref)))
            .collect()
    }
}

/// `inner.command(Command::SplitCoins(...))` returns the result handle
/// as an `Argument::Result(idx)`. Pull the index out so we can manually
/// construct `Argument::NestedResult(idx, sub)` handles per output —
/// the SDK builder's stateful "current command" wrapper would let us
/// do this too but the explicit path is clearer here.
fn extract_result_idx(result: Argument) -> u16 {
    match result {
        Argument::Result(idx) => idx,
        other => panic!("expected Argument::Result from command, got {other:?}"),
    }
}

// Silence unused-import warnings on items we'll surface later.
#[allow(dead_code)]
fn _imports_smoke() {
    let _ = Identifier::new("x");
    let _: Option<TypeTag> = None;
    let _: Option<StructTag> = None;
}
