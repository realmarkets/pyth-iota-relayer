#![allow(unused_imports, non_snake_case, non_upper_case_globals)]
use move_bindgen_runtime::*;
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;
pub const CONTRACT_UPGRADE: u8 = 0u8;
pub const SET_GOVERNANCE_DATA_SOURCE: u8 = 1u8;
pub const SET_DATA_SOURCES: u8 = 2u8;
pub const SET_UPDATE_FEE: u8 = 3u8;
pub const SET_STALE_PRICE_THRESHOLD: u8 = 4u8;
pub const SET_FEE_RECIPIENT: u8 = 5u8;
pub const E_INVALID_GOVERNANCE_ACTION: u64 = 6u64;
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GovernanceAction {
    pub value: u8,
}
impl MoveType for GovernanceAction {
    type Package = super::Package;
    const MODULE: &'static str = "governance_action";
    const NAME: &'static str = "GovernanceAction";
}
impl MoveArg for GovernanceAction {
    fn pure_bytes(self) -> PureBytes {
        PureBytes(::bcs::to_bytes(&self).expect("bcs serialization failed"))
    }
}
pub trait ArgumentGovernanceAction: PTBArgument {
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
impl ArgumentGovernanceAction for GovernanceAction {}
impl ArgumentGovernanceAction for Argument {}
/// Returns: `0xff00000000000001::governance_action::GovernanceAction`.
pub async fn from_u8(b: &mut PtbBuilder, arg0: impl PureU8) -> Argument {
    let a0 = arg0.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "governance_action",
        "from_u8",
        Vec::new(),
        vec![a0],
    )
}
/// Returns: `u8`.
pub async fn get_value(
    b: &mut PtbBuilder,
    arg0: impl ArgumentGovernanceAction,
) -> Argument {
    let a0 = arg0.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "governance_action",
        "get_value",
        Vec::new(),
        vec![a0],
    )
}
/// Returns: `0xff00000000000001::governance_action::GovernanceAction`.
pub async fn new_contract_upgrade(b: &mut PtbBuilder) -> Argument {
    b.move_call(
        b.package_id::<super::Package>(),
        "governance_action",
        "new_contract_upgrade",
        Vec::new(),
        vec![],
    )
}
/// Returns: `0xff00000000000001::governance_action::GovernanceAction`.
pub async fn new_set_governance_data_source(b: &mut PtbBuilder) -> Argument {
    b.move_call(
        b.package_id::<super::Package>(),
        "governance_action",
        "new_set_governance_data_source",
        Vec::new(),
        vec![],
    )
}
/// Returns: `0xff00000000000001::governance_action::GovernanceAction`.
pub async fn new_set_data_sources(b: &mut PtbBuilder) -> Argument {
    b.move_call(
        b.package_id::<super::Package>(),
        "governance_action",
        "new_set_data_sources",
        Vec::new(),
        vec![],
    )
}
/// Returns: `0xff00000000000001::governance_action::GovernanceAction`.
pub async fn new_set_update_fee(b: &mut PtbBuilder) -> Argument {
    b.move_call(
        b.package_id::<super::Package>(),
        "governance_action",
        "new_set_update_fee",
        Vec::new(),
        vec![],
    )
}
/// Returns: `0xff00000000000001::governance_action::GovernanceAction`.
pub async fn new_set_stale_price_threshold(b: &mut PtbBuilder) -> Argument {
    b.move_call(
        b.package_id::<super::Package>(),
        "governance_action",
        "new_set_stale_price_threshold",
        Vec::new(),
        vec![],
    )
}
/// Returns: `0xff00000000000001::governance_action::GovernanceAction`.
pub async fn new_set_fee_recipient(b: &mut PtbBuilder) -> Argument {
    b.move_call(
        b.package_id::<super::Package>(),
        "governance_action",
        "new_set_fee_recipient",
        Vec::new(),
        vec![],
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
    pub fn governanceaction_tag(&self) -> TypeTag {
        <GovernanceAction as MoveType>::type_tag_at(self.package)
    }
}
