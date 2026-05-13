#![allow(unused_imports, non_snake_case, non_upper_case_globals)]
//! A storable handler for Balances in general. Is used in the `Coin`
//! module to allow balance operations and can be used to implement
//! custom coins with `Supply` and `Balance`s.
use move_bindgen_runtime::*;
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;
/// For when trying to destroy a non-zero balance.
pub const ENonZero: u64 = 0u64;
/// For when an overflow is happening on Supply operations.
pub const EOverflow: u64 = 1u64;
/// For when trying to withdraw more than there is.
pub const ENotEnough: u64 = 2u64;
/// Sender is not @0x0 the system address.
pub const ENotSystemAddress: u64 = 3u64;
/// Epoch is not 0 (the genesis epoch).
pub const ENotGenesisEpoch: u64 = 4u64;
/// A Supply of T. Used for minting and burning.
/// Wrapped into a `TreasuryCap` in the `Coin` module.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Supply<T0: MoveType> {
    pub value: u64,
    #[serde(skip)]
    pub _phantom_t0: PhantomData<T0>,
}
impl<T0: MoveType> MoveType for Supply<T0> {
    type Package = super::Package;
    const MODULE: &'static str = "balance";
    const NAME: &'static str = "Supply";
    fn type_params(addrs: &impl PackageAddrs) -> Vec<TypeTag> {
        vec![< T0 as MoveType > ::type_tag(addrs)]
    }
}
impl<T0: MoveType + ::serde::Serialize> MoveArg for Supply<T0> {
    fn pure_bytes(self) -> PureBytes {
        PureBytes(::bcs::to_bytes(&self).expect("bcs serialization failed"))
    }
}
pub trait ArgumentSupply<T0: MoveType>: PTBArgument {
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
impl<T0: MoveType + ::serde::Serialize> ArgumentSupply<T0> for Supply<T0> {}
impl<T0: MoveType> ArgumentSupply<T0> for Argument {}
/// Storable balance - an inner struct of a Coin type.
/// Can be used to store coins which don't need the key ability.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Balance<T0: MoveType> {
    pub value: u64,
    #[serde(skip)]
    pub _phantom_t0: PhantomData<T0>,
}
impl<T0: MoveType> MoveType for Balance<T0> {
    type Package = super::Package;
    const MODULE: &'static str = "balance";
    const NAME: &'static str = "Balance";
    fn type_params(addrs: &impl PackageAddrs) -> Vec<TypeTag> {
        vec![< T0 as MoveType > ::type_tag(addrs)]
    }
}
impl<T0: MoveType + ::serde::Serialize> MoveArg for Balance<T0> {
    fn pure_bytes(self) -> PureBytes {
        PureBytes(::bcs::to_bytes(&self).expect("bcs serialization failed"))
    }
}
pub trait ArgumentBalance<T0: MoveType>: PTBArgument {
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
impl<T0: MoveType + ::serde::Serialize> ArgumentBalance<T0> for Balance<T0> {}
impl<T0: MoveType> ArgumentBalance<T0> for Argument {}
/// Get the amount stored in a `Balance`.
///
/// Returns: `u64`.
pub async fn value<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentBalance<T0>,
) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "balance",
        "value",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![a0],
    )
}
/// Get the `Supply` value.
///
/// Returns: `u64`.
pub async fn supply_value<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentSupply<T0>,
) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "balance",
        "supply_value",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![a0],
    )
}
/// Create a new supply for type T.
///
/// Returns: `0x2::balance::Supply<T0>`.
pub async fn create_supply<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentObject<()>,
) -> Argument {
    let a0 = arg0.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "balance",
        "create_supply",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![a0],
    )
}
/// Increase supply by `value` and create a new `Balance<T>` with this value.
///
/// Returns: `0x2::balance::Balance<T0>`.
pub async fn increase_supply<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentSupply<T0>,
    arg1: impl PureU64,
) -> Argument {
    let a0 = arg0.into_argument_mut(b).await;
    let a1 = arg1.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "balance",
        "increase_supply",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![a0, a1],
    )
}
/// Burn a Balance<T> and decrease Supply<T>.
///
/// Returns: `u64`.
pub async fn decrease_supply<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentSupply<T0>,
    arg1: impl ArgumentBalance<T0>,
) -> Argument {
    let a0 = arg0.into_argument_mut(b).await;
    let a1 = arg1.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "balance",
        "decrease_supply",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![a0, a1],
    )
}
/// Create a zero `Balance` for type `T`.
///
/// Returns: `0x2::balance::Balance<T0>`.
pub async fn zero<T0: MoveType>(b: &mut PtbBuilder) -> Argument {
    b.move_call(
        b.package_id::<super::Package>(),
        "balance",
        "zero",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![],
    )
}
/// Join two balances together.
///
/// Returns: `u64`.
pub async fn join<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentBalance<T0>,
    arg1: impl ArgumentBalance<T0>,
) -> Argument {
    let a0 = arg0.into_argument_mut(b).await;
    let a1 = arg1.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "balance",
        "join",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![a0, a1],
    )
}
/// Split a `Balance` and take a sub balance from it.
///
/// Returns: `0x2::balance::Balance<T0>`.
pub async fn split<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentBalance<T0>,
    arg1: impl PureU64,
) -> Argument {
    let a0 = arg0.into_argument_mut(b).await;
    let a1 = arg1.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "balance",
        "split",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![a0, a1],
    )
}
/// Withdraw all balance. After this the remaining balance must be 0.
///
/// Returns: `0x2::balance::Balance<T0>`.
pub async fn withdraw_all<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentBalance<T0>,
) -> Argument {
    let a0 = arg0.into_argument_mut(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "balance",
        "withdraw_all",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![a0],
    )
}
/// Destroy a zero `Balance`.
pub async fn destroy_zero<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentBalance<T0>,
) {
    let a0 = arg0.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "balance",
        "destroy_zero",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![a0],
    );
}
