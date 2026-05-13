#![allow(unused_imports, non_snake_case, non_upper_case_globals)]
use move_bindgen_runtime::*;
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;
/// The price feed is not currently updating for an unknown reason.
pub const UNKNOWN: u64 = 0u64;
/// The price feed is updating as expected.
pub const TRADING: u64 = 1u64;
/// PriceStatus represents the availability status of a price feed.
/// Prices should only be used if they have a status of trading.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PriceStatus {
    pub status: u64,
}
impl MoveType for PriceStatus {
    type Package = super::Package;
    const MODULE: &'static str = "price_status";
    const NAME: &'static str = "PriceStatus";
}
impl MoveArg for PriceStatus {
    fn pure_bytes(self) -> PureBytes {
        PureBytes(::bcs::to_bytes(&self).expect("bcs serialization failed"))
    }
}
pub trait ArgumentPriceStatus: PTBArgument {
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
impl ArgumentPriceStatus for PriceStatus {}
impl ArgumentPriceStatus for Argument {}
/// Returns: `0xff00000000000001::price_status::PriceStatus`.
pub async fn from_u64(b: &mut PtbBuilder, arg0: impl PureU64) -> Argument {
    let a0 = arg0.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "price_status",
        "from_u64",
        Vec::new(),
        vec![a0],
    )
}
/// Returns: `u64`.
pub async fn get_status(b: &mut PtbBuilder, arg0: impl ArgumentPriceStatus) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "price_status",
        "get_status",
        Vec::new(),
        vec![a0],
    )
}
/// Returns: `0xff00000000000001::price_status::PriceStatus`.
pub async fn new_unknown(b: &mut PtbBuilder) -> Argument {
    b.move_call(
        b.package_id::<super::Package>(),
        "price_status",
        "new_unknown",
        Vec::new(),
        vec![],
    )
}
/// Returns: `0xff00000000000001::price_status::PriceStatus`.
pub async fn new_trading(b: &mut PtbBuilder) -> Argument {
    b.move_call(
        b.package_id::<super::Package>(),
        "price_status",
        "new_trading",
        Vec::new(),
        vec![],
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
    pub fn pricestatus_tag(&self) -> TypeTag {
        <PriceStatus as MoveType>::type_tag_at(self.package)
    }
}
