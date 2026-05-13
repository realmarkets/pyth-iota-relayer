#![allow(unused_imports, non_snake_case, non_upper_case_globals)]
//! This module implements a capability (`EmitterCap`), which allows one to send
//! Wormhole messages. Its external address is determined by the capability's
//! `id`, which is a 32-byte vector.
use move_bindgen_runtime::*;
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;
/// Event reflecting when `new` is called.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EmitterCreated {
    pub emitter_cap: ID,
}
impl MoveType for EmitterCreated {
    type Package = super::Package;
    const MODULE: &'static str = "emitter";
    const NAME: &'static str = "EmitterCreated";
}
impl MoveArg for EmitterCreated {
    fn pure_bytes(self) -> PureBytes {
        PureBytes(::bcs::to_bytes(&self).expect("bcs serialization failed"))
    }
}
pub trait ArgumentEmitterCreated: PTBArgument {
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
impl ArgumentEmitterCreated for EmitterCreated {}
impl ArgumentEmitterCreated for Argument {}
/// Event reflecting when `destroy` is called.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EmitterDestroyed {
    pub emitter_cap: ID,
}
impl MoveType for EmitterDestroyed {
    type Package = super::Package;
    const MODULE: &'static str = "emitter";
    const NAME: &'static str = "EmitterDestroyed";
}
impl MoveArg for EmitterDestroyed {
    fn pure_bytes(self) -> PureBytes {
        PureBytes(::bcs::to_bytes(&self).expect("bcs serialization failed"))
    }
}
pub trait ArgumentEmitterDestroyed: PTBArgument {
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
impl ArgumentEmitterDestroyed for EmitterDestroyed {}
impl ArgumentEmitterDestroyed for Argument {}
/// `EmitterCap` is a Iota object that gives a user or smart contract the
/// capability to send Wormhole messages. For every Wormhole message
/// emitted, a unique `sequence` is used.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EmitterCap {
    pub id: UID,
    /// Sequence number of the next wormhole message.
    pub sequence: u64,
}
impl MoveType for EmitterCap {
    type Package = super::Package;
    const MODULE: &'static str = "emitter";
    const NAME: &'static str = "EmitterCap";
}
pub trait ArgumentEmitterCap: PTBArgument {
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
impl ArgumentEmitterCap for Argument {}
impl ArgumentEmitterCap for ObjectId {
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
impl ArgumentEmitterCap for ObjectReference {}
impl ArgumentEmitterCap for Shared<ObjectId> {}
impl ArgumentEmitterCap for SharedMut<ObjectId> {}
impl ArgumentEmitterCap for Receiving<ObjectId> {}
/// Generate a new `EmitterCap`.
///
/// Returns: `0xff00000000000002::emitter::EmitterCap`.
pub async fn new(
    b: &mut PtbBuilder,
    arg0: impl super::state::ArgumentState,
) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    b.move_call(b.package_id::<super::Package>(), "emitter", "new", Vec::new(), vec![a0])
}
/// Returns current sequence (which will be used in the next Wormhole
/// message emitted).
///
/// Returns: `u64`.
pub async fn sequence(b: &mut PtbBuilder, arg0: impl ArgumentEmitterCap) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "emitter",
        "sequence",
        Vec::new(),
        vec![a0],
    )
}
/// Destroys an `EmitterCap`.
///
/// Note that this operation removes the ability to send messages using the
/// emitter id, and is irreversible.
pub async fn destroy(
    b: &mut PtbBuilder,
    arg0: impl super::state::ArgumentState,
    arg1: impl ArgumentEmitterCap,
) {
    let a0 = arg0.into_argument_ref(b).await;
    let a1 = arg1.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "emitter",
        "destroy",
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
    pub fn emittercreated_tag(&self) -> TypeTag {
        <EmitterCreated as MoveType>::type_tag_at(self.package)
    }
    pub fn emitterdestroyed_tag(&self) -> TypeTag {
        <EmitterDestroyed as MoveType>::type_tag_at(self.package)
    }
    pub fn emittercap_tag(&self) -> TypeTag {
        <EmitterCap as MoveType>::type_tag_at(self.package)
    }
}
