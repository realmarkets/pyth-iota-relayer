#![allow(unused_imports, non_snake_case, non_upper_case_globals)]
use move_bindgen_runtime::*;
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;
/// Please refer to the documentation at https://docs.pyth.network/documentation/pythnet-price-feeds/best-practices for how
/// to how this price safely.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Price {
    pub price: super::i64::I64,
    /// Confidence interval around the price
    pub conf: u64,
    /// The exponent
    pub expo: super::i64::I64,
    /// Unix timestamp of when this price was computed
    pub timestamp: u64,
}
impl MoveType for Price {
    type Package = super::Package;
    const MODULE: &'static str = "price";
    const NAME: &'static str = "Price";
}
impl MoveArg for Price {
    fn pure_bytes(self) -> PureBytes {
        PureBytes(::bcs::to_bytes(&self).expect("bcs serialization failed"))
    }
}
pub trait ArgumentPrice: PTBArgument {
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
impl ArgumentPrice for Price {}
impl ArgumentPrice for Argument {}
/// Returns: `0xff00000000000001::price::Price`.
pub async fn new(
    b: &mut PtbBuilder,
    arg0: impl super::i64::ArgumentI64,
    arg1: impl PureU64,
    arg2: impl super::i64::ArgumentI64,
    arg3: impl PureU64,
) -> Argument {
    let a0 = arg0.into_argument(b).await;
    let a1 = arg1.into_argument(b).await;
    let a2 = arg2.into_argument(b).await;
    let a3 = arg3.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "price",
        "new",
        Vec::new(),
        vec![a0, a1, a2, a3],
    )
}
/// Returns: `0xff00000000000001::i64::I64`.
pub async fn get_price(b: &mut PtbBuilder, arg0: impl ArgumentPrice) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "price",
        "get_price",
        Vec::new(),
        vec![a0],
    )
}
/// Returns: `u64`.
pub async fn get_conf(b: &mut PtbBuilder, arg0: impl ArgumentPrice) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "price",
        "get_conf",
        Vec::new(),
        vec![a0],
    )
}
/// Returns: `u64`.
pub async fn get_timestamp(b: &mut PtbBuilder, arg0: impl ArgumentPrice) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "price",
        "get_timestamp",
        Vec::new(),
        vec![a0],
    )
}
/// Returns: `0xff00000000000001::i64::I64`.
pub async fn get_expo(b: &mut PtbBuilder, arg0: impl ArgumentPrice) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "price",
        "get_expo",
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
    pub fn price_tag(&self) -> TypeTag {
        <Price as MoveType>::type_tag_at(self.package)
    }
}
