#![allow(unused_imports, non_snake_case, non_upper_case_globals)]
use move_bindgen_runtime::*;
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;
pub const EFunctionDisabled: u64 = 0u64;
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct VerifiedID {
    pub id: UID,
    pub owner: Address,
    pub key_claim_name: String,
    pub key_claim_value: String,
    pub issuer: String,
    pub audience: String,
}
impl MoveType for VerifiedID {
    type Package = super::Package;
    const MODULE: &'static str = "zklogin_verified_id";
    const NAME: &'static str = "VerifiedID";
}
pub trait ArgumentVerifiedID: PTBArgument {
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
impl ArgumentVerifiedID for Argument {}
impl ArgumentVerifiedID for ObjectId {
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
impl ArgumentVerifiedID for ObjectReference {}
impl ArgumentVerifiedID for Shared<ObjectId> {}
impl ArgumentVerifiedID for SharedMut<ObjectId> {}
impl ArgumentVerifiedID for Receiving<ObjectId> {}
/// Returns: `address`.
pub async fn owner(b: &mut PtbBuilder, arg0: impl ArgumentVerifiedID) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "zklogin_verified_id",
        "owner",
        Vec::new(),
        vec![a0],
    )
}
/// Returns: `&0x1::string::String`.
pub async fn key_claim_name(
    b: &mut PtbBuilder,
    arg0: impl ArgumentVerifiedID,
) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "zklogin_verified_id",
        "key_claim_name",
        Vec::new(),
        vec![a0],
    )
}
/// Returns: `&0x1::string::String`.
pub async fn key_claim_value(
    b: &mut PtbBuilder,
    arg0: impl ArgumentVerifiedID,
) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "zklogin_verified_id",
        "key_claim_value",
        Vec::new(),
        vec![a0],
    )
}
/// Returns: `&0x1::string::String`.
pub async fn issuer(b: &mut PtbBuilder, arg0: impl ArgumentVerifiedID) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "zklogin_verified_id",
        "issuer",
        Vec::new(),
        vec![a0],
    )
}
/// Returns: `&0x1::string::String`.
pub async fn audience(b: &mut PtbBuilder, arg0: impl ArgumentVerifiedID) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "zklogin_verified_id",
        "audience",
        Vec::new(),
        vec![a0],
    )
}
pub async fn delete(b: &mut PtbBuilder, arg0: impl ArgumentVerifiedID) {
    let a0 = arg0.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "zklogin_verified_id",
        "delete",
        Vec::new(),
        vec![a0],
    );
}
pub async fn verify_zklogin_id(
    b: &mut PtbBuilder,
    arg0: impl PureString,
    arg1: impl PureString,
    arg2: impl PureString,
    arg3: impl PureString,
    arg4: impl PureU256,
) {
    let a0 = arg0.into_argument(b).await;
    let a1 = arg1.into_argument(b).await;
    let a2 = arg2.into_argument(b).await;
    let a3 = arg3.into_argument(b).await;
    let a4 = arg4.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "zklogin_verified_id",
        "verify_zklogin_id",
        Vec::new(),
        vec![a0, a1, a2, a3, a4],
    );
}
/// Returns: `bool`.
pub async fn check_zklogin_id(
    b: &mut PtbBuilder,
    arg0: impl PureAddress,
    arg1: impl PureString,
    arg2: impl PureString,
    arg3: impl PureString,
    arg4: impl PureString,
    arg5: impl PureU256,
) -> Argument {
    let a0 = arg0.into_argument(b).await;
    let a1 = arg1.into_argument_ref(b).await;
    let a2 = arg2.into_argument_ref(b).await;
    let a3 = arg3.into_argument_ref(b).await;
    let a4 = arg4.into_argument_ref(b).await;
    let a5 = arg5.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "zklogin_verified_id",
        "check_zklogin_id",
        Vec::new(),
        vec![a0, a1, a2, a3, a4, a5],
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
    pub fn verifiedid_tag(&self) -> TypeTag {
        <VerifiedID as MoveType>::type_tag_at(self.package)
    }
}
