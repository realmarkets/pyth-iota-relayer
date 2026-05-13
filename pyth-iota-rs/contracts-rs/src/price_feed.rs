#![allow(unused_imports, non_snake_case, non_upper_case_globals)]
use move_bindgen_runtime::*;
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;
/// PriceFeed represents a current aggregate price for a particular product.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PriceFeed {
    /// The price identifier
    pub price_identifier: super::price_identifier::PriceIdentifier,
    /// The current aggregate price
    pub price: super::price::Price,
    /// The current exponentially moving average aggregate price
    pub ema_price: super::price::Price,
}
impl MoveType for PriceFeed {
    type Package = super::Package;
    const MODULE: &'static str = "price_feed";
    const NAME: &'static str = "PriceFeed";
}
impl MoveArg for PriceFeed {
    fn pure_bytes(self) -> PureBytes {
        PureBytes(::bcs::to_bytes(&self).expect("bcs serialization failed"))
    }
}
pub trait ArgumentPriceFeed: PTBArgument {
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
impl ArgumentPriceFeed for PriceFeed {}
impl ArgumentPriceFeed for Argument {}
/// Returns: `0xff00000000000001::price_feed::PriceFeed`.
pub async fn new(
    b: &mut PtbBuilder,
    arg0: impl super::price_identifier::ArgumentPriceIdentifier,
    arg1: impl super::price::ArgumentPrice,
    arg2: impl super::price::ArgumentPrice,
) -> Argument {
    let a0 = arg0.into_argument(b).await;
    let a1 = arg1.into_argument(b).await;
    let a2 = arg2.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "price_feed",
        "new",
        Vec::new(),
        vec![a0, a1, a2],
    )
}
/// Returns: `0xff00000000000001::price_feed::PriceFeed`.
pub async fn from(b: &mut PtbBuilder, arg0: impl ArgumentPriceFeed) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "price_feed",
        "from",
        Vec::new(),
        vec![a0],
    )
}
/// Returns: `0xff00000000000001::price_identifier::PriceIdentifier`.
pub async fn get_price_identifier(
    b: &mut PtbBuilder,
    arg0: impl ArgumentPriceFeed,
) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "price_feed",
        "get_price_identifier",
        Vec::new(),
        vec![a0],
    )
}
/// Returns: `0xff00000000000001::price::Price`.
pub async fn get_price(b: &mut PtbBuilder, arg0: impl ArgumentPriceFeed) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "price_feed",
        "get_price",
        Vec::new(),
        vec![a0],
    )
}
/// Returns: `0xff00000000000001::price::Price`.
pub async fn get_ema_price(
    b: &mut PtbBuilder,
    arg0: impl ArgumentPriceFeed,
) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "price_feed",
        "get_ema_price",
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
    pub fn pricefeed_tag(&self) -> TypeTag {
        <PriceFeed as MoveType>::type_tag_at(self.package)
    }
}
