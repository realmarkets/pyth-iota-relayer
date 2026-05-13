#![allow(unused_imports, non_snake_case, non_upper_case_globals)]
//! Defines the `Coin` type - platform wide representation of fungible
//! tokens and coins. `Coin` can be described as a secure wrapper around
//! `Balance` type.
use move_bindgen_runtime::*;
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;
/// A type passed to create_supply is not a one-time witness.
pub const EBadWitness: u64 = 0u64;
/// Invalid arguments are passed to a function.
pub const EInvalidArg: u64 = 1u64;
/// Trying to split a coin more times than its balance allows.
pub const ENotEnough: u64 = 2u64;
pub const EGlobalPauseNotAllowed: u64 = 3u64;
/// A coin of type `T` worth `value`. Transferable and storable
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Coin<T0: MoveType> {
    pub id: UID,
    pub balance: super::balance::Balance<T0>,
    #[serde(skip)]
    pub _phantom_t0: PhantomData<T0>,
}
impl<T0: MoveType> MoveType for Coin<T0> {
    type Package = super::Package;
    const MODULE: &'static str = "coin";
    const NAME: &'static str = "Coin";
    fn type_params(addrs: &impl PackageAddrs) -> Vec<TypeTag> {
        vec![< T0 as MoveType > ::type_tag(addrs)]
    }
}
pub trait ArgumentCoin<T0: MoveType>: PTBArgument {
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
impl<T0: MoveType> ArgumentCoin<T0> for Argument {}
impl<T0: MoveType> ArgumentCoin<T0> for ObjectId {
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
impl<T0: MoveType> ArgumentCoin<T0> for ObjectReference {}
impl<T0: MoveType> ArgumentCoin<T0> for Shared<ObjectId> {}
impl<T0: MoveType> ArgumentCoin<T0> for SharedMut<ObjectId> {}
impl<T0: MoveType> ArgumentCoin<T0> for Receiving<ObjectId> {}
/// Each Coin type T created through `create_currency` function will have a
/// unique instance of CoinMetadata<T> that stores the metadata for this coin type.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CoinMetadata<T0: MoveType> {
    pub id: UID,
    /// Number of decimal places the coin uses.
    /// A coin with `value ` N and `decimals` D should be shown as N / 10^D
    /// E.g., a coin with `value` 7002 and decimals 3 should be displayed as 7.002
    /// This is metadata for display usage only.
    pub decimals: u8,
    /// Name for the token
    pub name: String,
    /// Symbol for the token
    pub symbol: String,
    /// Description of the token
    pub description: String,
    /// URL for the token logo
    pub icon_url: Option<super::url::Url>,
    #[serde(skip)]
    pub _phantom_t0: PhantomData<T0>,
}
impl<T0: MoveType> MoveType for CoinMetadata<T0> {
    type Package = super::Package;
    const MODULE: &'static str = "coin";
    const NAME: &'static str = "CoinMetadata";
    fn type_params(addrs: &impl PackageAddrs) -> Vec<TypeTag> {
        vec![< T0 as MoveType > ::type_tag(addrs)]
    }
}
pub trait ArgumentCoinMetadata<T0: MoveType>: PTBArgument {
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
impl<T0: MoveType> ArgumentCoinMetadata<T0> for Argument {}
impl<T0: MoveType> ArgumentCoinMetadata<T0> for ObjectId {
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
impl<T0: MoveType> ArgumentCoinMetadata<T0> for ObjectReference {}
impl<T0: MoveType> ArgumentCoinMetadata<T0> for Shared<ObjectId> {}
impl<T0: MoveType> ArgumentCoinMetadata<T0> for SharedMut<ObjectId> {}
impl<T0: MoveType> ArgumentCoinMetadata<T0> for Receiving<ObjectId> {}
/// Similar to CoinMetadata, but created only for regulated coins that use the DenyList.
/// This object is always immutable.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RegulatedCoinMetadata<T0: MoveType> {
    pub id: UID,
    /// The ID of the coin's CoinMetadata object.
    pub coin_metadata_object: ID,
    /// The ID of the coin's DenyCap object.
    pub deny_cap_object: ID,
    #[serde(skip)]
    pub _phantom_t0: PhantomData<T0>,
}
impl<T0: MoveType> MoveType for RegulatedCoinMetadata<T0> {
    type Package = super::Package;
    const MODULE: &'static str = "coin";
    const NAME: &'static str = "RegulatedCoinMetadata";
    fn type_params(addrs: &impl PackageAddrs) -> Vec<TypeTag> {
        vec![< T0 as MoveType > ::type_tag(addrs)]
    }
}
pub trait ArgumentRegulatedCoinMetadata<T0: MoveType>: PTBArgument {
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
impl<T0: MoveType> ArgumentRegulatedCoinMetadata<T0> for Argument {}
impl<T0: MoveType> ArgumentRegulatedCoinMetadata<T0> for ObjectId {
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
impl<T0: MoveType> ArgumentRegulatedCoinMetadata<T0> for ObjectReference {}
impl<T0: MoveType> ArgumentRegulatedCoinMetadata<T0> for Shared<ObjectId> {}
impl<T0: MoveType> ArgumentRegulatedCoinMetadata<T0> for SharedMut<ObjectId> {}
impl<T0: MoveType> ArgumentRegulatedCoinMetadata<T0> for Receiving<ObjectId> {}
/// Capability allowing the bearer to mint and burn
/// coins of type `T`. Transferable
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TreasuryCap<T0: MoveType> {
    pub id: UID,
    pub total_supply: super::balance::Supply<T0>,
    #[serde(skip)]
    pub _phantom_t0: PhantomData<T0>,
}
impl<T0: MoveType> MoveType for TreasuryCap<T0> {
    type Package = super::Package;
    const MODULE: &'static str = "coin";
    const NAME: &'static str = "TreasuryCap";
    fn type_params(addrs: &impl PackageAddrs) -> Vec<TypeTag> {
        vec![< T0 as MoveType > ::type_tag(addrs)]
    }
}
pub trait ArgumentTreasuryCap<T0: MoveType>: PTBArgument {
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
impl<T0: MoveType> ArgumentTreasuryCap<T0> for Argument {}
impl<T0: MoveType> ArgumentTreasuryCap<T0> for ObjectId {
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
impl<T0: MoveType> ArgumentTreasuryCap<T0> for ObjectReference {}
impl<T0: MoveType> ArgumentTreasuryCap<T0> for Shared<ObjectId> {}
impl<T0: MoveType> ArgumentTreasuryCap<T0> for SharedMut<ObjectId> {}
impl<T0: MoveType> ArgumentTreasuryCap<T0> for Receiving<ObjectId> {}
/// Capability allowing the bearer to deny addresses from using the currency's coins--
/// immediately preventing those addresses from interacting with the coin as an input to a
/// transaction and at the start of the next preventing them from receiving the coin.
/// If `allow_global_pause` is true, the bearer can enable a global pause that behaves as if
/// all addresses were added to the deny list.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DenyCapV1<T0: MoveType> {
    pub id: UID,
    pub allow_global_pause: bool,
    #[serde(skip)]
    pub _phantom_t0: PhantomData<T0>,
}
impl<T0: MoveType> MoveType for DenyCapV1<T0> {
    type Package = super::Package;
    const MODULE: &'static str = "coin";
    const NAME: &'static str = "DenyCapV1";
    fn type_params(addrs: &impl PackageAddrs) -> Vec<TypeTag> {
        vec![< T0 as MoveType > ::type_tag(addrs)]
    }
}
pub trait ArgumentDenyCapV1<T0: MoveType>: PTBArgument {
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
impl<T0: MoveType> ArgumentDenyCapV1<T0> for Argument {}
impl<T0: MoveType> ArgumentDenyCapV1<T0> for ObjectId {
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
impl<T0: MoveType> ArgumentDenyCapV1<T0> for ObjectReference {}
impl<T0: MoveType> ArgumentDenyCapV1<T0> for Shared<ObjectId> {}
impl<T0: MoveType> ArgumentDenyCapV1<T0> for SharedMut<ObjectId> {}
impl<T0: MoveType> ArgumentDenyCapV1<T0> for Receiving<ObjectId> {}
/// Return the total number of `T`'s in circulation.
///
/// Returns: `u64`.
pub async fn total_supply<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentTreasuryCap<T0>,
) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "coin",
        "total_supply",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![a0],
    )
}
/// Unwrap `TreasuryCap` getting the `Supply`.
///
/// Operation is irreversible. Supply cannot be converted into a `TreasuryCap` due
/// to different security guarantees (TreasuryCap can be created only once for a type)
///
/// Returns: `0x2::balance::Supply<T0>`.
pub async fn treasury_into_supply<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentTreasuryCap<T0>,
) -> Argument {
    let a0 = arg0.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "coin",
        "treasury_into_supply",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![a0],
    )
}
/// Get immutable reference to the treasury's `Supply`.
///
/// Returns: `&0x2::balance::Supply<T0>`.
pub async fn supply_immut<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentTreasuryCap<T0>,
) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "coin",
        "supply_immut",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![a0],
    )
}
/// Get mutable reference to the treasury's `Supply`.
///
/// Returns: `&mut 0x2::balance::Supply<T0>`.
pub async fn supply_mut<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentTreasuryCap<T0>,
) -> Argument {
    let a0 = arg0.into_argument_mut(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "coin",
        "supply_mut",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![a0],
    )
}
/// Public getter for the coin's value
///
/// Returns: `u64`.
pub async fn value<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentCoin<T0>,
) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "coin",
        "value",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![a0],
    )
}
/// Get immutable reference to the balance of a coin.
///
/// Returns: `&0x2::balance::Balance<T0>`.
pub async fn balance<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentCoin<T0>,
) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "coin",
        "balance",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![a0],
    )
}
/// Get a mutable reference to the balance of a coin.
///
/// Returns: `&mut 0x2::balance::Balance<T0>`.
pub async fn balance_mut<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentCoin<T0>,
) -> Argument {
    let a0 = arg0.into_argument_mut(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "coin",
        "balance_mut",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![a0],
    )
}
/// Wrap a balance into a Coin to make it transferable.
///
/// Returns: `0x2::coin::Coin<T0>`.
pub async fn from_balance<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl super::balance::ArgumentBalance<T0>,
) -> Argument {
    let a0 = arg0.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "coin",
        "from_balance",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![a0],
    )
}
/// Destruct a Coin wrapper and keep the balance.
///
/// Returns: `0x2::balance::Balance<T0>`.
pub async fn into_balance<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentCoin<T0>,
) -> Argument {
    let a0 = arg0.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "coin",
        "into_balance",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![a0],
    )
}
/// Take a `Coin` worth of `value` from `Balance`.
/// Aborts if `value > balance.value`
///
/// Returns: `0x2::coin::Coin<T0>`.
pub async fn take<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl super::balance::ArgumentBalance<T0>,
    arg1: impl PureU64,
) -> Argument {
    let a0 = arg0.into_argument_mut(b).await;
    let a1 = arg1.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "coin",
        "take",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![a0, a1],
    )
}
/// Put a `Coin<T>` to the `Balance<T>`.
pub async fn put<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl super::balance::ArgumentBalance<T0>,
    arg1: impl ArgumentCoin<T0>,
) {
    let a0 = arg0.into_argument_mut(b).await;
    let a1 = arg1.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "coin",
        "put",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![a0, a1],
    );
}
/// Consume the coin `c` and add its value to `self`.
/// Aborts if `c.value + self.value > U64_MAX`
pub async fn join<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentCoin<T0>,
    arg1: impl ArgumentCoin<T0>,
) {
    let a0 = arg0.into_argument_mut(b).await;
    let a1 = arg1.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "coin",
        "join",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![a0, a1],
    );
}
/// Split coin `self` to two coins, one with balance `split_amount`,
/// and the remaining balance is left is `self`.
///
/// Returns: `0x2::coin::Coin<T0>`.
pub async fn split<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentCoin<T0>,
    arg1: impl PureU64,
) -> Argument {
    let a0 = arg0.into_argument_mut(b).await;
    let a1 = arg1.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "coin",
        "split",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![a0, a1],
    )
}
/// Split coin `self` into `n - 1` coins with equal balances. The remainder is left in
/// `self`. Return newly created coins.
///
/// Returns: `vector<0x2::coin::Coin<T0>>`.
pub async fn divide_into_n<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentCoin<T0>,
    arg1: impl PureU64,
) -> Argument {
    let a0 = arg0.into_argument_mut(b).await;
    let a1 = arg1.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "coin",
        "divide_into_n",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![a0, a1],
    )
}
/// Make any Coin with a zero value. Useful for placeholding
/// bids/payments or preemptively making empty balances.
///
/// Returns: `0x2::coin::Coin<T0>`.
pub async fn zero<T0: MoveType>(b: &mut PtbBuilder) -> Argument {
    b.move_call(
        b.package_id::<super::Package>(),
        "coin",
        "zero",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![],
    )
}
/// Destroy a coin with value zero
pub async fn destroy_zero<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentCoin<T0>,
) {
    let a0 = arg0.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "coin",
        "destroy_zero",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![a0],
    );
}
/// Create a new currency type `T` as and return the `TreasuryCap` for
/// `T` to the caller. Can only be called with a `one-time-witness`
/// type, ensuring that there's only one `TreasuryCap` per `T`.
///
/// Returns: `(0x2::coin::TreasuryCap<T0>, 0x2::coin::CoinMetadata<T0>)`.
pub async fn create_currency<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentObject<()>,
    arg1: impl PureU8,
    arg2: impl PureVec<u8>,
    arg3: impl PureVec<u8>,
    arg4: impl PureVec<u8>,
    arg5: impl PureOption<super::url::Url>,
) -> (Argument, Argument) {
    let a0 = arg0.into_argument(b).await;
    let a1 = arg1.into_argument(b).await;
    let a2 = arg2.into_argument(b).await;
    let a3 = arg3.into_argument(b).await;
    let a4 = arg4.into_argument(b).await;
    let a5 = arg5.into_argument(b).await;
    let __r = b
        .move_call_n(
            b.package_id::<super::Package>(),
            "coin",
            "create_currency",
            vec![< T0 as MoveType > ::type_tag(b)],
            vec![a0, a1, a2, a3, a4, a5],
            2u16,
        );
    (__r[0], __r[1])
}
/// This creates a new currency, via `create_currency`, but with an extra capability that
/// allows for specific addresses to have their coins frozen. When an address is added to the
/// deny list, it is immediately unable to interact with the currency's coin as input objects.
/// Additionally at the start of the next epoch, they will be unable to receive the currency's
/// coin.
/// The `allow_global_pause` flag enables an additional API that will cause all addresses to
/// be denied. Note however, that this doesn't affect per-address entries of the deny list and
/// will not change the result of the "contains" APIs.
///
/// Returns: `(0x2::coin::TreasuryCap<T0>, 0x2::coin::DenyCapV1<T0>, 0x2::coin::CoinMetadata<T0>)`.
pub async fn create_regulated_currency_v1<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentObject<()>,
    arg1: impl PureU8,
    arg2: impl PureVec<u8>,
    arg3: impl PureVec<u8>,
    arg4: impl PureVec<u8>,
    arg5: impl PureOption<super::url::Url>,
    arg6: impl PureBool,
) -> (Argument, Argument, Argument) {
    let a0 = arg0.into_argument(b).await;
    let a1 = arg1.into_argument(b).await;
    let a2 = arg2.into_argument(b).await;
    let a3 = arg3.into_argument(b).await;
    let a4 = arg4.into_argument(b).await;
    let a5 = arg5.into_argument(b).await;
    let a6 = arg6.into_argument(b).await;
    let __r = b
        .move_call_n(
            b.package_id::<super::Package>(),
            "coin",
            "create_regulated_currency_v1",
            vec![< T0 as MoveType > ::type_tag(b)],
            vec![a0, a1, a2, a3, a4, a5, a6],
            3u16,
        );
    (__r[0], __r[1], __r[2])
}
/// Create a coin worth `value` and increase the total supply
/// in `cap` accordingly.
///
/// Returns: `0x2::coin::Coin<T0>`.
pub async fn mint<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentTreasuryCap<T0>,
    arg1: impl PureU64,
) -> Argument {
    let a0 = arg0.into_argument_mut(b).await;
    let a1 = arg1.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "coin",
        "mint",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![a0, a1],
    )
}
/// Mint some amount of T as a `Balance` and increase the total
/// supply in `cap` accordingly.
/// Aborts if `value` + `cap.total_supply` >= U64_MAX
///
/// Returns: `0x2::balance::Balance<T0>`.
pub async fn mint_balance<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentTreasuryCap<T0>,
    arg1: impl PureU64,
) -> Argument {
    let a0 = arg0.into_argument_mut(b).await;
    let a1 = arg1.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "coin",
        "mint_balance",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![a0, a1],
    )
}
/// Destroy the coin `c` and decrease the total supply in `cap`
/// accordingly.
///
/// Returns: `u64`.
pub async fn burn<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentTreasuryCap<T0>,
    arg1: impl ArgumentCoin<T0>,
) -> Argument {
    let a0 = arg0.into_argument_mut(b).await;
    let a1 = arg1.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "coin",
        "burn",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![a0, a1],
    )
}
/// Adds the given address to the deny list, preventing it from interacting with the specified
/// coin type as an input to a transaction. Additionally at the start of the next epoch, the
/// address will be unable to receive objects of this coin type.
pub async fn deny_list_v1_add<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl super::deny_list::ArgumentDenyList,
    arg1: impl ArgumentDenyCapV1<T0>,
    arg2: impl PureAddress,
) {
    let a0 = arg0.into_argument_mut(b).await;
    let a1 = arg1.into_argument_mut(b).await;
    let a2 = arg2.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "coin",
        "deny_list_v1_add",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![a0, a1, a2],
    );
}
/// Removes an address from the deny list. Similar to `deny_list_v1_add`, the effect for input
/// objects will be immediate, but the effect for receiving objects will be delayed until the
/// next epoch.
pub async fn deny_list_v1_remove<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl super::deny_list::ArgumentDenyList,
    arg1: impl ArgumentDenyCapV1<T0>,
    arg2: impl PureAddress,
) {
    let a0 = arg0.into_argument_mut(b).await;
    let a1 = arg1.into_argument_mut(b).await;
    let a2 = arg2.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "coin",
        "deny_list_v1_remove",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![a0, a1, a2],
    );
}
/// Check if the deny list contains the given address for the current epoch. Denied addresses
/// in the current epoch will be unable to receive objects of this coin type.
///
/// Returns: `bool`.
pub async fn deny_list_v1_contains_current_epoch<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl super::deny_list::ArgumentDenyList,
    arg1: impl PureAddress,
) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    let a1 = arg1.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "coin",
        "deny_list_v1_contains_current_epoch",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![a0, a1],
    )
}
/// Check if the deny list contains the given address for the next epoch. Denied addresses in
/// the next epoch will immediately be unable to use objects of this coin type as inputs. At the
/// start of the next epoch, the address will be unable to receive objects of this coin type.
///
/// Returns: `bool`.
pub async fn deny_list_v1_contains_next_epoch<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl super::deny_list::ArgumentDenyList,
    arg1: impl PureAddress,
) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    let a1 = arg1.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "coin",
        "deny_list_v1_contains_next_epoch",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![a0, a1],
    )
}
/// Enable the global pause for the given coin type. This will immediately prevent all addresses
/// from using objects of this coin type as inputs. At the start of the next epoch, all
/// addresses will be unable to receive objects of this coin type.
pub async fn deny_list_v1_enable_global_pause<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl super::deny_list::ArgumentDenyList,
    arg1: impl ArgumentDenyCapV1<T0>,
) {
    let a0 = arg0.into_argument_mut(b).await;
    let a1 = arg1.into_argument_mut(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "coin",
        "deny_list_v1_enable_global_pause",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![a0, a1],
    );
}
/// Disable the global pause for the given coin type. This will immediately allow all addresses
/// to resume using objects of this coin type as inputs. However, receiving objects of this coin
/// type will still be paused until the start of the next epoch.
pub async fn deny_list_v1_disable_global_pause<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl super::deny_list::ArgumentDenyList,
    arg1: impl ArgumentDenyCapV1<T0>,
) {
    let a0 = arg0.into_argument_mut(b).await;
    let a1 = arg1.into_argument_mut(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "coin",
        "deny_list_v1_disable_global_pause",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![a0, a1],
    );
}
/// Check if the global pause is enabled for the given coin type in the current epoch.
///
/// Returns: `bool`.
pub async fn deny_list_v1_is_global_pause_enabled_current_epoch<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl super::deny_list::ArgumentDenyList,
) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "coin",
        "deny_list_v1_is_global_pause_enabled_current_epoch",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![a0],
    )
}
/// Check if the global pause is enabled for the given coin type in the next epoch.
///
/// Returns: `bool`.
pub async fn deny_list_v1_is_global_pause_enabled_next_epoch<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl super::deny_list::ArgumentDenyList,
) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "coin",
        "deny_list_v1_is_global_pause_enabled_next_epoch",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![a0],
    )
}
/// Mint `amount` of `Coin` and send it to `recipient`. Invokes `mint()`.
pub async fn mint_and_transfer<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentTreasuryCap<T0>,
    arg1: impl PureU64,
    arg2: impl PureAddress,
) {
    let a0 = arg0.into_argument_mut(b).await;
    let a1 = arg1.into_argument(b).await;
    let a2 = arg2.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "coin",
        "mint_and_transfer",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![a0, a1, a2],
    );
}
/// Update name of the coin in `CoinMetadata`
pub async fn update_name<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentTreasuryCap<T0>,
    arg1: impl ArgumentCoinMetadata<T0>,
    arg2: impl PureString,
) {
    let a0 = arg0.into_argument_ref(b).await;
    let a1 = arg1.into_argument_mut(b).await;
    let a2 = arg2.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "coin",
        "update_name",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![a0, a1, a2],
    );
}
/// Update the symbol of the coin in `CoinMetadata`
pub async fn update_symbol<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentTreasuryCap<T0>,
    arg1: impl ArgumentCoinMetadata<T0>,
    arg2: impl PureString,
) {
    let a0 = arg0.into_argument_ref(b).await;
    let a1 = arg1.into_argument_mut(b).await;
    let a2 = arg2.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "coin",
        "update_symbol",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![a0, a1, a2],
    );
}
/// Update the description of the coin in `CoinMetadata`
pub async fn update_description<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentTreasuryCap<T0>,
    arg1: impl ArgumentCoinMetadata<T0>,
    arg2: impl PureString,
) {
    let a0 = arg0.into_argument_ref(b).await;
    let a1 = arg1.into_argument_mut(b).await;
    let a2 = arg2.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "coin",
        "update_description",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![a0, a1, a2],
    );
}
/// Update the url of the coin in `CoinMetadata`
pub async fn update_icon_url<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentTreasuryCap<T0>,
    arg1: impl ArgumentCoinMetadata<T0>,
    arg2: impl PureString,
) {
    let a0 = arg0.into_argument_ref(b).await;
    let a1 = arg1.into_argument_mut(b).await;
    let a2 = arg2.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "coin",
        "update_icon_url",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![a0, a1, a2],
    );
}
/// Returns: `u8`.
pub async fn get_decimals<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentCoinMetadata<T0>,
) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "coin",
        "get_decimals",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![a0],
    )
}
/// Returns: `0x1::string::String`.
pub async fn get_name<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentCoinMetadata<T0>,
) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "coin",
        "get_name",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![a0],
    )
}
/// Returns: `0x1::ascii::String`.
pub async fn get_symbol<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentCoinMetadata<T0>,
) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "coin",
        "get_symbol",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![a0],
    )
}
/// Returns: `0x1::string::String`.
pub async fn get_description<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentCoinMetadata<T0>,
) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "coin",
        "get_description",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![a0],
    )
}
/// Returns: `0x1::option::Option<0x2::url::Url>`.
pub async fn get_icon_url<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentCoinMetadata<T0>,
) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "coin",
        "get_icon_url",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![a0],
    )
}
