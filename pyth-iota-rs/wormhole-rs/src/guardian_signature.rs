#![allow(unused_imports, non_snake_case, non_upper_case_globals)]
use move_bindgen_runtime::*;
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GuardianSignature {
    pub r: super::bytes32::Bytes32,
    pub s: super::bytes32::Bytes32,
    pub recovery_id: u8,
    pub index: u8,
}
impl MoveType for GuardianSignature {
    type Package = super::Package;
    const MODULE: &'static str = "guardian_signature";
    const NAME: &'static str = "GuardianSignature";
}
impl MoveArg for GuardianSignature {
    fn pure_bytes(self) -> PureBytes {
        PureBytes(::bcs::to_bytes(&self).expect("bcs serialization failed"))
    }
}
pub trait ArgumentGuardianSignature: PTBArgument {
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
impl ArgumentGuardianSignature for GuardianSignature {}
impl ArgumentGuardianSignature for Argument {}
/// Returns: `0xff00000000000002::guardian_signature::GuardianSignature`.
pub async fn new(
    b: &mut PtbBuilder,
    arg0: impl super::bytes32::ArgumentBytes32,
    arg1: impl super::bytes32::ArgumentBytes32,
    arg2: impl PureU8,
    arg3: impl PureU8,
) -> Argument {
    let a0 = arg0.into_argument(b).await;
    let a1 = arg1.into_argument(b).await;
    let a2 = arg2.into_argument(b).await;
    let a3 = arg3.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "guardian_signature",
        "new",
        Vec::new(),
        vec![a0, a1, a2, a3],
    )
}
/// Returns: `0xff00000000000002::bytes32::Bytes32`.
pub async fn r(b: &mut PtbBuilder, arg0: impl ArgumentGuardianSignature) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "guardian_signature",
        "r",
        Vec::new(),
        vec![a0],
    )
}
/// Returns: `0xff00000000000002::bytes32::Bytes32`.
pub async fn s(b: &mut PtbBuilder, arg0: impl ArgumentGuardianSignature) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "guardian_signature",
        "s",
        Vec::new(),
        vec![a0],
    )
}
/// Returns: `u8`.
pub async fn recovery_id(
    b: &mut PtbBuilder,
    arg0: impl ArgumentGuardianSignature,
) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "guardian_signature",
        "recovery_id",
        Vec::new(),
        vec![a0],
    )
}
/// Returns: `u8`.
pub async fn index(
    b: &mut PtbBuilder,
    arg0: impl ArgumentGuardianSignature,
) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "guardian_signature",
        "index",
        Vec::new(),
        vec![a0],
    )
}
/// Returns: `u64`.
pub async fn index_as_u64(
    b: &mut PtbBuilder,
    arg0: impl ArgumentGuardianSignature,
) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "guardian_signature",
        "index_as_u64",
        Vec::new(),
        vec![a0],
    )
}
/// Returns: `vector<u8>`.
pub async fn to_rsv(
    b: &mut PtbBuilder,
    arg0: impl ArgumentGuardianSignature,
) -> Argument {
    let a0 = arg0.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "guardian_signature",
        "to_rsv",
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
    pub fn guardiansignature_tag(&self) -> TypeTag {
        <GuardianSignature as MoveType>::type_tag_at(self.package)
    }
}
