#![allow(unused_imports, non_snake_case, non_upper_case_globals)]
//! This module implements the global state variables for Wormhole as a shared
//! object. The `State` object is used to perform anything that requires access
//! to data that defines the Wormhole contract. Examples of which are publishing
//! Wormhole messages (requires depositing a message fee), verifying `VAA` by
//! checking signatures versus an existing Guardian set, and generating new
//! emitters for Wormhole integrators.
use move_bindgen_runtime::*;
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;
/// Cannot initialize state with zero guardians.
pub const E_ZERO_GUARDIANS: u64 = 0u64;
/// Build digest does not agree with current implementation.
pub const E_INVALID_BUILD_DIGEST: u64 = 1u64;
/// Iota's chain ID is hard-coded to one value.
pub const CHAIN_ID: u16 = 50118u16;
/// Capability reflecting that the current build version is used to invoke
/// state methods.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LatestOnly {
    pub dummy_field: bool,
}
impl MoveType for LatestOnly {
    type Package = super::Package;
    const MODULE: &'static str = "state";
    const NAME: &'static str = "LatestOnly";
}
impl MoveArg for LatestOnly {
    fn pure_bytes(self) -> PureBytes {
        PureBytes(::bcs::to_bytes(&self).expect("bcs serialization failed"))
    }
}
pub trait ArgumentLatestOnly: PTBArgument {
    #[allow(async_fn_in_trait)]
    async fn into_argument(self, b: &mut PtbBuilder) -> Argument
    where
        Self: Sized,
    {
        b.inner.apply_argument(self)
    }
    #[allow(async_fn_in_trait)]
    async fn into_argument_ref(self, b: &mut PtbBuilder) -> Argument
    where
        Self: Sized,
    {
        self.into_argument(b).await
    }
    #[allow(async_fn_in_trait)]
    async fn into_argument_mut(self, b: &mut PtbBuilder) -> Argument
    where
        Self: Sized,
    {
        self.into_argument(b).await
    }
}
impl ArgumentLatestOnly for LatestOnly {}
impl ArgumentLatestOnly for Argument {}
/// Container for all state variables for Wormhole.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct State {
    pub id: UID,
    /// Governance chain ID.
    pub governance_chain: u16,
    /// Governance contract address.
    pub governance_contract: super::external_address::ExternalAddress,
    /// Current active guardian set index.
    pub guardian_set_index: u32,
    /// All guardian sets (including expired ones).
    pub guardian_sets: ::iota_framework_rs::table::Table<
        u32,
        super::guardian_set::GuardianSet,
    >,
    /// Period for which a guardian set stays active after it has been
    /// replaced.
    ///
    /// NOTE: `Clock` timestamp is in units of ms while this value is in
    /// terms of seconds. See `guardian_set` module for more info.
    pub guardian_set_seconds_to_live: u32,
    /// Consumed VAA hashes to protect against replay. VAAs relevant to
    /// Wormhole are just governance VAAs.
    pub consumed_vaas: super::consumed_vaas::ConsumedVAAs,
    /// Wormhole fee collector.
    pub fee_collector: super::fee_collector::FeeCollector,
    /// Upgrade capability.
    pub upgrade_cap: ::iota_framework_rs::package::UpgradeCap,
}
impl MoveType for State {
    type Package = super::Package;
    const MODULE: &'static str = "state";
    const NAME: &'static str = "State";
}
pub trait ArgumentState: PTBArgument {
    #[allow(async_fn_in_trait)]
    async fn into_argument(self, b: &mut PtbBuilder) -> Argument
    where
        Self: Sized,
    {
        b.inner.apply_argument(self)
    }
    #[allow(async_fn_in_trait)]
    async fn into_argument_ref(self, b: &mut PtbBuilder) -> Argument
    where
        Self: Sized,
    {
        self.into_argument(b).await
    }
    #[allow(async_fn_in_trait)]
    async fn into_argument_mut(self, b: &mut PtbBuilder) -> Argument
    where
        Self: Sized,
    {
        self.into_argument(b).await
    }
}
impl ArgumentState for Argument {}
impl ArgumentState for ObjectId {
    async fn into_argument(self, b: &mut PtbBuilder) -> Argument {
        b.resolve_object(self).await
    }
    async fn into_argument_ref(self, b: &mut PtbBuilder) -> Argument {
        b.resolve_object_shared(self, false).await
    }
    async fn into_argument_mut(self, b: &mut PtbBuilder) -> Argument {
        b.resolve_object_shared(self, true).await
    }
}
impl ArgumentState for ObjectReference {}
impl ArgumentState for Shared<ObjectId> {}
impl ArgumentState for SharedMut<ObjectId> {}
impl ArgumentState for Receiving<ObjectId> {}
/// Convenience method to get hard-coded Wormhole chain ID (recognized by
/// the Wormhole network).
///
/// Returns: `u16`.
pub async fn chain_id(b: &mut PtbBuilder) -> Argument {
    b.move_call(
        b.package_id::<super::Package>(),
        "state",
        "chain_id",
        Vec::new(),
        vec![],
    )
}
/// Retrieve governance module name.
///
/// Returns: `0xff00000000000002::bytes32::Bytes32`.
pub async fn governance_module(b: &mut PtbBuilder) -> Argument {
    b.move_call(
        b.package_id::<super::Package>(),
        "state",
        "governance_module",
        Vec::new(),
        vec![],
    )
}
/// Retrieve governance chain ID, which is governance's emitter chain ID.
///
/// Returns: `u16`.
pub async fn governance_chain(b: &mut PtbBuilder, arg0: impl ArgumentState) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "state",
        "governance_chain",
        Vec::new(),
        vec![a0],
    )
}
/// Retrieve governance emitter address.
///
/// Returns: `0xff00000000000002::external_address::ExternalAddress`.
pub async fn governance_contract(
    b: &mut PtbBuilder,
    arg0: impl ArgumentState,
) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "state",
        "governance_contract",
        Vec::new(),
        vec![a0],
    )
}
/// Retrieve current Guardian set index. This value is important for
/// verifying VAA signatures and especially important for governance VAAs.
///
/// Returns: `u32`.
pub async fn guardian_set_index(
    b: &mut PtbBuilder,
    arg0: impl ArgumentState,
) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "state",
        "guardian_set_index",
        Vec::new(),
        vec![a0],
    )
}
/// Retrieve how long after a Guardian set can live for in terms of Iota
/// timestamp (in seconds).
///
/// Returns: `u32`.
pub async fn guardian_set_seconds_to_live(
    b: &mut PtbBuilder,
    arg0: impl ArgumentState,
) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "state",
        "guardian_set_seconds_to_live",
        Vec::new(),
        vec![a0],
    )
}
/// Retrieve a particular Guardian set by its Guardian set index. This
/// method is used when verifying a VAA.
///
/// See `wormhole::vaa` for more info.
///
/// Returns: `&0xff00000000000002::guardian_set::GuardianSet`.
pub async fn guardian_set_at(
    b: &mut PtbBuilder,
    arg0: impl ArgumentState,
    arg1: impl PureU32,
) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    let a1 = arg1.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "state",
        "guardian_set_at",
        Vec::new(),
        vec![a0, a1],
    )
}
/// Retrieve current fee to send Wormhole message.
///
/// Returns: `u64`.
pub async fn message_fee(b: &mut PtbBuilder, arg0: impl ArgumentState) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "state",
        "message_fee",
        Vec::new(),
        vec![a0],
    )
}
/// Read-only handle bound to a runtime package address. Use
/// via `super::Package::at(addr).<module>()` and chain
/// `<type>_tag()` to build a [`TypeTag`] without spinning up
/// a `PtbBuilder`.
pub struct ModuleAt {
    pub(crate) package: Address,
}
impl ModuleAt {
    pub fn latestonly_tag(&self) -> TypeTag {
        <LatestOnly as MoveType>::type_tag_at(self.package)
    }
    pub fn state_tag(&self) -> TypeTag {
        <State as MoveType>::type_tag_at(self.package)
    }
}
