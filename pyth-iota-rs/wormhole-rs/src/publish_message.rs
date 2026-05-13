#![allow(unused_imports, non_snake_case, non_upper_case_globals)]
//! This module implements two methods: `prepare_message` and `publish_message`,
//! which are to be executed in a transaction block in this order.
//!
//! `prepare_message` allows a contract to pack Wormhole message info (payload
//! that has meaning to an integrator plus nonce) in preparation to publish a
//! `WormholeMessage` event via `publish_message`. Only the owner of an
//! `EmitterCap` has the capability of creating this `MessageTicket`.
//!
//! `publish_message` unpacks the `MessageTicket` and emits a
//! `WormholeMessage` with this message info and timestamp. This event is
//! observed by the Guardian network.
//!
//! The purpose of splitting this message publishing into two steps is in case
//! Wormhole needs to be upgraded and there is a breaking change for this
//! module, an integrator would not be left broken. It is discouraged to put
//! `publish_message` in an integrator's package logic. Otherwise, this
//! integrator needs to be prepared to upgrade his contract to handle the latest
//! version of `publish_message`.
//!
//! Instead, an integtrator is encouraged to execute a transaction block, which
//! executes `publish_message` using the latest Wormhole package ID and to
//! implement `prepare_message` in his contract to produce `MessageTicket`,
//! which `publish_message` consumes.
use move_bindgen_runtime::*;
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;
/// This type is emitted via `iota::event` module. Guardians pick up this
/// observation and attest to its existence.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WormholeMessage {
    /// `EmitterCap` object ID.
    pub sender: ID,
    /// From `EmitterCap`.
    pub sequence: u64,
    /// A.K.A. Batch ID.
    pub nonce: u32,
    /// Arbitrary message data relevant to integrator.
    pub payload: Vec<u8>,
    /// This will always be `0`.
    pub consistency_level: u8,
    /// `Clock` timestamp.
    pub timestamp: u64,
}
impl MoveType for WormholeMessage {
    type Package = super::Package;
    const MODULE: &'static str = "publish_message";
    const NAME: &'static str = "WormholeMessage";
}
impl MoveArg for WormholeMessage {
    fn pure_bytes(self) -> PureBytes {
        PureBytes(::bcs::to_bytes(&self).expect("bcs serialization failed"))
    }
}
pub trait ArgumentWormholeMessage: PTBArgument {
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
impl ArgumentWormholeMessage for WormholeMessage {}
impl ArgumentWormholeMessage for Argument {}
/// This type represents Wormhole message data. The sender is the object ID
/// of an `EmitterCap`, who acts as the capability of creating this type.
/// The only way to destroy this type is calling `publish_message` with
/// a fee to emit a `WormholeMessage` with the unpacked members of this
/// struct.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MessageTicket {
    /// `EmitterCap` object ID.
    pub sender: ID,
    /// From `EmitterCap`.
    pub sequence: u64,
    /// A.K.A. Batch ID.
    pub nonce: u32,
    /// Arbitrary message data relevant to integrator.
    pub payload: Vec<u8>,
}
impl MoveType for MessageTicket {
    type Package = super::Package;
    const MODULE: &'static str = "publish_message";
    const NAME: &'static str = "MessageTicket";
}
impl MoveArg for MessageTicket {
    fn pure_bytes(self) -> PureBytes {
        PureBytes(::bcs::to_bytes(&self).expect("bcs serialization failed"))
    }
}
pub trait ArgumentMessageTicket: PTBArgument {
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
impl ArgumentMessageTicket for MessageTicket {}
impl ArgumentMessageTicket for Argument {}
/// `prepare_message` constructs Wormhole message parameters. An
/// `EmitterCap` provides the capability to send an arbitrary payload.
///
/// NOTE: Integrators of Wormhole should be calling only this method from
/// their contracts. This method is not guarded by version control (thus not
/// requiring a reference to the Wormhole `State` object), so it is intended
/// to work for any package version.
///
/// Returns: `0xff00000000000002::publish_message::MessageTicket`.
pub async fn prepare_message(
    b: &mut PtbBuilder,
    arg0: impl super::emitter::ArgumentEmitterCap,
    arg1: impl PureU32,
    arg2: impl PureVec<u8>,
) -> Argument {
    let a0 = arg0.into_argument_mut(b).await;
    let a1 = arg1.into_argument(b).await;
    let a2 = arg2.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "publish_message",
        "prepare_message",
        Vec::new(),
        vec![a0, a1, a2],
    )
}
/// `publish_message` emits a message as a Iota event. This method uses the
/// input `EmitterCap` as the registered sender of the
/// `WormholeMessage`. It also produces a new sequence for this emitter.
///
/// NOTE: This method is guarded by a minimum build version check. This
/// method could break backward compatibility on an upgrade.
///
/// It is important for integrators to refrain from calling this method
/// within their contracts. This method is meant to be called in a
/// transaction block after receiving a `MessageTicket` from calling
/// `prepare_message` within a contract. If in a circumstance where this
/// module has a breaking change in an upgrade, `prepare_message` will not
/// be affected by this change.
///
/// See `prepare_message` for more details.
///
/// Returns: `u64`.
pub async fn publish_message(
    b: &mut PtbBuilder,
    arg0: impl super::state::ArgumentState,
    arg1: impl ::iota_framework_rs::coin::ArgumentCoin<::iota_framework_rs::iota::IOTA>,
    arg2: impl ArgumentMessageTicket,
    arg3: impl ::iota_framework_rs::clock::ArgumentClock,
) -> Argument {
    let a0 = arg0.into_argument_mut(b).await;
    let a1 = arg1.into_argument(b).await;
    let a2 = arg2.into_argument(b).await;
    let a3 = arg3.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "publish_message",
        "publish_message",
        Vec::new(),
        vec![a0, a1, a2, a3],
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
    pub fn wormholemessage_tag(&self) -> TypeTag {
        <WormholeMessage as MoveType>::type_tag_at(self.package)
    }
    pub fn messageticket_tag(&self) -> TypeTag {
        <MessageTicket as MoveType>::type_tag_at(self.package)
    }
}
