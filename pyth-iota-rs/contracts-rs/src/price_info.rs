#![allow(unused_imports, non_snake_case, non_upper_case_globals)]
use move_bindgen_runtime::*;
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;
pub const KEY: &[u8] = &[
    112u8, 114u8, 105u8, 99u8, 101u8, 95u8, 105u8, 110u8, 102u8, 111u8,
];
pub const FEE_STORAGE_KEY: &[u8] = &[
    102u8, 101u8, 101u8, 95u8, 115u8, 116u8, 111u8, 114u8, 97u8, 103u8, 101u8,
];
pub const E_PRICE_INFO_REGISTRY_ALREADY_EXISTS: u64 = 0u64;
pub const E_PRICE_IDENTIFIER_ALREADY_REGISTERED: u64 = 1u64;
pub const E_PRICE_IDENTIFIER_NOT_REGISTERED: u64 = 2u64;
/// Iota object version of PriceInfo.
/// Has a key ability, is unique for each price identifier, and lives in global store.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PriceInfoObject {
    pub id: UID,
    pub price_info: PriceInfo,
}
impl MoveType for PriceInfoObject {
    type Package = super::Package;
    const MODULE: &'static str = "price_info";
    const NAME: &'static str = "PriceInfoObject";
}
pub trait ArgumentPriceInfoObject: PTBArgument {
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
impl ArgumentPriceInfoObject for Argument {}
impl ArgumentPriceInfoObject for ObjectId {
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
impl ArgumentPriceInfoObject for ObjectReference {}
impl ArgumentPriceInfoObject for Shared<ObjectId> {}
impl ArgumentPriceInfoObject for SharedMut<ObjectId> {}
impl ArgumentPriceInfoObject for Receiving<ObjectId> {}
/// Copyable and droppable.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PriceInfo {
    pub attestation_time: u64,
    pub arrival_time: u64,
    pub price_feed: super::price_feed::PriceFeed,
}
impl MoveType for PriceInfo {
    type Package = super::Package;
    const MODULE: &'static str = "price_info";
    const NAME: &'static str = "PriceInfo";
}
impl MoveArg for PriceInfo {
    fn pure_bytes(self) -> PureBytes {
        PureBytes(::bcs::to_bytes(&self).expect("bcs serialization failed"))
    }
}
pub trait ArgumentPriceInfo: PTBArgument {
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
impl ArgumentPriceInfo for PriceInfo {}
impl ArgumentPriceInfo for Argument {}
/// Returns ID of price info object corresponding to price_identifier as a byte vector.
///
/// Returns: `vector<u8>`.
pub async fn get_id_bytes(
    b: &mut PtbBuilder,
    arg0: impl ArgumentObject<()>,
    arg1: impl super::price_identifier::ArgumentPriceIdentifier,
) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    let a1 = arg1.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "price_info",
        "get_id_bytes",
        Vec::new(),
        vec![a0, a1],
    )
}
/// Returns ID of price info object corresponding to price_identifier as an ID.
///
/// Returns: `0x2::object::ID`.
pub async fn get_id(
    b: &mut PtbBuilder,
    arg0: impl ArgumentObject<()>,
    arg1: impl super::price_identifier::ArgumentPriceIdentifier,
) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    let a1 = arg1.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "price_info",
        "get_id",
        Vec::new(),
        vec![a0, a1],
    )
}
/// Returns: `bool`.
pub async fn contains(
    b: &mut PtbBuilder,
    arg0: impl ArgumentObject<()>,
    arg1: impl super::price_identifier::ArgumentPriceIdentifier,
) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    let a1 = arg1.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "price_info",
        "contains",
        Vec::new(),
        vec![a0, a1],
    )
}
/// Returns: `u64`.
pub async fn get_balance(
    b: &mut PtbBuilder,
    arg0: impl ArgumentPriceInfoObject,
) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "price_info",
        "get_balance",
        Vec::new(),
        vec![a0],
    )
}
pub async fn deposit_fee_coins(
    b: &mut PtbBuilder,
    arg0: impl ArgumentPriceInfoObject,
    arg1: impl ::iota_framework_rs::coin::ArgumentCoin<::iota_framework_rs::iota::IOTA>,
) {
    let a0 = arg0.into_argument_mut(b).await;
    let a1 = arg1.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "price_info",
        "deposit_fee_coins",
        Vec::new(),
        vec![a0, a1],
    );
}
/// Returns: `0xff00000000000001::price_info::PriceInfo`.
pub async fn new_price_info(
    b: &mut PtbBuilder,
    arg0: impl PureU64,
    arg1: impl PureU64,
    arg2: impl super::price_feed::ArgumentPriceFeed,
) -> Argument {
    let a0 = arg0.into_argument(b).await;
    let a1 = arg1.into_argument(b).await;
    let a2 = arg2.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "price_info",
        "new_price_info",
        Vec::new(),
        vec![a0, a1, a2],
    )
}
/// Returns: `0x2::object::ID`.
pub async fn uid_to_inner(
    b: &mut PtbBuilder,
    arg0: impl ArgumentPriceInfoObject,
) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "price_info",
        "uid_to_inner",
        Vec::new(),
        vec![a0],
    )
}
/// Returns: `0xff00000000000001::price_info::PriceInfo`.
pub async fn get_price_info_from_price_info_object(
    b: &mut PtbBuilder,
    arg0: impl ArgumentPriceInfoObject,
) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "price_info",
        "get_price_info_from_price_info_object",
        Vec::new(),
        vec![a0],
    )
}
/// Returns: `0xff00000000000001::price_identifier::PriceIdentifier`.
pub async fn get_price_identifier(
    b: &mut PtbBuilder,
    arg0: impl ArgumentPriceInfo,
) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "price_info",
        "get_price_identifier",
        Vec::new(),
        vec![a0],
    )
}
/// Returns: `&0xff00000000000001::price_feed::PriceFeed`.
pub async fn get_price_feed(
    b: &mut PtbBuilder,
    arg0: impl ArgumentPriceInfo,
) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "price_info",
        "get_price_feed",
        Vec::new(),
        vec![a0],
    )
}
/// Returns: `u64`.
pub async fn get_attestation_time(
    b: &mut PtbBuilder,
    arg0: impl ArgumentPriceInfo,
) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "price_info",
        "get_attestation_time",
        Vec::new(),
        vec![a0],
    )
}
/// Returns: `u64`.
pub async fn get_arrival_time(
    b: &mut PtbBuilder,
    arg0: impl ArgumentPriceInfo,
) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "price_info",
        "get_arrival_time",
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
    pub fn priceinfoobject_tag(&self) -> TypeTag {
        <PriceInfoObject as MoveType>::type_tag_at(self.package)
    }
    pub fn priceinfo_tag(&self) -> TypeTag {
        <PriceInfo as MoveType>::type_tag_at(self.package)
    }
}
