#![allow(unused_imports, non_snake_case, non_upper_case_globals)]
use move_bindgen_runtime::*;
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;
pub const ENotOwner: u64 = 0u64;
pub const EIncorrectAmount: u64 = 1u64;
pub const ENotEnough: u64 = 2u64;
pub const ENotEmpty: u64 = 3u64;
pub const EListedExclusively: u64 = 4u64;
pub const EWrongKiosk: u64 = 5u64;
pub const EAlreadyListed: u64 = 6u64;
pub const EItemLocked: u64 = 8u64;
pub const EItemIsListed: u64 = 9u64;
pub const EItemMismatch: u64 = 10u64;
pub const EItemNotFound: u64 = 11u64;
pub const ENotListed: u64 = 12u64;
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Kiosk {
    pub id: UID,
    pub profits: super::balance::Balance<super::iota::IOTA>,
    pub owner: Address,
    pub item_count: u32,
}
impl MoveType for Kiosk {
    type Package = super::Package;
    const MODULE: &'static str = "kiosk";
    const NAME: &'static str = "Kiosk";
}
pub trait ArgumentKiosk: PTBArgument {
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
impl ArgumentKiosk for Argument {}
impl ArgumentKiosk for ObjectId {
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
impl ArgumentKiosk for ObjectReference {}
impl ArgumentKiosk for Shared<ObjectId> {}
impl ArgumentKiosk for SharedMut<ObjectId> {}
impl ArgumentKiosk for Receiving<ObjectId> {}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct KioskOwnerCap {
    pub id: UID,
    pub r#for: ID,
}
impl MoveType for KioskOwnerCap {
    type Package = super::Package;
    const MODULE: &'static str = "kiosk";
    const NAME: &'static str = "KioskOwnerCap";
}
pub trait ArgumentKioskOwnerCap: PTBArgument {
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
impl ArgumentKioskOwnerCap for Argument {}
impl ArgumentKioskOwnerCap for ObjectId {
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
impl ArgumentKioskOwnerCap for ObjectReference {}
impl ArgumentKioskOwnerCap for Shared<ObjectId> {}
impl ArgumentKioskOwnerCap for SharedMut<ObjectId> {}
impl ArgumentKioskOwnerCap for Receiving<ObjectId> {}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PurchaseCap<T0: MoveType> {
    pub id: UID,
    pub kiosk_id: ID,
    pub item_id: ID,
    pub min_price: u64,
    #[serde(skip)]
    pub _phantom_t0: PhantomData<T0>,
}
impl<T0: MoveType> MoveType for PurchaseCap<T0> {
    type Package = super::Package;
    const MODULE: &'static str = "kiosk";
    const NAME: &'static str = "PurchaseCap";
    fn type_params(addrs: &impl PackageAddrs) -> Vec<TypeTag> {
        vec![< T0 as MoveType > ::type_tag(addrs)]
    }
}
pub trait ArgumentPurchaseCap<T0: MoveType>: PTBArgument {
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
impl<T0: MoveType> ArgumentPurchaseCap<T0> for Argument {}
impl<T0: MoveType> ArgumentPurchaseCap<T0> for ObjectId {
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
impl<T0: MoveType> ArgumentPurchaseCap<T0> for ObjectReference {}
impl<T0: MoveType> ArgumentPurchaseCap<T0> for Shared<ObjectId> {}
impl<T0: MoveType> ArgumentPurchaseCap<T0> for SharedMut<ObjectId> {}
impl<T0: MoveType> ArgumentPurchaseCap<T0> for Receiving<ObjectId> {}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Borrow {
    pub kiosk_id: ID,
    pub item_id: ID,
}
impl MoveType for Borrow {
    type Package = super::Package;
    const MODULE: &'static str = "kiosk";
    const NAME: &'static str = "Borrow";
}
impl MoveArg for Borrow {
    fn pure_bytes(self) -> PureBytes {
        PureBytes(::bcs::to_bytes(&self).expect("bcs serialization failed"))
    }
}
pub trait ArgumentBorrow: PTBArgument {
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
impl ArgumentBorrow for Borrow {}
impl ArgumentBorrow for Argument {}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Item {
    pub id: ID,
}
impl MoveType for Item {
    type Package = super::Package;
    const MODULE: &'static str = "kiosk";
    const NAME: &'static str = "Item";
}
impl MoveArg for Item {
    fn pure_bytes(self) -> PureBytes {
        PureBytes(::bcs::to_bytes(&self).expect("bcs serialization failed"))
    }
}
pub trait ArgumentItem: PTBArgument {
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
impl ArgumentItem for Item {}
impl ArgumentItem for Argument {}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Listing {
    pub id: ID,
    pub is_exclusive: bool,
}
impl MoveType for Listing {
    type Package = super::Package;
    const MODULE: &'static str = "kiosk";
    const NAME: &'static str = "Listing";
}
impl MoveArg for Listing {
    fn pure_bytes(self) -> PureBytes {
        PureBytes(::bcs::to_bytes(&self).expect("bcs serialization failed"))
    }
}
pub trait ArgumentListing: PTBArgument {
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
impl ArgumentListing for Listing {}
impl ArgumentListing for Argument {}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Lock {
    pub id: ID,
}
impl MoveType for Lock {
    type Package = super::Package;
    const MODULE: &'static str = "kiosk";
    const NAME: &'static str = "Lock";
}
impl MoveArg for Lock {
    fn pure_bytes(self) -> PureBytes {
        PureBytes(::bcs::to_bytes(&self).expect("bcs serialization failed"))
    }
}
pub trait ArgumentLock: PTBArgument {
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
impl ArgumentLock for Lock {}
impl ArgumentLock for Argument {}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ItemListed<T0: MoveType> {
    pub kiosk: ID,
    pub id: ID,
    pub price: u64,
    #[serde(skip)]
    pub _phantom_t0: PhantomData<T0>,
}
impl<T0: MoveType> MoveType for ItemListed<T0> {
    type Package = super::Package;
    const MODULE: &'static str = "kiosk";
    const NAME: &'static str = "ItemListed";
    fn type_params(addrs: &impl PackageAddrs) -> Vec<TypeTag> {
        vec![< T0 as MoveType > ::type_tag(addrs)]
    }
}
impl<T0: MoveType + ::serde::Serialize> MoveArg for ItemListed<T0> {
    fn pure_bytes(self) -> PureBytes {
        PureBytes(::bcs::to_bytes(&self).expect("bcs serialization failed"))
    }
}
pub trait ArgumentItemListed<T0: MoveType>: PTBArgument {
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
impl<T0: MoveType + ::serde::Serialize> ArgumentItemListed<T0> for ItemListed<T0> {}
impl<T0: MoveType> ArgumentItemListed<T0> for Argument {}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ItemPurchased<T0: MoveType> {
    pub kiosk: ID,
    pub id: ID,
    pub price: u64,
    #[serde(skip)]
    pub _phantom_t0: PhantomData<T0>,
}
impl<T0: MoveType> MoveType for ItemPurchased<T0> {
    type Package = super::Package;
    const MODULE: &'static str = "kiosk";
    const NAME: &'static str = "ItemPurchased";
    fn type_params(addrs: &impl PackageAddrs) -> Vec<TypeTag> {
        vec![< T0 as MoveType > ::type_tag(addrs)]
    }
}
impl<T0: MoveType + ::serde::Serialize> MoveArg for ItemPurchased<T0> {
    fn pure_bytes(self) -> PureBytes {
        PureBytes(::bcs::to_bytes(&self).expect("bcs serialization failed"))
    }
}
pub trait ArgumentItemPurchased<T0: MoveType>: PTBArgument {
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
impl<T0: MoveType + ::serde::Serialize> ArgumentItemPurchased<T0> for ItemPurchased<T0> {}
impl<T0: MoveType> ArgumentItemPurchased<T0> for Argument {}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ItemDelisted<T0: MoveType> {
    pub kiosk: ID,
    pub id: ID,
    #[serde(skip)]
    pub _phantom_t0: PhantomData<T0>,
}
impl<T0: MoveType> MoveType for ItemDelisted<T0> {
    type Package = super::Package;
    const MODULE: &'static str = "kiosk";
    const NAME: &'static str = "ItemDelisted";
    fn type_params(addrs: &impl PackageAddrs) -> Vec<TypeTag> {
        vec![< T0 as MoveType > ::type_tag(addrs)]
    }
}
impl<T0: MoveType + ::serde::Serialize> MoveArg for ItemDelisted<T0> {
    fn pure_bytes(self) -> PureBytes {
        PureBytes(::bcs::to_bytes(&self).expect("bcs serialization failed"))
    }
}
pub trait ArgumentItemDelisted<T0: MoveType>: PTBArgument {
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
impl<T0: MoveType + ::serde::Serialize> ArgumentItemDelisted<T0> for ItemDelisted<T0> {}
impl<T0: MoveType> ArgumentItemDelisted<T0> for Argument {}
pub async fn default(b: &mut PtbBuilder) {
    b.move_call(
        b.package_id::<super::Package>(),
        "kiosk",
        "default",
        Vec::new(),
        vec![],
    );
}
/// Returns: `(0x2::kiosk::Kiosk, 0x2::kiosk::KioskOwnerCap)`.
pub async fn new(b: &mut PtbBuilder) -> (Argument, Argument) {
    let __r = b
        .move_call_n(
            b.package_id::<super::Package>(),
            "kiosk",
            "new",
            Vec::new(),
            vec![],
            2u16,
        );
    (__r[0], __r[1])
}
/// Returns: `0x2::coin::Coin<0x2::iota::IOTA>`.
pub async fn close_and_withdraw(
    b: &mut PtbBuilder,
    arg0: impl ArgumentKiosk,
    arg1: impl ArgumentKioskOwnerCap,
) -> Argument {
    let a0 = arg0.into_argument(b).await;
    let a1 = arg1.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "kiosk",
        "close_and_withdraw",
        Vec::new(),
        vec![a0, a1],
    )
}
pub async fn set_owner(
    b: &mut PtbBuilder,
    arg0: impl ArgumentKiosk,
    arg1: impl ArgumentKioskOwnerCap,
) {
    let a0 = arg0.into_argument_mut(b).await;
    let a1 = arg1.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "kiosk",
        "set_owner",
        Vec::new(),
        vec![a0, a1],
    );
}
pub async fn set_owner_custom(
    b: &mut PtbBuilder,
    arg0: impl ArgumentKiosk,
    arg1: impl ArgumentKioskOwnerCap,
    arg2: impl PureAddress,
) {
    let a0 = arg0.into_argument_mut(b).await;
    let a1 = arg1.into_argument_ref(b).await;
    let a2 = arg2.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "kiosk",
        "set_owner_custom",
        Vec::new(),
        vec![a0, a1, a2],
    );
}
pub async fn place<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentKiosk,
    arg1: impl ArgumentKioskOwnerCap,
    arg2: impl ArgumentObject<()>,
) {
    let a0 = arg0.into_argument_mut(b).await;
    let a1 = arg1.into_argument_ref(b).await;
    let a2 = arg2.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "kiosk",
        "place",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![a0, a1, a2],
    );
}
pub async fn lock<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentKiosk,
    arg1: impl ArgumentKioskOwnerCap,
    arg2: impl super::transfer_policy::ArgumentTransferPolicy<T0>,
    arg3: impl ArgumentObject<()>,
) {
    let a0 = arg0.into_argument_mut(b).await;
    let a1 = arg1.into_argument_ref(b).await;
    let a2 = arg2.into_argument_ref(b).await;
    let a3 = arg3.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "kiosk",
        "lock",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![a0, a1, a2, a3],
    );
}
/// Returns: `T0`.
pub async fn take<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentKiosk,
    arg1: impl ArgumentKioskOwnerCap,
    arg2: impl PureID,
) -> Argument {
    let a0 = arg0.into_argument_mut(b).await;
    let a1 = arg1.into_argument_ref(b).await;
    let a2 = arg2.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "kiosk",
        "take",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![a0, a1, a2],
    )
}
pub async fn list<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentKiosk,
    arg1: impl ArgumentKioskOwnerCap,
    arg2: impl PureID,
    arg3: impl PureU64,
) {
    let a0 = arg0.into_argument_mut(b).await;
    let a1 = arg1.into_argument_ref(b).await;
    let a2 = arg2.into_argument(b).await;
    let a3 = arg3.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "kiosk",
        "list",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![a0, a1, a2, a3],
    );
}
pub async fn place_and_list<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentKiosk,
    arg1: impl ArgumentKioskOwnerCap,
    arg2: impl ArgumentObject<()>,
    arg3: impl PureU64,
) {
    let a0 = arg0.into_argument_mut(b).await;
    let a1 = arg1.into_argument_ref(b).await;
    let a2 = arg2.into_argument(b).await;
    let a3 = arg3.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "kiosk",
        "place_and_list",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![a0, a1, a2, a3],
    );
}
pub async fn delist<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentKiosk,
    arg1: impl ArgumentKioskOwnerCap,
    arg2: impl PureID,
) {
    let a0 = arg0.into_argument_mut(b).await;
    let a1 = arg1.into_argument_ref(b).await;
    let a2 = arg2.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "kiosk",
        "delist",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![a0, a1, a2],
    );
}
/// Returns: `(T0, 0x2::transfer_policy::TransferRequest<T0>)`.
pub async fn purchase<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentKiosk,
    arg1: impl PureID,
    arg2: impl super::coin::ArgumentCoin<super::iota::IOTA>,
) -> (Argument, Argument) {
    let a0 = arg0.into_argument_mut(b).await;
    let a1 = arg1.into_argument(b).await;
    let a2 = arg2.into_argument(b).await;
    let __r = b
        .move_call_n(
            b.package_id::<super::Package>(),
            "kiosk",
            "purchase",
            vec![< T0 as MoveType > ::type_tag(b)],
            vec![a0, a1, a2],
            2u16,
        );
    (__r[0], __r[1])
}
/// Returns: `0x2::kiosk::PurchaseCap<T0>`.
pub async fn list_with_purchase_cap<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentKiosk,
    arg1: impl ArgumentKioskOwnerCap,
    arg2: impl PureID,
    arg3: impl PureU64,
) -> Argument {
    let a0 = arg0.into_argument_mut(b).await;
    let a1 = arg1.into_argument_ref(b).await;
    let a2 = arg2.into_argument(b).await;
    let a3 = arg3.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "kiosk",
        "list_with_purchase_cap",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![a0, a1, a2, a3],
    )
}
/// Returns: `(T0, 0x2::transfer_policy::TransferRequest<T0>)`.
pub async fn purchase_with_cap<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentKiosk,
    arg1: impl ArgumentPurchaseCap<T0>,
    arg2: impl super::coin::ArgumentCoin<super::iota::IOTA>,
) -> (Argument, Argument) {
    let a0 = arg0.into_argument_mut(b).await;
    let a1 = arg1.into_argument(b).await;
    let a2 = arg2.into_argument(b).await;
    let __r = b
        .move_call_n(
            b.package_id::<super::Package>(),
            "kiosk",
            "purchase_with_cap",
            vec![< T0 as MoveType > ::type_tag(b)],
            vec![a0, a1, a2],
            2u16,
        );
    (__r[0], __r[1])
}
pub async fn return_purchase_cap<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentKiosk,
    arg1: impl ArgumentPurchaseCap<T0>,
) {
    let a0 = arg0.into_argument_mut(b).await;
    let a1 = arg1.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "kiosk",
        "return_purchase_cap",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![a0, a1],
    );
}
/// Returns: `0x2::coin::Coin<0x2::iota::IOTA>`.
pub async fn withdraw(
    b: &mut PtbBuilder,
    arg0: impl ArgumentKiosk,
    arg1: impl ArgumentKioskOwnerCap,
    arg2: impl PureOption<u64>,
) -> Argument {
    let a0 = arg0.into_argument_mut(b).await;
    let a1 = arg1.into_argument_ref(b).await;
    let a2 = arg2.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "kiosk",
        "withdraw",
        Vec::new(),
        vec![a0, a1, a2],
    )
}
/// Returns: `bool`.
pub async fn has_item(
    b: &mut PtbBuilder,
    arg0: impl ArgumentKiosk,
    arg1: impl PureID,
) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    let a1 = arg1.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "kiosk",
        "has_item",
        Vec::new(),
        vec![a0, a1],
    )
}
/// Returns: `bool`.
pub async fn has_item_with_type<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentKiosk,
    arg1: impl PureID,
) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    let a1 = arg1.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "kiosk",
        "has_item_with_type",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![a0, a1],
    )
}
/// Returns: `bool`.
pub async fn is_locked(
    b: &mut PtbBuilder,
    arg0: impl ArgumentKiosk,
    arg1: impl PureID,
) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    let a1 = arg1.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "kiosk",
        "is_locked",
        Vec::new(),
        vec![a0, a1],
    )
}
/// Returns: `bool`.
pub async fn is_listed(
    b: &mut PtbBuilder,
    arg0: impl ArgumentKiosk,
    arg1: impl PureID,
) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    let a1 = arg1.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "kiosk",
        "is_listed",
        Vec::new(),
        vec![a0, a1],
    )
}
/// Returns: `bool`.
pub async fn is_listed_exclusively(
    b: &mut PtbBuilder,
    arg0: impl ArgumentKiosk,
    arg1: impl PureID,
) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    let a1 = arg1.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "kiosk",
        "is_listed_exclusively",
        Vec::new(),
        vec![a0, a1],
    )
}
/// Returns: `bool`.
pub async fn has_access(
    b: &mut PtbBuilder,
    arg0: impl ArgumentKiosk,
    arg1: impl ArgumentKioskOwnerCap,
) -> Argument {
    let a0 = arg0.into_argument_mut(b).await;
    let a1 = arg1.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "kiosk",
        "has_access",
        Vec::new(),
        vec![a0, a1],
    )
}
/// Returns: `&mut 0x2::object::UID`.
pub async fn uid_mut_as_owner(
    b: &mut PtbBuilder,
    arg0: impl ArgumentKiosk,
    arg1: impl ArgumentKioskOwnerCap,
) -> Argument {
    let a0 = arg0.into_argument_mut(b).await;
    let a1 = arg1.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "kiosk",
        "uid_mut_as_owner",
        Vec::new(),
        vec![a0, a1],
    )
}
/// Returns: `&0x2::object::UID`.
pub async fn uid(b: &mut PtbBuilder, arg0: impl ArgumentKiosk) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    b.move_call(b.package_id::<super::Package>(), "kiosk", "uid", Vec::new(), vec![a0])
}
/// Returns: `address`.
pub async fn owner(b: &mut PtbBuilder, arg0: impl ArgumentKiosk) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    b.move_call(b.package_id::<super::Package>(), "kiosk", "owner", Vec::new(), vec![a0])
}
/// Returns: `u32`.
pub async fn item_count(b: &mut PtbBuilder, arg0: impl ArgumentKiosk) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "kiosk",
        "item_count",
        Vec::new(),
        vec![a0],
    )
}
/// Returns: `u64`.
pub async fn profits_amount(b: &mut PtbBuilder, arg0: impl ArgumentKiosk) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "kiosk",
        "profits_amount",
        Vec::new(),
        vec![a0],
    )
}
/// Returns: `&mut 0x2::balance::Balance<0x2::iota::IOTA>`.
pub async fn profits_mut(
    b: &mut PtbBuilder,
    arg0: impl ArgumentKiosk,
    arg1: impl ArgumentKioskOwnerCap,
) -> Argument {
    let a0 = arg0.into_argument_mut(b).await;
    let a1 = arg1.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "kiosk",
        "profits_mut",
        Vec::new(),
        vec![a0, a1],
    )
}
/// Returns: `&T0`.
pub async fn borrow<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentKiosk,
    arg1: impl ArgumentKioskOwnerCap,
    arg2: impl PureID,
) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    let a1 = arg1.into_argument_ref(b).await;
    let a2 = arg2.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "kiosk",
        "borrow",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![a0, a1, a2],
    )
}
/// Returns: `&mut T0`.
pub async fn borrow_mut<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentKiosk,
    arg1: impl ArgumentKioskOwnerCap,
    arg2: impl PureID,
) -> Argument {
    let a0 = arg0.into_argument_mut(b).await;
    let a1 = arg1.into_argument_ref(b).await;
    let a2 = arg2.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "kiosk",
        "borrow_mut",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![a0, a1, a2],
    )
}
/// Returns: `(T0, 0x2::kiosk::Borrow)`.
pub async fn borrow_val<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentKiosk,
    arg1: impl ArgumentKioskOwnerCap,
    arg2: impl PureID,
) -> (Argument, Argument) {
    let a0 = arg0.into_argument_mut(b).await;
    let a1 = arg1.into_argument_ref(b).await;
    let a2 = arg2.into_argument(b).await;
    let __r = b
        .move_call_n(
            b.package_id::<super::Package>(),
            "kiosk",
            "borrow_val",
            vec![< T0 as MoveType > ::type_tag(b)],
            vec![a0, a1, a2],
            2u16,
        );
    (__r[0], __r[1])
}
pub async fn return_val<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentKiosk,
    arg1: impl ArgumentObject<()>,
    arg2: impl ArgumentBorrow,
) {
    let a0 = arg0.into_argument_mut(b).await;
    let a1 = arg1.into_argument(b).await;
    let a2 = arg2.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "kiosk",
        "return_val",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![a0, a1, a2],
    );
}
/// Returns: `0x2::object::ID`.
pub async fn kiosk_owner_cap_for(
    b: &mut PtbBuilder,
    arg0: impl ArgumentKioskOwnerCap,
) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "kiosk",
        "kiosk_owner_cap_for",
        Vec::new(),
        vec![a0],
    )
}
/// Returns: `0x2::object::ID`.
pub async fn purchase_cap_kiosk<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentPurchaseCap<T0>,
) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "kiosk",
        "purchase_cap_kiosk",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![a0],
    )
}
/// Returns: `0x2::object::ID`.
pub async fn purchase_cap_item<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentPurchaseCap<T0>,
) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "kiosk",
        "purchase_cap_item",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![a0],
    )
}
/// Returns: `u64`.
pub async fn purchase_cap_min_price<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentPurchaseCap<T0>,
) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "kiosk",
        "purchase_cap_min_price",
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
    pub fn kiosk_tag(&self) -> TypeTag {
        <Kiosk as MoveType>::type_tag_at(self.package)
    }
    pub fn kioskownercap_tag(&self) -> TypeTag {
        <KioskOwnerCap as MoveType>::type_tag_at(self.package)
    }
    pub fn borrow_tag(&self) -> TypeTag {
        <Borrow as MoveType>::type_tag_at(self.package)
    }
    pub fn item_tag(&self) -> TypeTag {
        <Item as MoveType>::type_tag_at(self.package)
    }
    pub fn listing_tag(&self) -> TypeTag {
        <Listing as MoveType>::type_tag_at(self.package)
    }
    pub fn lock_tag(&self) -> TypeTag {
        <Lock as MoveType>::type_tag_at(self.package)
    }
}
