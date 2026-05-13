#![allow(unused_imports, non_snake_case, non_upper_case_globals)]
use move_bindgen_runtime::*;
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;
pub const E_EXPONENT_DOES_NOT_FIT_IN_U8: u64 = 0u64;
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UpdateFee {
    pub mantissa: u64,
    pub exponent: u64,
}
impl MoveType for UpdateFee {
    type Package = super::Package;
    const MODULE: &'static str = "set_update_fee";
    const NAME: &'static str = "UpdateFee";
}
impl MoveArg for UpdateFee {
    fn pure_bytes(self) -> PureBytes {
        PureBytes(::bcs::to_bytes(&self).expect("bcs serialization failed"))
    }
}
pub trait ArgumentUpdateFee: PTBArgument {
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
impl ArgumentUpdateFee for UpdateFee {}
impl ArgumentUpdateFee for Argument {}
/// Read-only handle bound to a runtime package address. Use
/// via `super::Package::at(addr).<module>()` and chain
/// `<type>_tag()` to build a [`TypeTag`] without spinning up
/// a `PtbBuilder`.
pub struct ModuleAt {
    pub(crate) package: Address,
}
impl ModuleAt {
    pub fn updatefee_tag(&self) -> TypeTag {
        <UpdateFee as MoveType>::type_tag_at(self.package)
    }
}
