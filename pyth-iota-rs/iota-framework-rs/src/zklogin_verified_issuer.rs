#![allow(unused_imports, non_snake_case, non_upper_case_globals)]
use move_bindgen_runtime::*;
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;
pub const EInvalidInput: u64 = 0u64;
pub const EInvalidProof: u64 = 1u64;
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct VerifiedIssuer {
    pub id: UID,
    pub owner: Address,
    pub issuer: String,
}
impl MoveType for VerifiedIssuer {
    type Package = super::Package;
    const MODULE: &'static str = "zklogin_verified_issuer";
    const NAME: &'static str = "VerifiedIssuer";
}
pub trait ArgumentVerifiedIssuer: PTBArgument {
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
impl ArgumentVerifiedIssuer for Argument {}
impl ArgumentVerifiedIssuer for ObjectId {
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
impl ArgumentVerifiedIssuer for ObjectReference {}
impl ArgumentVerifiedIssuer for Shared<ObjectId> {}
impl ArgumentVerifiedIssuer for SharedMut<ObjectId> {}
impl ArgumentVerifiedIssuer for Receiving<ObjectId> {}
/// Returns: `address`.
pub async fn owner(b: &mut PtbBuilder, arg0: impl ArgumentVerifiedIssuer) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "zklogin_verified_issuer",
        "owner",
        Vec::new(),
        vec![a0],
    )
}
/// Returns: `&0x1::string::String`.
pub async fn issuer(b: &mut PtbBuilder, arg0: impl ArgumentVerifiedIssuer) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "zklogin_verified_issuer",
        "issuer",
        Vec::new(),
        vec![a0],
    )
}
pub async fn delete(b: &mut PtbBuilder, arg0: impl ArgumentVerifiedIssuer) {
    let a0 = arg0.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "zklogin_verified_issuer",
        "delete",
        Vec::new(),
        vec![a0],
    );
}
pub async fn verify_zklogin_issuer(
    b: &mut PtbBuilder,
    arg0: impl PureU256,
    arg1: impl PureString,
) {
    let a0 = arg0.into_argument(b).await;
    let a1 = arg1.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "zklogin_verified_issuer",
        "verify_zklogin_issuer",
        Vec::new(),
        vec![a0, a1],
    );
}
/// Returns: `bool`.
pub async fn check_zklogin_issuer(
    b: &mut PtbBuilder,
    arg0: impl PureAddress,
    arg1: impl PureU256,
    arg2: impl PureString,
) -> Argument {
    let a0 = arg0.into_argument(b).await;
    let a1 = arg1.into_argument(b).await;
    let a2 = arg2.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "zklogin_verified_issuer",
        "check_zklogin_issuer",
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
    pub fn verifiedissuer_tag(&self) -> TypeTag {
        <VerifiedIssuer as MoveType>::type_tag_at(self.package)
    }
}
