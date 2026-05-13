#![allow(unused_imports, non_snake_case, non_upper_case_globals)]
use move_bindgen_runtime::*;
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;
/// Sender is not @0x0 the system address.
pub const ENotSystemAddress: u64 = 0u64;
pub const EWrongInnerVersion: u64 = 1u64;
pub const EJwksNotSorted: u64 = 2u64;
/// Singleton shared object which stores the global authenticator state.
/// The actual state is stored in a dynamic field of type AuthenticatorStateInner to support
/// future versions of the authenticator state.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AuthenticatorState {
    pub id: UID,
    pub version: u64,
}
impl MoveType for AuthenticatorState {
    type Package = super::Package;
    const MODULE: &'static str = "authenticator_state";
    const NAME: &'static str = "AuthenticatorState";
}
pub trait ArgumentAuthenticatorState: PTBArgument {
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
impl ArgumentAuthenticatorState for Argument {}
impl ArgumentAuthenticatorState for ObjectId {
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
impl ArgumentAuthenticatorState for ObjectReference {}
impl ArgumentAuthenticatorState for Shared<ObjectId> {}
impl ArgumentAuthenticatorState for SharedMut<ObjectId> {}
impl ArgumentAuthenticatorState for Receiving<ObjectId> {}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AuthenticatorStateInner {
    pub version: u64,
    /// List of currently active JWKs.
    pub active_jwks: Vec<ActiveJwk>,
}
impl MoveType for AuthenticatorStateInner {
    type Package = super::Package;
    const MODULE: &'static str = "authenticator_state";
    const NAME: &'static str = "AuthenticatorStateInner";
}
impl MoveArg for AuthenticatorStateInner {
    fn pure_bytes(self) -> PureBytes {
        PureBytes(::bcs::to_bytes(&self).expect("bcs serialization failed"))
    }
}
pub trait ArgumentAuthenticatorStateInner: PTBArgument {
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
impl ArgumentAuthenticatorStateInner for AuthenticatorStateInner {}
impl ArgumentAuthenticatorStateInner for Argument {}
/// Must match the JWK struct in fastcrypto-zkp
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct JWK {
    pub kty: String,
    pub e: String,
    pub n: String,
    pub alg: String,
}
impl MoveType for JWK {
    type Package = super::Package;
    const MODULE: &'static str = "authenticator_state";
    const NAME: &'static str = "JWK";
}
impl MoveArg for JWK {
    fn pure_bytes(self) -> PureBytes {
        PureBytes(::bcs::to_bytes(&self).expect("bcs serialization failed"))
    }
}
pub trait ArgumentJWK: PTBArgument {
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
impl ArgumentJWK for JWK {}
impl ArgumentJWK for Argument {}
/// Must match the JwkId struct in fastcrypto-zkp
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct JwkId {
    pub iss: String,
    pub kid: String,
}
impl MoveType for JwkId {
    type Package = super::Package;
    const MODULE: &'static str = "authenticator_state";
    const NAME: &'static str = "JwkId";
}
impl MoveArg for JwkId {
    fn pure_bytes(self) -> PureBytes {
        PureBytes(::bcs::to_bytes(&self).expect("bcs serialization failed"))
    }
}
pub trait ArgumentJwkId: PTBArgument {
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
impl ArgumentJwkId for JwkId {}
impl ArgumentJwkId for Argument {}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ActiveJwk {
    pub jwk_id: JwkId,
    pub jwk: JWK,
    pub epoch: u64,
}
impl MoveType for ActiveJwk {
    type Package = super::Package;
    const MODULE: &'static str = "authenticator_state";
    const NAME: &'static str = "ActiveJwk";
}
impl MoveArg for ActiveJwk {
    fn pure_bytes(self) -> PureBytes {
        PureBytes(::bcs::to_bytes(&self).expect("bcs serialization failed"))
    }
}
pub trait ArgumentActiveJwk: PTBArgument {
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
impl ArgumentActiveJwk for ActiveJwk {}
impl ArgumentActiveJwk for Argument {}
/// Read-only handle bound to a runtime package address. Use
/// via `super::Package::at(addr).<module>()` and chain
/// `<type>_tag()` to build a [`TypeTag`] without spinning up
/// a `PtbBuilder`.
pub struct ModuleAt {
    pub(crate) package: Address,
}
impl ModuleAt {
    pub fn authenticatorstate_tag(&self) -> TypeTag {
        <AuthenticatorState as MoveType>::type_tag_at(self.package)
    }
    pub fn authenticatorstateinner_tag(&self) -> TypeTag {
        <AuthenticatorStateInner as MoveType>::type_tag_at(self.package)
    }
    pub fn jwk_tag(&self) -> TypeTag {
        <JWK as MoveType>::type_tag_at(self.package)
    }
    pub fn jwkid_tag(&self) -> TypeTag {
        <JwkId as MoveType>::type_tag_at(self.package)
    }
    pub fn activejwk_tag(&self) -> TypeTag {
        <ActiveJwk as MoveType>::type_tag_at(self.package)
    }
}
