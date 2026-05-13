#![allow(unused_imports, non_snake_case, non_upper_case_globals)]
use move_bindgen_runtime::*;
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;
pub const E_NO_GUARDIANS: u64 = 0u64;
pub const E_NON_INCREMENTAL_GUARDIAN_SETS: u64 = 1u64;
pub const ACTION_UPDATE_GUARDIAN_SET: u8 = 2u8;
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GovernanceWitness {
    pub dummy_field: bool,
}
impl MoveType for GovernanceWitness {
    type Package = super::Package;
    const MODULE: &'static str = "update_guardian_set";
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
pub struct GuardianSetAdded {
    pub new_index: u32,
}
impl MoveType for GuardianSetAdded {
    type Package = super::Package;
    const MODULE: &'static str = "update_guardian_set";
    const NAME: &'static str = "GuardianSetAdded";
}
impl MoveArg for GuardianSetAdded {
    fn pure_bytes(self) -> PureBytes {
        PureBytes(::bcs::to_bytes(&self).expect("bcs serialization failed"))
    }
}
pub trait ArgumentGuardianSetAdded: PTBArgument {
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
impl ArgumentGuardianSetAdded for GuardianSetAdded {}
impl ArgumentGuardianSetAdded for Argument {}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UpdateGuardianSet {
    pub new_index: u32,
    pub guardians: Vec<super::guardian::Guardian>,
}
impl MoveType for UpdateGuardianSet {
    type Package = super::Package;
    const MODULE: &'static str = "update_guardian_set";
    const NAME: &'static str = "UpdateGuardianSet";
}
impl MoveArg for UpdateGuardianSet {
    fn pure_bytes(self) -> PureBytes {
        PureBytes(::bcs::to_bytes(&self).expect("bcs serialization failed"))
    }
}
pub trait ArgumentUpdateGuardianSet: PTBArgument {
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
impl ArgumentUpdateGuardianSet for UpdateGuardianSet {}
impl ArgumentUpdateGuardianSet for Argument {}
/// Returns: `0xff00000000000002::governance_message::DecreeTicket<0xff00000000000002::update_guardian_set::GovernanceWitness>`.
pub async fn authorize_governance(
    b: &mut PtbBuilder,
    arg0: impl super::state::ArgumentState,
) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "update_guardian_set",
        "authorize_governance",
        Vec::new(),
        vec![a0],
    )
}
/// Returns: `u32`.
pub async fn update_guardian_set(
    b: &mut PtbBuilder,
    arg0: impl super::state::ArgumentState,
    arg1: impl super::governance_message::ArgumentDecreeReceipt<GovernanceWitness>,
    arg2: impl ::iota_framework_rs::clock::ArgumentClock,
) -> Argument {
    let a0 = arg0.into_argument_mut(b).await;
    let a1 = arg1.into_argument(b).await;
    let a2 = arg2.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "update_guardian_set",
        "update_guardian_set",
        Vec::new(),
        vec![a0, a1, a2],
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
    pub fn guardiansetadded_tag(&self) -> TypeTag {
        <GuardianSetAdded as MoveType>::type_tag_at(self.package)
    }
    pub fn updateguardianset_tag(&self) -> TypeTag {
        <UpdateGuardianSet as MoveType>::type_tag_at(self.package)
    }
}
