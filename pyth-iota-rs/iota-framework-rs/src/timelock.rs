#![allow(unused_imports, non_snake_case, non_upper_case_globals)]
//! A timelock implementation.
use move_bindgen_runtime::*;
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;
/// The lock has not expired yet.
pub const ENotExpiredYet: u64 = 1u64;
/// For when trying to join two time-locked balances with different expiration time.
pub const EDifferentExpirationTime: u64 = 2u64;
/// For when trying to join two time-locked balances with different labels.
pub const EDifferentLabels: u64 = 3u64;
/// `TimeLock` struct that holds a locked object.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TimeLock<T0: MoveType> {
    pub id: UID,
    /// The locked object.
    pub locked: T0,
    /// This is the epoch time stamp of when the lock expires.
    pub expiration_timestamp_ms: u64,
    /// Timelock related label.
    pub label: Option<String>,
}
impl<T0: MoveType> MoveType for TimeLock<T0> {
    type Package = super::Package;
    const MODULE: &'static str = "timelock";
    const NAME: &'static str = "TimeLock";
    fn type_params(addrs: &impl PackageAddrs) -> Vec<TypeTag> {
        vec![< T0 as MoveType > ::type_tag(addrs)]
    }
}
pub trait ArgumentTimeLock<T0: MoveType>: PTBArgument {
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
impl<T0: MoveType> ArgumentTimeLock<T0> for Argument {}
impl<T0: MoveType> ArgumentTimeLock<T0> for ObjectId {
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
impl<T0: MoveType> ArgumentTimeLock<T0> for ObjectReference {}
impl<T0: MoveType> ArgumentTimeLock<T0> for Shared<ObjectId> {}
impl<T0: MoveType> ArgumentTimeLock<T0> for SharedMut<ObjectId> {}
impl<T0: MoveType> ArgumentTimeLock<T0> for Receiving<ObjectId> {}
/// Function to lock an object till a unix timestamp in milliseconds.
///
/// Returns: `0x2::timelock::TimeLock<T0>`.
pub async fn lock<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentObject<()>,
    arg1: impl PureU64,
) -> Argument {
    let a0 = arg0.into_argument(b).await;
    let a1 = arg1.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "timelock",
        "lock",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![a0, a1],
    )
}
/// Function to lock a labeled object till a unix timestamp in milliseconds.
///
/// Returns: `0x2::timelock::TimeLock<T0>`.
pub async fn lock_with_label<T0: MoveType, T1: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl super::labeler::ArgumentLabelerCap<T1>,
    arg1: impl ArgumentObject<()>,
    arg2: impl PureU64,
) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    let a1 = arg1.into_argument(b).await;
    let a2 = arg2.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "timelock",
        "lock_with_label",
        vec![< T0 as MoveType > ::type_tag(b), < T1 as MoveType > ::type_tag(b)],
        vec![a0, a1, a2],
    )
}
/// Function to lock an object `obj` until `expiration_timestamp_ms` and transfer it to address `to`.
/// Since `Timelock<T>` does not support public transfer, use this function to lock an object to an address.
pub async fn lock_and_transfer<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentObject<()>,
    arg1: impl PureAddress,
    arg2: impl PureU64,
) {
    let a0 = arg0.into_argument(b).await;
    let a1 = arg1.into_argument(b).await;
    let a2 = arg2.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "timelock",
        "lock_and_transfer",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![a0, a1, a2],
    );
}
/// Function to lock a labeled object `obj` until `expiration_timestamp_ms` and transfer it to address `to`.
/// Since `Timelock<T>` does not support public transfer, use this function to lock a labeled object to an address.
pub async fn lock_with_label_and_transfer<T0: MoveType, T1: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl super::labeler::ArgumentLabelerCap<T1>,
    arg1: impl ArgumentObject<()>,
    arg2: impl PureAddress,
    arg3: impl PureU64,
) {
    let a0 = arg0.into_argument_ref(b).await;
    let a1 = arg1.into_argument(b).await;
    let a2 = arg2.into_argument(b).await;
    let a3 = arg3.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "timelock",
        "lock_with_label_and_transfer",
        vec![< T0 as MoveType > ::type_tag(b), < T1 as MoveType > ::type_tag(b)],
        vec![a0, a1, a2, a3],
    );
}
/// Function to unlock the object from a `TimeLock` based on the epoch start time.
///
/// Returns: `T0`.
pub async fn unlock<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentTimeLock<T0>,
) -> Argument {
    let a0 = arg0.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "timelock",
        "unlock",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![a0],
    )
}
/// Function to unlock the object from a `TimeLock` based on the `Clock` object.
///
/// Returns: `T0`.
pub async fn unlock_with_clock<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentTimeLock<T0>,
    arg1: impl super::clock::ArgumentClock,
) -> Argument {
    let a0 = arg0.into_argument(b).await;
    let a1 = arg1.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "timelock",
        "unlock_with_clock",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![a0, a1],
    )
}
/// Join two `TimeLock<Balance<T>>` together.
pub async fn join<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentTimeLock<super::balance::Balance<T0>>,
    arg1: impl ArgumentTimeLock<super::balance::Balance<T0>>,
) {
    let a0 = arg0.into_argument_mut(b).await;
    let a1 = arg1.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "timelock",
        "join",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![a0, a1],
    );
}
/// Join everything in `others` with `self`.
pub async fn join_vec<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentTimeLock<super::balance::Balance<T0>>,
    arg1: impl PureVec<TimeLock<super::balance::Balance<T0>>>,
) {
    let a0 = arg0.into_argument_mut(b).await;
    let a1 = arg1.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "timelock",
        "join_vec",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![a0, a1],
    );
}
/// Split a `TimeLock<Balance<T>>` and take a sub balance from it.
///
/// Returns: `0x2::timelock::TimeLock<0x2::balance::Balance<T0>>`.
pub async fn split<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentTimeLock<super::balance::Balance<T0>>,
    arg1: impl PureU64,
) -> Argument {
    let a0 = arg0.into_argument_mut(b).await;
    let a1 = arg1.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "timelock",
        "split",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![a0, a1],
    )
}
/// Split the given `TimeLock<Balance<T>>` into two parts, one with principal `value`,
/// and transfer the newly split part to the sender address.
pub async fn split_balance<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentTimeLock<super::balance::Balance<T0>>,
    arg1: impl PureU64,
) {
    let a0 = arg0.into_argument_mut(b).await;
    let a1 = arg1.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "timelock",
        "split_balance",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![a0, a1],
    );
}
/// A utility function to transfer a `TimeLock` to the sender.
pub async fn transfer_to_sender<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentTimeLock<T0>,
) {
    let a0 = arg0.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "timelock",
        "transfer_to_sender",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![a0],
    );
}
/// A utility function to pack a `TimeLock` that can be invoked only by a system package.
///
/// Returns: `0x2::timelock::TimeLock<T0>`.
pub async fn system_pack<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl super::system_admin_cap::ArgumentIotaSystemAdminCap,
    arg1: impl ArgumentObject<()>,
    arg2: impl PureU64,
    arg3: impl PureOption<String>,
) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    let a1 = arg1.into_argument(b).await;
    let a2 = arg2.into_argument(b).await;
    let a3 = arg3.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "timelock",
        "system_pack",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![a0, a1, a2, a3],
    )
}
/// An utility function to unpack a `TimeLock` that can be invoked only by a system package.
///
/// Returns: `(T0, u64, 0x1::option::Option<0x1::string::String>)`.
pub async fn system_unpack<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl super::system_admin_cap::ArgumentIotaSystemAdminCap,
    arg1: impl ArgumentTimeLock<T0>,
) -> (Argument, Argument, Argument) {
    let a0 = arg0.into_argument_ref(b).await;
    let a1 = arg1.into_argument(b).await;
    let __r = b
        .move_call_n(
            b.package_id::<super::Package>(),
            "timelock",
            "system_unpack",
            vec![< T0 as MoveType > ::type_tag(b)],
            vec![a0, a1],
            3u16,
        );
    (__r[0], __r[1], __r[2])
}
/// Return a fully qualified type name with the original package IDs
/// that is used as type related a label value.
///
/// Returns: `0x1::string::String`.
pub async fn type_name<T0: MoveType>(b: &mut PtbBuilder) -> Argument {
    b.move_call(
        b.package_id::<super::Package>(),
        "timelock",
        "type_name",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![],
    )
}
/// Function to get the expiration timestamp of a `TimeLock`.
///
/// Returns: `u64`.
pub async fn expiration_timestamp_ms<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentTimeLock<T0>,
) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "timelock",
        "expiration_timestamp_ms",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![a0],
    )
}
/// Function to check if a `TimeLock` is locked based on the epoch start time.
///
/// Returns: `bool`.
pub async fn is_locked<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentTimeLock<T0>,
) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "timelock",
        "is_locked",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![a0],
    )
}
/// Function to get the remaining time of a `TimeLock` based on the epoch start time.
/// Returns 0 if the lock has expired.
///
/// Returns: `u64`.
pub async fn remaining_time<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentTimeLock<T0>,
) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "timelock",
        "remaining_time",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![a0],
    )
}
/// Function to check if a `TimeLock` is locked based on the `Clock` object.
///
/// Returns: `bool`.
pub async fn is_locked_with_clock<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentTimeLock<T0>,
    arg1: impl super::clock::ArgumentClock,
) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    let a1 = arg1.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "timelock",
        "is_locked_with_clock",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![a0, a1],
    )
}
/// Function to get the remaining time of a `TimeLock` based on the `Clock` object.
/// Returns 0 if the lock has expired.
///
/// Returns: `u64`.
pub async fn remaining_time_with_clock<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentTimeLock<T0>,
    arg1: impl super::clock::ArgumentClock,
) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    let a1 = arg1.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "timelock",
        "remaining_time_with_clock",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![a0, a1],
    )
}
/// Function to get the locked object of a `TimeLock`.
///
/// Returns: `&T0`.
pub async fn locked<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentTimeLock<T0>,
) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "timelock",
        "locked",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![a0],
    )
}
/// Function to get the label of a `TimeLock`.
///
/// Returns: `0x1::option::Option<0x1::string::String>`.
pub async fn label<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentTimeLock<T0>,
) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "timelock",
        "label",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![a0],
    )
}
/// Check if a `TimeLock` is labeled with the type `L`.
///
/// Returns: `bool`.
pub async fn is_labeled_with<T0: MoveType, T1: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentTimeLock<T0>,
) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "timelock",
        "is_labeled_with",
        vec![< T0 as MoveType > ::type_tag(b), < T1 as MoveType > ::type_tag(b)],
        vec![a0],
    )
}
