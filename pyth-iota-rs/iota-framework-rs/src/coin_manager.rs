#![allow(unused_imports, non_snake_case, non_upper_case_globals)]
//! The purpose of a CoinManager is to allow access to all
//! properties of a Coin on-chain from within a single shared object
//! This includes access to the total supply and metadata
//! In addition a optional maximum supply can be set and a custom
//! additional Metadata field can be added.
use move_bindgen_runtime::*;
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;
/// The error returned when the maximum supply reached
pub const EMaximumSupplyReached: u64 = 0u64;
/// The error returned if a attempt is made to change the maximum supply after setting it
pub const EMaximumSupplyAlreadySet: u64 = 1u64;
/// The error returned if a attempt is made to change the maximum supply that is lower than the total supply
pub const EMaximumSupplyLowerThanTotalSupply: u64 = 2u64;
/// The error returned if a attempt is made to change the maximum supply that is higher than the maximum possible supply
pub const EMaximumSupplyHigherThanPossible: u64 = 3u64;
/// The error returned if you try to edit nonexisting additional metadata
pub const EAdditionalMetadataDoesNotExist: u64 = 4u64;
/// The maximum supply supported by `CoinManager`
pub const MAX_SUPPLY: u64 = 18446744073709551614u64;
/// Holds all related objects to a Coin in a convenient shared function
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CoinManager<T0: MoveType> {
    pub id: UID,
    /// The original TreasuryCap object as returned by `create_currency`
    pub treasury_cap: super::coin::TreasuryCap<T0>,
    /// Metadata object, original one from the `coin` module, if available
    pub metadata: Option<super::coin::CoinMetadata<T0>>,
    /// Immutable Metadata object, only to be used as a last resort if the original metadata is frozen
    pub immutable_metadata: Option<ImmutableCoinMetadata<T0>>,
    /// Optional maximum supply, if set you can't mint more as this number - can only be set once
    pub maximum_supply: Option<u64>,
    /// Flag indicating if the supply is considered immutable (TreasuryCap is exchanged for this)
    pub supply_immutable: bool,
    /// Flag indicating if the metadata is considered immutable (MetadataCap is exchanged for this)
    pub metadata_immutable: bool,
    #[serde(skip)]
    pub _phantom_t0: PhantomData<T0>,
}
impl<T0: MoveType> MoveType for CoinManager<T0> {
    type Package = super::Package;
    const MODULE: &'static str = "coin_manager";
    const NAME: &'static str = "CoinManager";
    fn type_params(addrs: &impl PackageAddrs) -> Vec<TypeTag> {
        vec![< T0 as MoveType > ::type_tag(addrs)]
    }
}
pub trait ArgumentCoinManager<T0: MoveType>: PTBArgument {
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
impl<T0: MoveType> ArgumentCoinManager<T0> for Argument {}
impl<T0: MoveType> ArgumentCoinManager<T0> for ObjectId {
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
impl<T0: MoveType> ArgumentCoinManager<T0> for ObjectReference {}
impl<T0: MoveType> ArgumentCoinManager<T0> for Shared<ObjectId> {}
impl<T0: MoveType> ArgumentCoinManager<T0> for SharedMut<ObjectId> {}
impl<T0: MoveType> ArgumentCoinManager<T0> for Receiving<ObjectId> {}
/// Like `TreasuryCap`, but for dealing with `TreasuryCap` inside `CoinManager` objects
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CoinManagerTreasuryCap<T0: MoveType> {
    pub id: UID,
    #[serde(skip)]
    pub _phantom_t0: PhantomData<T0>,
}
impl<T0: MoveType> MoveType for CoinManagerTreasuryCap<T0> {
    type Package = super::Package;
    const MODULE: &'static str = "coin_manager";
    const NAME: &'static str = "CoinManagerTreasuryCap";
    fn type_params(addrs: &impl PackageAddrs) -> Vec<TypeTag> {
        vec![< T0 as MoveType > ::type_tag(addrs)]
    }
}
pub trait ArgumentCoinManagerTreasuryCap<T0: MoveType>: PTBArgument {
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
impl<T0: MoveType> ArgumentCoinManagerTreasuryCap<T0> for Argument {}
impl<T0: MoveType> ArgumentCoinManagerTreasuryCap<T0> for ObjectId {
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
impl<T0: MoveType> ArgumentCoinManagerTreasuryCap<T0> for ObjectReference {}
impl<T0: MoveType> ArgumentCoinManagerTreasuryCap<T0> for Shared<ObjectId> {}
impl<T0: MoveType> ArgumentCoinManagerTreasuryCap<T0> for SharedMut<ObjectId> {}
impl<T0: MoveType> ArgumentCoinManagerTreasuryCap<T0> for Receiving<ObjectId> {}
/// Metadata has it's own Cap, independent of the `TreasuryCap`
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CoinManagerMetadataCap<T0: MoveType> {
    pub id: UID,
    #[serde(skip)]
    pub _phantom_t0: PhantomData<T0>,
}
impl<T0: MoveType> MoveType for CoinManagerMetadataCap<T0> {
    type Package = super::Package;
    const MODULE: &'static str = "coin_manager";
    const NAME: &'static str = "CoinManagerMetadataCap";
    fn type_params(addrs: &impl PackageAddrs) -> Vec<TypeTag> {
        vec![< T0 as MoveType > ::type_tag(addrs)]
    }
}
pub trait ArgumentCoinManagerMetadataCap<T0: MoveType>: PTBArgument {
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
impl<T0: MoveType> ArgumentCoinManagerMetadataCap<T0> for Argument {}
impl<T0: MoveType> ArgumentCoinManagerMetadataCap<T0> for ObjectId {
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
impl<T0: MoveType> ArgumentCoinManagerMetadataCap<T0> for ObjectReference {}
impl<T0: MoveType> ArgumentCoinManagerMetadataCap<T0> for Shared<ObjectId> {}
impl<T0: MoveType> ArgumentCoinManagerMetadataCap<T0> for SharedMut<ObjectId> {}
impl<T0: MoveType> ArgumentCoinManagerMetadataCap<T0> for Receiving<ObjectId> {}
/// The immutable version of CoinMetadata, used in case of migrating from frozen objects
/// to a `CoinManager` holding the metadata.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ImmutableCoinMetadata<T0: MoveType> {
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
impl<T0: MoveType> MoveType for ImmutableCoinMetadata<T0> {
    type Package = super::Package;
    const MODULE: &'static str = "coin_manager";
    const NAME: &'static str = "ImmutableCoinMetadata";
    fn type_params(addrs: &impl PackageAddrs) -> Vec<TypeTag> {
        vec![< T0 as MoveType > ::type_tag(addrs)]
    }
}
impl<T0: MoveType + ::serde::Serialize> MoveArg for ImmutableCoinMetadata<T0> {
    fn pure_bytes(self) -> PureBytes {
        PureBytes(::bcs::to_bytes(&self).expect("bcs serialization failed"))
    }
}
pub trait ArgumentImmutableCoinMetadata<T0: MoveType>: PTBArgument {
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
impl<T0: MoveType + ::serde::Serialize> ArgumentImmutableCoinMetadata<T0>
for ImmutableCoinMetadata<T0> {}
impl<T0: MoveType> ArgumentImmutableCoinMetadata<T0> for Argument {}
/// Event triggered once `Coin` ownership is transferred to a new `CoinManager`
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CoinManaged {
    pub coin_name: String,
}
impl MoveType for CoinManaged {
    type Package = super::Package;
    const MODULE: &'static str = "coin_manager";
    const NAME: &'static str = "CoinManaged";
}
impl MoveArg for CoinManaged {
    fn pure_bytes(self) -> PureBytes {
        PureBytes(::bcs::to_bytes(&self).expect("bcs serialization failed"))
    }
}
pub trait ArgumentCoinManaged: PTBArgument {
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
impl ArgumentCoinManaged for CoinManaged {}
impl ArgumentCoinManaged for Argument {}
/// Event triggered if the ownership of the treasury part of a `CoinManager` is renounced
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TreasuryOwnershipRenounced {
    pub coin_name: String,
}
impl MoveType for TreasuryOwnershipRenounced {
    type Package = super::Package;
    const MODULE: &'static str = "coin_manager";
    const NAME: &'static str = "TreasuryOwnershipRenounced";
}
impl MoveArg for TreasuryOwnershipRenounced {
    fn pure_bytes(self) -> PureBytes {
        PureBytes(::bcs::to_bytes(&self).expect("bcs serialization failed"))
    }
}
pub trait ArgumentTreasuryOwnershipRenounced: PTBArgument {
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
impl ArgumentTreasuryOwnershipRenounced for TreasuryOwnershipRenounced {}
impl ArgumentTreasuryOwnershipRenounced for Argument {}
/// Event triggered if the ownership of the metadata part of a `CoinManager` is renounced
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MetadataOwnershipRenounced {
    pub coin_name: String,
}
impl MoveType for MetadataOwnershipRenounced {
    type Package = super::Package;
    const MODULE: &'static str = "coin_manager";
    const NAME: &'static str = "MetadataOwnershipRenounced";
}
impl MoveArg for MetadataOwnershipRenounced {
    fn pure_bytes(self) -> PureBytes {
        PureBytes(::bcs::to_bytes(&self).expect("bcs serialization failed"))
    }
}
pub trait ArgumentMetadataOwnershipRenounced: PTBArgument {
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
impl ArgumentMetadataOwnershipRenounced for MetadataOwnershipRenounced {}
impl ArgumentMetadataOwnershipRenounced for Argument {}
/// Wraps all important objects related to a `Coin` inside a shared object
///
/// Returns: `(0x2::coin_manager::CoinManagerTreasuryCap<T0>, 0x2::coin_manager::CoinManagerMetadataCap<T0>, 0x2::coin_manager::CoinManager<T0>)`.
pub async fn new<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl super::coin::ArgumentTreasuryCap<T0>,
    arg1: impl super::coin::ArgumentCoinMetadata<T0>,
) -> (Argument, Argument, Argument) {
    let a0 = arg0.into_argument(b).await;
    let a1 = arg1.into_argument(b).await;
    let __r = b
        .move_call_n(
            b.package_id::<super::Package>(),
            "coin_manager",
            "new",
            vec![< T0 as MoveType > ::type_tag(b)],
            vec![a0, a1],
            3u16,
        );
    (__r[0], __r[1], __r[2])
}
/// This function allows the same as `new` but under the assumption the Metadata can not be transferred
/// This would typically be the case with `Coin` instances where the metadata is already frozen.
///
/// Returns: `(0x2::coin_manager::CoinManagerTreasuryCap<T0>, 0x2::coin_manager::CoinManager<T0>)`.
pub async fn new_with_immutable_metadata<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl super::coin::ArgumentTreasuryCap<T0>,
    arg1: impl super::coin::ArgumentCoinMetadata<T0>,
) -> (Argument, Argument) {
    let a0 = arg0.into_argument(b).await;
    let a1 = arg1.into_argument_ref(b).await;
    let __r = b
        .move_call_n(
            b.package_id::<super::Package>(),
            "coin_manager",
            "new_with_immutable_metadata",
            vec![< T0 as MoveType > ::type_tag(b)],
            vec![a0, a1],
            2u16,
        );
    (__r[0], __r[1])
}
/// Convenience wrapper to create a new `Coin` and instantly wrap the cap inside a `CoinManager`
///
/// Returns: `(0x2::coin_manager::CoinManagerTreasuryCap<T0>, 0x2::coin_manager::CoinManagerMetadataCap<T0>, 0x2::coin_manager::CoinManager<T0>)`.
pub async fn create<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentObject<()>,
    arg1: impl PureU8,
    arg2: impl PureVec<u8>,
    arg3: impl PureVec<u8>,
    arg4: impl PureVec<u8>,
    arg5: impl PureOption<super::url::Url>,
) -> (Argument, Argument, Argument) {
    let a0 = arg0.into_argument(b).await;
    let a1 = arg1.into_argument(b).await;
    let a2 = arg2.into_argument(b).await;
    let a3 = arg3.into_argument(b).await;
    let a4 = arg4.into_argument(b).await;
    let a5 = arg5.into_argument(b).await;
    let __r = b
        .move_call_n(
            b.package_id::<super::Package>(),
            "coin_manager",
            "create",
            vec![< T0 as MoveType > ::type_tag(b)],
            vec![a0, a1, a2, a3, a4, a5],
            3u16,
        );
    (__r[0], __r[1], __r[2])
}
pub async fn add_additional_metadata<T0: MoveType, T1: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentCoinManagerMetadataCap<T0>,
    arg1: impl ArgumentCoinManager<T0>,
    arg2: impl ArgumentObject<()>,
) {
    let a0 = arg0.into_argument_ref(b).await;
    let a1 = arg1.into_argument_mut(b).await;
    let a2 = arg2.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "coin_manager",
        "add_additional_metadata",
        vec![< T0 as MoveType > ::type_tag(b), < T1 as MoveType > ::type_tag(b)],
        vec![a0, a1, a2],
    );
}
/// Returns: `T2`.
pub async fn replace_additional_metadata<T0: MoveType, T1: MoveType, T2: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentCoinManagerMetadataCap<T0>,
    arg1: impl ArgumentCoinManager<T0>,
    arg2: impl ArgumentObject<()>,
) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    let a1 = arg1.into_argument_mut(b).await;
    let a2 = arg2.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "coin_manager",
        "replace_additional_metadata",
        vec![
            < T0 as MoveType > ::type_tag(b), < T1 as MoveType > ::type_tag(b), < T2 as
            MoveType > ::type_tag(b)
        ],
        vec![a0, a1, a2],
    )
}
/// Returns: `&T1`.
pub async fn additional_metadata<T0: MoveType, T1: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentCoinManager<T0>,
) -> Argument {
    let a0 = arg0.into_argument_mut(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "coin_manager",
        "additional_metadata",
        vec![< T0 as MoveType > ::type_tag(b), < T1 as MoveType > ::type_tag(b)],
        vec![a0],
    )
}
/// A one-time callable function to set a maximum mintable supply on a coin.
/// This can only be set once and is irrevertable.
pub async fn enforce_maximum_supply<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentCoinManagerTreasuryCap<T0>,
    arg1: impl ArgumentCoinManager<T0>,
    arg2: impl PureU64,
) {
    let a0 = arg0.into_argument_ref(b).await;
    let a1 = arg1.into_argument_mut(b).await;
    let a2 = arg2.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "coin_manager",
        "enforce_maximum_supply",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![a0, a1, a2],
    );
}
/// A irreversible action renouncing supply ownership which can be called if you hold the `CoinManagerTreasuryCap`.
/// This action provides `Coin` holders with some assurances if called, namely that there will
/// not be any new minting or changes to the supply from this point onward. The maximum supply
/// will be set to the current supply and will not be changed any more afterwards.
pub async fn renounce_treasury_ownership<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentCoinManagerTreasuryCap<T0>,
    arg1: impl ArgumentCoinManager<T0>,
) {
    let a0 = arg0.into_argument(b).await;
    let a1 = arg1.into_argument_mut(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "coin_manager",
        "renounce_treasury_ownership",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![a0, a1],
    );
}
/// A irreversible action renouncing manager ownership which can be called if you hold the `CoinManagerMetadataCap`.
/// This action provides `Coin` holders with some assurances if called, namely that there will
/// not be any changes to the metadata from this point onward.
pub async fn renounce_metadata_ownership<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentCoinManagerMetadataCap<T0>,
    arg1: impl ArgumentCoinManager<T0>,
) {
    let a0 = arg0.into_argument(b).await;
    let a1 = arg1.into_argument_mut(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "coin_manager",
        "renounce_metadata_ownership",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![a0, a1],
    );
}
/// Convenience function allowing users to query if the ownership of the supply of this `Coin`
/// and thus the ability to mint new `Coin` has been renounced.
///
/// Returns: `bool`.
pub async fn supply_is_immutable<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentCoinManager<T0>,
) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "coin_manager",
        "supply_is_immutable",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![a0],
    )
}
/// Convenience function allowing users to query if the ownership of the metadata management
/// and thus the ability to change any of the metadata has been renounced.
///
/// Returns: `bool`.
pub async fn metadata_is_immutable<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentCoinManager<T0>,
) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "coin_manager",
        "metadata_is_immutable",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![a0],
    )
}
/// Returns: `&0x2::coin::CoinMetadata<T0>`.
pub async fn metadata<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentCoinManager<T0>,
) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "coin_manager",
        "metadata",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![a0],
    )
}
/// Returns: `&0x2::coin_manager::ImmutableCoinMetadata<T0>`.
pub async fn immutable_metadata<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentCoinManager<T0>,
) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "coin_manager",
        "immutable_metadata",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![a0],
    )
}
/// Get the total supply as a number
///
/// Returns: `u64`.
pub async fn total_supply<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentCoinManager<T0>,
) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "coin_manager",
        "total_supply",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![a0],
    )
}
/// Get the maximum supply possible as a number.
/// If no maximum set it's the maximum u64 possible
///
/// Returns: `u64`.
pub async fn maximum_supply<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentCoinManager<T0>,
) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "coin_manager",
        "maximum_supply",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![a0],
    )
}
/// Convenience function returning the remaining supply that can be minted still
///
/// Returns: `u64`.
pub async fn available_supply<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentCoinManager<T0>,
) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "coin_manager",
        "available_supply",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![a0],
    )
}
/// Returns if a maximum supply has been set for this Coin or not
///
/// Returns: `bool`.
pub async fn has_maximum_supply<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentCoinManager<T0>,
) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "coin_manager",
        "has_maximum_supply",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![a0],
    )
}
/// Get immutable reference to the treasury's `Supply`.
///
/// Returns: `&0x2::balance::Supply<T0>`.
pub async fn supply_immut<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentCoinManager<T0>,
) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "coin_manager",
        "supply_immut",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![a0],
    )
}
/// Create a coin worth `value` and increase the total supply
/// in `cap` accordingly.
///
/// Returns: `0x2::coin::Coin<T0>`.
pub async fn mint<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentCoinManagerTreasuryCap<T0>,
    arg1: impl ArgumentCoinManager<T0>,
    arg2: impl PureU64,
) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    let a1 = arg1.into_argument_mut(b).await;
    let a2 = arg2.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "coin_manager",
        "mint",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![a0, a1, a2],
    )
}
/// Mint some amount of T as a `Balance` and increase the total
/// supply in `cap` accordingly.
/// Aborts if `value` + `cap.total_supply` >= U64_MAX
///
/// Returns: `0x2::balance::Balance<T0>`.
pub async fn mint_balance<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentCoinManagerTreasuryCap<T0>,
    arg1: impl ArgumentCoinManager<T0>,
    arg2: impl PureU64,
) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    let a1 = arg1.into_argument_mut(b).await;
    let a2 = arg2.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "coin_manager",
        "mint_balance",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![a0, a1, a2],
    )
}
/// Destroy the coin `c` and decrease the total supply in `cap`
/// accordingly.
///
/// Returns: `u64`.
pub async fn burn<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentCoinManagerTreasuryCap<T0>,
    arg1: impl ArgumentCoinManager<T0>,
    arg2: impl super::coin::ArgumentCoin<T0>,
) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    let a1 = arg1.into_argument_mut(b).await;
    let a2 = arg2.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "coin_manager",
        "burn",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![a0, a1, a2],
    )
}
/// Mint `amount` of `Coin` and send it to `recipient`. Invokes `mint()`.
pub async fn mint_and_transfer<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentCoinManagerTreasuryCap<T0>,
    arg1: impl ArgumentCoinManager<T0>,
    arg2: impl PureU64,
    arg3: impl PureAddress,
) {
    let a0 = arg0.into_argument_ref(b).await;
    let a1 = arg1.into_argument_mut(b).await;
    let a2 = arg2.into_argument(b).await;
    let a3 = arg3.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "coin_manager",
        "mint_and_transfer",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![a0, a1, a2, a3],
    );
}
/// Update the `name` of the coin in the `CoinMetadata`.
pub async fn update_name<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentCoinManagerMetadataCap<T0>,
    arg1: impl ArgumentCoinManager<T0>,
    arg2: impl PureString,
) {
    let a0 = arg0.into_argument_ref(b).await;
    let a1 = arg1.into_argument_mut(b).await;
    let a2 = arg2.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "coin_manager",
        "update_name",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![a0, a1, a2],
    );
}
/// Update the `symbol` of the coin in the `CoinMetadata`.
pub async fn update_symbol<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentCoinManagerMetadataCap<T0>,
    arg1: impl ArgumentCoinManager<T0>,
    arg2: impl PureString,
) {
    let a0 = arg0.into_argument_ref(b).await;
    let a1 = arg1.into_argument_mut(b).await;
    let a2 = arg2.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "coin_manager",
        "update_symbol",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![a0, a1, a2],
    );
}
/// Update the `description` of the coin in the `CoinMetadata`.
pub async fn update_description<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentCoinManagerMetadataCap<T0>,
    arg1: impl ArgumentCoinManager<T0>,
    arg2: impl PureString,
) {
    let a0 = arg0.into_argument_ref(b).await;
    let a1 = arg1.into_argument_mut(b).await;
    let a2 = arg2.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "coin_manager",
        "update_description",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![a0, a1, a2],
    );
}
/// Update the `url` of the coin in the `CoinMetadata`
pub async fn update_icon_url<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentCoinManagerMetadataCap<T0>,
    arg1: impl ArgumentCoinManager<T0>,
    arg2: impl PureString,
) {
    let a0 = arg0.into_argument_ref(b).await;
    let a1 = arg1.into_argument_mut(b).await;
    let a2 = arg2.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "coin_manager",
        "update_icon_url",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![a0, a1, a2],
    );
}
/// Returns: `u8`.
pub async fn decimals<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentCoinManager<T0>,
) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "coin_manager",
        "decimals",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![a0],
    )
}
/// Returns: `0x1::string::String`.
pub async fn name<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentCoinManager<T0>,
) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "coin_manager",
        "name",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![a0],
    )
}
/// Returns: `0x1::ascii::String`.
pub async fn symbol<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentCoinManager<T0>,
) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "coin_manager",
        "symbol",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![a0],
    )
}
/// Returns: `0x1::string::String`.
pub async fn description<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentCoinManager<T0>,
) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "coin_manager",
        "description",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![a0],
    )
}
/// Returns: `0x1::option::Option<0x2::url::Url>`.
pub async fn icon_url<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentCoinManager<T0>,
) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "coin_manager",
        "icon_url",
        vec![< T0 as MoveType > ::type_tag(b)],
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
    pub fn coinmanaged_tag(&self) -> TypeTag {
        <CoinManaged as MoveType>::type_tag_at(self.package)
    }
    pub fn treasuryownershiprenounced_tag(&self) -> TypeTag {
        <TreasuryOwnershipRenounced as MoveType>::type_tag_at(self.package)
    }
    pub fn metadataownershiprenounced_tag(&self) -> TypeTag {
        <MetadataOwnershipRenounced as MoveType>::type_tag_at(self.package)
    }
}
