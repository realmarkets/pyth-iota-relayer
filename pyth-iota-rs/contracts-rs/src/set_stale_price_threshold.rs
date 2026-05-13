#![allow(unused_imports, non_snake_case, non_upper_case_globals)]
use move_bindgen_runtime::*;
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StalePriceThreshold {
    pub threshold: u64,
}
impl MoveType for StalePriceThreshold {
    type Package = super::Package;
    const MODULE: &'static str = "set_stale_price_threshold";
    const NAME: &'static str = "StalePriceThreshold";
}
impl MoveArg for StalePriceThreshold {
    fn pure_bytes(self) -> PureBytes {
        PureBytes(::bcs::to_bytes(&self).expect("bcs serialization failed"))
    }
}
pub trait ArgumentStalePriceThreshold: PTBArgument {
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
impl ArgumentStalePriceThreshold for StalePriceThreshold {}
impl ArgumentStalePriceThreshold for Argument {}
/// Read-only handle bound to a runtime package address. Use
/// via `super::Package::at(addr).<module>()` and chain
/// `<type>_tag()` to build a [`TypeTag`] without spinning up
/// a `PtbBuilder`.
pub struct ModuleAt {
    pub(crate) package: Address,
}
impl ModuleAt {
    pub fn stalepricethreshold_tag(&self) -> TypeTag {
        <StalePriceThreshold as MoveType>::type_tag_at(self.package)
    }
}
