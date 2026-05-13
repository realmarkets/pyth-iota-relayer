#![allow(unused_imports, non_snake_case, non_upper_case_globals)]
use move_bindgen_runtime::*;
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;
/// Build digest does not agree with current implementation.
pub const E_INVALID_BUILD_DIGEST: u64 = 0u64;
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
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct State {
    pub id: UID,
    pub governance_data_source: super::data_source::DataSource,
    pub stale_price_threshold: u64,
    pub base_update_fee: u64,
    pub fee_recipient_address: Address,
    pub last_executed_governance_sequence: u64,
    pub consumed_vaas: ::wormhole_rs::consumed_vaas::ConsumedVAAs,
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
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CurrentDigest {
    pub dummy_field: bool,
}
impl MoveType for CurrentDigest {
    type Package = super::Package;
    const MODULE: &'static str = "state";
    const NAME: &'static str = "CurrentDigest";
}
impl MoveArg for CurrentDigest {
    fn pure_bytes(self) -> PureBytes {
        PureBytes(::bcs::to_bytes(&self).expect("bcs serialization failed"))
    }
}
pub trait ArgumentCurrentDigest: PTBArgument {
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
impl ArgumentCurrentDigest for CurrentDigest {}
impl ArgumentCurrentDigest for Argument {}
/// Returns: `u64`.
pub async fn get_stale_price_threshold_secs(
    b: &mut PtbBuilder,
    arg0: impl ArgumentState,
) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "state",
        "get_stale_price_threshold_secs",
        Vec::new(),
        vec![a0],
    )
}
/// Returns: `u64`.
pub async fn get_base_update_fee(
    b: &mut PtbBuilder,
    arg0: impl ArgumentState,
) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "state",
        "get_base_update_fee",
        Vec::new(),
        vec![a0],
    )
}
/// Returns: `address`.
pub async fn get_fee_recipient(
    b: &mut PtbBuilder,
    arg0: impl ArgumentState,
) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "state",
        "get_fee_recipient",
        Vec::new(),
        vec![a0],
    )
}
/// Returns: `bool`.
pub async fn is_valid_data_source(
    b: &mut PtbBuilder,
    arg0: impl ArgumentState,
    arg1: impl super::data_source::ArgumentDataSource,
) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    let a1 = arg1.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "state",
        "is_valid_data_source",
        Vec::new(),
        vec![a0, a1],
    )
}
/// Returns: `bool`.
pub async fn is_valid_governance_data_source(
    b: &mut PtbBuilder,
    arg0: impl ArgumentState,
    arg1: impl super::data_source::ArgumentDataSource,
) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    let a1 = arg1.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "state",
        "is_valid_governance_data_source",
        Vec::new(),
        vec![a0, a1],
    )
}
/// Returns: `bool`.
pub async fn price_feed_object_exists(
    b: &mut PtbBuilder,
    arg0: impl ArgumentState,
    arg1: impl super::price_identifier::ArgumentPriceIdentifier,
) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    let a1 = arg1.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "state",
        "price_feed_object_exists",
        Vec::new(),
        vec![a0, a1],
    )
}
/// Retrieve governance chain ID, which is governance's emitter chain ID.
///
/// Returns: `0xff00000000000001::data_source::DataSource`.
pub async fn governance_data_source(
    b: &mut PtbBuilder,
    arg0: impl ArgumentState,
) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "state",
        "governance_data_source",
        Vec::new(),
        vec![a0],
    )
}
/// Returns: `u64`.
pub async fn get_last_executed_governance_sequence(
    b: &mut PtbBuilder,
    arg0: impl ArgumentState,
) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "state",
        "get_last_executed_governance_sequence",
        Vec::new(),
        vec![a0],
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
/// Returns: `0x2::object::ID`.
pub async fn get_price_info_object_id(
    b: &mut PtbBuilder,
    arg0: impl ArgumentState,
    arg1: impl PureVec<u8>,
) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    let a1 = arg1.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "state",
        "get_price_info_object_id",
        Vec::new(),
        vec![a0, a1],
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
    pub fn currentdigest_tag(&self) -> TypeTag {
        <CurrentDigest as MoveType>::type_tag_at(self.package)
    }
}
