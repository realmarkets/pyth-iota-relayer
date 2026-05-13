#![allow(unused_imports, non_snake_case, non_upper_case_globals)]
use move_bindgen_runtime::*;
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;
pub const ACTION_SET_FEE: u8 = 3u8;
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GovernanceWitness {
    pub dummy_field: bool,
}
impl MoveType for GovernanceWitness {
    type Package = super::Package;
    const MODULE: &'static str = "set_fee";
    const NAME: &'static str = "GovernanceWitness";
}
impl MoveArg for GovernanceWitness {
    fn pure_bytes(self) -> PureBytes {
        PureBytes(::bcs::to_bytes(&self).expect("bcs serialization failed"))
    }
}
pub trait ArgumentGovernanceWitness: PTBArgument {
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
impl ArgumentGovernanceWitness for GovernanceWitness {}
impl ArgumentGovernanceWitness for Argument {}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SetFee {
    pub amount: u64,
}
impl MoveType for SetFee {
    type Package = super::Package;
    const MODULE: &'static str = "set_fee";
    const NAME: &'static str = "SetFee";
}
impl MoveArg for SetFee {
    fn pure_bytes(self) -> PureBytes {
        PureBytes(::bcs::to_bytes(&self).expect("bcs serialization failed"))
    }
}
pub trait ArgumentSetFee: PTBArgument {
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
impl ArgumentSetFee for SetFee {}
impl ArgumentSetFee for Argument {}
/// Returns: `0xff00000000000002::governance_message::DecreeTicket<0xff00000000000002::set_fee::GovernanceWitness>`.
pub async fn authorize_governance(
    b: &mut PtbBuilder,
    arg0: impl super::state::ArgumentState,
) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "set_fee",
        "authorize_governance",
        Vec::new(),
        vec![a0],
    )
}
/// Returns: `u64`.
pub async fn set_fee(
    b: &mut PtbBuilder,
    arg0: impl super::state::ArgumentState,
    arg1: impl super::governance_message::ArgumentDecreeReceipt<GovernanceWitness>,
) -> Argument {
    let a0 = arg0.into_argument_mut(b).await;
    let a1 = arg1.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "set_fee",
        "set_fee",
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
    pub fn governancewitness_tag(&self) -> TypeTag {
        <GovernanceWitness as MoveType>::type_tag_at(self.package)
    }
    pub fn setfee_tag(&self) -> TypeTag {
        <SetFee as MoveType>::type_tag_at(self.package)
    }
}
