#![allow(unused_imports, non_snake_case, non_upper_case_globals)]
use move_bindgen_runtime::*;
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;
pub const MAGIC: &[u8] = &[80u8, 84u8, 71u8, 77u8];
pub const MODULE: u8 = 1u8;
pub const E_INVALID_GOVERNANCE_MODULE: u64 = 0u64;
pub const E_INVALID_GOVERNANCE_MAGIC_VALUE: u64 = 1u64;
pub const E_TARGET_CHAIN_MISMATCH: u64 = 2u64;
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GovernanceInstruction {
    pub module_: u8,
    pub action: super::governance_action::GovernanceAction,
    pub target_chain_id: u64,
    pub payload: Vec<u8>,
}
impl MoveType for GovernanceInstruction {
    type Package = super::Package;
    const MODULE: &'static str = "governance_instruction";
    const NAME: &'static str = "GovernanceInstruction";
}
impl MoveArg for GovernanceInstruction {
    fn pure_bytes(self) -> PureBytes {
        PureBytes(::bcs::to_bytes(&self).expect("bcs serialization failed"))
    }
}
pub trait ArgumentGovernanceInstruction: PTBArgument {
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
impl ArgumentGovernanceInstruction for GovernanceInstruction {}
impl ArgumentGovernanceInstruction for Argument {}
/// Returns: `0xff00000000000001::governance_instruction::GovernanceInstruction`.
pub async fn from_byte_vec(b: &mut PtbBuilder, arg0: impl PureVec<u8>) -> Argument {
    let a0 = arg0.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "governance_instruction",
        "from_byte_vec",
        Vec::new(),
        vec![a0],
    )
}
/// Returns: `u8`.
pub async fn get_module(
    b: &mut PtbBuilder,
    arg0: impl ArgumentGovernanceInstruction,
) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "governance_instruction",
        "get_module",
        Vec::new(),
        vec![a0],
    )
}
/// Returns: `0xff00000000000001::governance_action::GovernanceAction`.
pub async fn get_action(
    b: &mut PtbBuilder,
    arg0: impl ArgumentGovernanceInstruction,
) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "governance_instruction",
        "get_action",
        Vec::new(),
        vec![a0],
    )
}
/// Returns: `u64`.
pub async fn get_target_chain_id(
    b: &mut PtbBuilder,
    arg0: impl ArgumentGovernanceInstruction,
) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "governance_instruction",
        "get_target_chain_id",
        Vec::new(),
        vec![a0],
    )
}
/// Returns: `vector<u8>`.
pub async fn destroy(
    b: &mut PtbBuilder,
    arg0: impl ArgumentGovernanceInstruction,
) -> Argument {
    let a0 = arg0.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "governance_instruction",
        "destroy",
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
    pub fn governanceinstruction_tag(&self) -> TypeTag {
        <GovernanceInstruction as MoveType>::type_tag_at(self.package)
    }
}
