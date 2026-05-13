#![allow(unused_imports, non_snake_case, non_upper_case_globals)]
use move_bindgen_runtime::*;
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PythInitializationEvent {
    pub dummy_field: bool,
}
impl MoveType for PythInitializationEvent {
    type Package = super::Package;
    const MODULE: &'static str = "event";
    const NAME: &'static str = "PythInitializationEvent";
}
impl MoveArg for PythInitializationEvent {
    fn pure_bytes(self) -> PureBytes {
        PureBytes(::bcs::to_bytes(&self).expect("bcs serialization failed"))
    }
}
pub trait ArgumentPythInitializationEvent: PTBArgument {
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
impl ArgumentPythInitializationEvent for PythInitializationEvent {}
impl ArgumentPythInitializationEvent for Argument {}
/// Signifies that a price feed has been updated
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PriceFeedUpdateEvent {
    /// Value of the price feed
    pub price_feed: super::price_feed::PriceFeed,
    /// Timestamp of the update
    pub timestamp: u64,
}
impl MoveType for PriceFeedUpdateEvent {
    type Package = super::Package;
    const MODULE: &'static str = "event";
    const NAME: &'static str = "PriceFeedUpdateEvent";
}
impl MoveArg for PriceFeedUpdateEvent {
    fn pure_bytes(self) -> PureBytes {
        PureBytes(::bcs::to_bytes(&self).expect("bcs serialization failed"))
    }
}
pub trait ArgumentPriceFeedUpdateEvent: PTBArgument {
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
impl ArgumentPriceFeedUpdateEvent for PriceFeedUpdateEvent {}
impl ArgumentPriceFeedUpdateEvent for Argument {}
/// Read-only handle bound to a runtime package address. Use
/// via `super::Package::at(addr).<module>()` and chain
/// `<type>_tag()` to build a [`TypeTag`] without spinning up
/// a `PtbBuilder`.
pub struct ModuleAt {
    pub(crate) package: Address,
}
impl ModuleAt {
    pub fn pythinitializationevent_tag(&self) -> TypeTag {
        <PythInitializationEvent as MoveType>::type_tag_at(self.package)
    }
    pub fn pricefeedupdateevent_tag(&self) -> TypeTag {
        <PriceFeedUpdateEvent as MoveType>::type_tag_at(self.package)
    }
}
