#![allow(unused_imports, non_snake_case, non_upper_case_globals)]
use move_bindgen_runtime::*;
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;
pub const E_ZERO_ADDRESS: u64 = 1u64;
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Guardian {
    pub pubkey: super::bytes20::Bytes20,
}
impl MoveType for Guardian {
    type Package = super::Package;
    const MODULE: &'static str = "guardian";
    const NAME: &'static str = "Guardian";
}
impl MoveArg for Guardian {
    fn pure_bytes(self) -> PureBytes {
        PureBytes(::bcs::to_bytes(&self).expect("bcs serialization failed"))
    }
}
pub trait ArgumentGuardian: PTBArgument {
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
impl ArgumentGuardian for Guardian {}
impl ArgumentGuardian for Argument {}
/// Returns: `0xff00000000000002::guardian::Guardian`.
pub async fn new(b: &mut PtbBuilder, arg0: impl PureVec<u8>) -> Argument {
    let a0 = arg0.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "guardian",
        "new",
        Vec::new(),
        vec![a0],
    )
}
/// Returns: `0xff00000000000002::bytes20::Bytes20`.
pub async fn pubkey(b: &mut PtbBuilder, arg0: impl ArgumentGuardian) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "guardian",
        "pubkey",
        Vec::new(),
        vec![a0],
    )
}
/// Returns: `vector<u8>`.
pub async fn as_bytes(b: &mut PtbBuilder, arg0: impl ArgumentGuardian) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "guardian",
        "as_bytes",
        Vec::new(),
        vec![a0],
    )
}
/// Returns: `bool`.
pub async fn verify(
    b: &mut PtbBuilder,
    arg0: impl ArgumentGuardian,
    arg1: impl super::guardian_signature::ArgumentGuardianSignature,
    arg2: impl PureVec<u8>,
) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    let a1 = arg1.into_argument(b).await;
    let a2 = arg2.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "guardian",
        "verify",
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
    pub fn guardian_tag(&self) -> TypeTag {
        <Guardian as MoveType>::type_tag_at(self.package)
    }
}
