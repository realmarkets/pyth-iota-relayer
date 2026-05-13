#![allow(unused_imports, non_snake_case, non_upper_case_globals)]
use move_bindgen_runtime::*;
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;
pub const IDENTIFIER_BYTES_LENGTH: u64 = 32u64;
pub const E_INCORRECT_IDENTIFIER_LENGTH: u64 = 0u64;
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PriceIdentifier {
    pub bytes: Vec<u8>,
}
impl MoveType for PriceIdentifier {
    type Package = super::Package;
    const MODULE: &'static str = "price_identifier";
    const NAME: &'static str = "PriceIdentifier";
}
impl MoveArg for PriceIdentifier {
    fn pure_bytes(self) -> PureBytes {
        PureBytes(::bcs::to_bytes(&self).expect("bcs serialization failed"))
    }
}
pub trait ArgumentPriceIdentifier: PTBArgument {
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
impl ArgumentPriceIdentifier for PriceIdentifier {}
impl ArgumentPriceIdentifier for Argument {}
/// Returns: `0xff00000000000001::price_identifier::PriceIdentifier`.
pub async fn from_byte_vec(b: &mut PtbBuilder, arg0: impl PureVec<u8>) -> Argument {
    let a0 = arg0.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "price_identifier",
        "from_byte_vec",
        Vec::new(),
        vec![a0],
    )
}
/// Returns: `vector<u8>`.
pub async fn get_bytes(
    b: &mut PtbBuilder,
    arg0: impl ArgumentPriceIdentifier,
) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "price_identifier",
        "get_bytes",
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
    pub fn priceidentifier_tag(&self) -> TypeTag {
        <PriceIdentifier as MoveType>::type_tag_at(self.package)
    }
}
