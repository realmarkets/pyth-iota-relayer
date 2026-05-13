#![allow(unused_imports, non_snake_case, non_upper_case_globals)]
use move_bindgen_runtime::*;
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ConsumedVAAs {
    pub hashes: super::set::Set<super::bytes32::Bytes32>,
}
impl MoveType for ConsumedVAAs {
    type Package = super::Package;
    const MODULE: &'static str = "consumed_vaas";
    const NAME: &'static str = "ConsumedVAAs";
}
impl MoveArg for ConsumedVAAs {
    fn pure_bytes(self) -> PureBytes {
        PureBytes(::bcs::to_bytes(&self).expect("bcs serialization failed"))
    }
}
pub trait ArgumentConsumedVAAs: PTBArgument {
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
impl ArgumentConsumedVAAs for ConsumedVAAs {}
impl ArgumentConsumedVAAs for Argument {}
/// Returns: `0xff00000000000002::consumed_vaas::ConsumedVAAs`.
pub async fn new(b: &mut PtbBuilder) -> Argument {
    b.move_call(
        b.package_id::<super::Package>(),
        "consumed_vaas",
        "new",
        Vec::new(),
        vec![],
    )
}
pub async fn consume(
    b: &mut PtbBuilder,
    arg0: impl ArgumentConsumedVAAs,
    arg1: impl super::bytes32::ArgumentBytes32,
) {
    let a0 = arg0.into_argument_mut(b).await;
    let a1 = arg1.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "consumed_vaas",
        "consume",
        Vec::new(),
        vec![a0, a1],
    );
}
/// Read-only handle bound to a runtime package address. Use
/// via `super::Package::at(addr).<module>()` and chain
/// `<type>_tag()` to build a [`TypeTag`] without spinning up
/// a `PtbBuilder`.
pub struct ModuleAt {
    pub(crate) package: Address,
}
impl ModuleAt {
    pub fn consumedvaas_tag(&self) -> TypeTag {
        <ConsumedVAAs as MoveType>::type_tag_at(self.package)
    }
}
