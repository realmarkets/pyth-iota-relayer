#![allow(unused_imports, non_snake_case, non_upper_case_globals)]
use move_bindgen_runtime::*;
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;
pub const E_DIGEST_ZERO_BYTES: u64 = 0u64;
pub const ACTION_UPGRADE_CONTRACT: u8 = 1u8;
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GovernanceWitness {
    pub dummy_field: bool,
}
impl MoveType for GovernanceWitness {
    type Package = super::Package;
    const MODULE: &'static str = "upgrade_contract";
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
pub struct ContractUpgraded {
    pub old_contract: ID,
    pub new_contract: ID,
}
impl MoveType for ContractUpgraded {
    type Package = super::Package;
    const MODULE: &'static str = "upgrade_contract";
    const NAME: &'static str = "ContractUpgraded";
}
impl MoveArg for ContractUpgraded {
    fn pure_bytes(self) -> PureBytes {
        PureBytes(::bcs::to_bytes(&self).expect("bcs serialization failed"))
    }
}
pub trait ArgumentContractUpgraded: PTBArgument {
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
impl ArgumentContractUpgraded for ContractUpgraded {}
impl ArgumentContractUpgraded for Argument {}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UpgradeContract {
    pub digest: super::bytes32::Bytes32,
}
impl MoveType for UpgradeContract {
    type Package = super::Package;
    const MODULE: &'static str = "upgrade_contract";
    const NAME: &'static str = "UpgradeContract";
}
impl MoveArg for UpgradeContract {
    fn pure_bytes(self) -> PureBytes {
        PureBytes(::bcs::to_bytes(&self).expect("bcs serialization failed"))
    }
}
pub trait ArgumentUpgradeContract: PTBArgument {
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
impl ArgumentUpgradeContract for UpgradeContract {}
impl ArgumentUpgradeContract for Argument {}
/// Returns: `0xff00000000000002::governance_message::DecreeTicket<0xff00000000000002::upgrade_contract::GovernanceWitness>`.
pub async fn authorize_governance(
    b: &mut PtbBuilder,
    arg0: impl super::state::ArgumentState,
) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "upgrade_contract",
        "authorize_governance",
        Vec::new(),
        vec![a0],
    )
}
/// Returns: `0x2::package::UpgradeTicket`.
pub async fn authorize_upgrade(
    b: &mut PtbBuilder,
    arg0: impl super::state::ArgumentState,
    arg1: impl super::governance_message::ArgumentDecreeReceipt<GovernanceWitness>,
) -> Argument {
    let a0 = arg0.into_argument_mut(b).await;
    let a1 = arg1.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "upgrade_contract",
        "authorize_upgrade",
        Vec::new(),
        vec![a0, a1],
    )
}
pub async fn commit_upgrade(
    b: &mut PtbBuilder,
    arg0: impl super::state::ArgumentState,
    arg1: impl ::iota_framework_rs::package::ArgumentUpgradeReceipt,
) {
    let a0 = arg0.into_argument_mut(b).await;
    let a1 = arg1.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "upgrade_contract",
        "commit_upgrade",
        Vec::new(),
        vec![a0, a1],
    );
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
    pub fn contractupgraded_tag(&self) -> TypeTag {
        <ContractUpgraded as MoveType>::type_tag_at(self.package)
    }
    pub fn upgradecontract_tag(&self) -> TypeTag {
        <UpgradeContract as MoveType>::type_tag_at(self.package)
    }
}
