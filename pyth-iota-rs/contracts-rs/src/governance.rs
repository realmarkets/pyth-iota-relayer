#![allow(unused_imports, non_snake_case, non_upper_case_globals)]
use move_bindgen_runtime::*;
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;
pub const E_INVALID_GOVERNANCE_ACTION: u64 = 0u64;
pub const E_MUST_USE_CONTRACT_UPGRADE_MODULE_TO_DO_UPGRADES: u64 = 1u64;
pub const E_CANNOT_EXECUTE_GOVERNANCE_ACTION_WITH_OBSOLETE_SEQUENCE_NUMBER: u64 = 2u64;
pub const E_INVALID_GOVERNANCE_DATA_SOURCE: u64 = 4u64;
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WormholeVAAVerificationReceipt {
    pub payload: Vec<u8>,
    pub digest: ::wormhole_rs::bytes32::Bytes32,
    pub sequence: u64,
}
impl MoveType for WormholeVAAVerificationReceipt {
    type Package = super::Package;
    const MODULE: &'static str = "governance";
    const NAME: &'static str = "WormholeVAAVerificationReceipt";
}
impl MoveArg for WormholeVAAVerificationReceipt {
    fn pure_bytes(self) -> PureBytes {
        PureBytes(::bcs::to_bytes(&self).expect("bcs serialization failed"))
    }
}
pub trait ArgumentWormholeVAAVerificationReceipt: PTBArgument {
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
impl ArgumentWormholeVAAVerificationReceipt for WormholeVAAVerificationReceipt {}
impl ArgumentWormholeVAAVerificationReceipt for Argument {}
/// Returns: `vector<u8>`.
pub async fn take_payload(
    b: &mut PtbBuilder,
    arg0: impl ArgumentWormholeVAAVerificationReceipt,
) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "governance",
        "take_payload",
        Vec::new(),
        vec![a0],
    )
}
/// Returns: `0xff00000000000002::bytes32::Bytes32`.
pub async fn take_digest(
    b: &mut PtbBuilder,
    arg0: impl ArgumentWormholeVAAVerificationReceipt,
) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "governance",
        "take_digest",
        Vec::new(),
        vec![a0],
    )
}
/// Returns: `u64`.
pub async fn take_sequence(
    b: &mut PtbBuilder,
    arg0: impl ArgumentWormholeVAAVerificationReceipt,
) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "governance",
        "take_sequence",
        Vec::new(),
        vec![a0],
    )
}
pub async fn destroy(
    b: &mut PtbBuilder,
    arg0: impl ArgumentWormholeVAAVerificationReceipt,
) {
    let a0 = arg0.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "governance",
        "destroy",
        Vec::new(),
        vec![a0],
    );
}
/// Returns: `0xff00000000000001::governance::WormholeVAAVerificationReceipt`.
pub async fn verify_vaa(
    b: &mut PtbBuilder,
    arg0: impl super::state::ArgumentState,
    arg1: impl ::wormhole_rs::vaa::ArgumentVAA,
) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    let a1 = arg1.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "governance",
        "verify_vaa",
        Vec::new(),
        vec![a0, a1],
    )
}
pub async fn execute_governance_instruction(
    b: &mut PtbBuilder,
    arg0: impl super::state::ArgumentState,
    arg1: impl ArgumentWormholeVAAVerificationReceipt,
) {
    let a0 = arg0.into_argument_mut(b).await;
    let a1 = arg1.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "governance",
        "execute_governance_instruction",
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
    pub fn wormholevaaverificationreceipt_tag(&self) -> TypeTag {
        <WormholeVAAVerificationReceipt as MoveType>::type_tag_at(self.package)
    }
}
