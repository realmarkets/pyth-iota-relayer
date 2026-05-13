#![allow(unused_imports, non_snake_case, non_upper_case_globals)]
//! APIs for accessing time from move calls, via the `Clock`: a unique
//! shared object that is created at 0x6 during genesis.
use move_bindgen_runtime::*;
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;
/// Sender is not @0x0 the system address.
pub const ENotSystemAddress: u64 = 0u64;
/// Singleton shared object that exposes time to Move calls.  This
/// object is found at address 0x6, and can only be read (accessed
/// via an immutable reference) by entry functions.
///
/// Entry Functions that attempt to accept `Clock` by mutable
/// reference or value will fail to verify, and honest validators
/// will not sign or execute transactions that use `Clock` as an
/// input parameter, unless it is passed by immutable reference.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Clock {
    pub id: UID,
    /// The clock's timestamp, which is set automatically by a
    /// system transaction every time consensus commits a
    /// schedule, or by `iota::clock::increment_for_testing` during
    /// testing.
    pub timestamp_ms: u64,
}
impl MoveType for Clock {
    type Package = super::Package;
    const MODULE: &'static str = "clock";
    const NAME: &'static str = "Clock";
}
pub trait ArgumentClock: PTBArgument {
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
impl ArgumentClock for Argument {}
impl ArgumentClock for ObjectId {
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
impl ArgumentClock for ObjectReference {}
impl ArgumentClock for Shared<ObjectId> {}
impl ArgumentClock for SharedMut<ObjectId> {}
impl ArgumentClock for Receiving<ObjectId> {}
/// The `clock`'s current timestamp as a running total of
/// milliseconds since an arbitrary point in the past.
///
/// Returns: `u64`.
pub async fn timestamp_ms(b: &mut PtbBuilder, arg0: impl ArgumentClock) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "clock",
        "timestamp_ms",
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
    pub fn clock_tag(&self) -> TypeTag {
        <Clock as MoveType>::type_tag_at(self.package)
    }
}
