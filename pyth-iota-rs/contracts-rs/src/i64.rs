#![allow(unused_imports, non_snake_case, non_upper_case_globals)]
use move_bindgen_runtime::*;
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;
pub const MAX_POSITIVE_MAGNITUDE: u64 = 9223372036854775807u64;
pub const MAX_NEGATIVE_MAGNITUDE: u64 = 9223372036854775808u64;
/// As Move does not support negative numbers natively, we use our own internal
/// representation.
///
/// To consume these values, first call `get_is_negative()` to determine if the I64
/// represents a negative or positive value. Then call `get_magnitude_if_positive()` or
/// `get_magnitude_if_negative()` to get the magnitude of the number in unsigned u64 format.
/// This API forces consumers to handle positive and negative numbers safely.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct I64 {
    pub negative: bool,
    pub magnitude: u64,
}
impl MoveType for I64 {
    type Package = super::Package;
    const MODULE: &'static str = "i64";
    const NAME: &'static str = "I64";
}
impl MoveArg for I64 {
    fn pure_bytes(self) -> PureBytes {
        PureBytes(::bcs::to_bytes(&self).expect("bcs serialization failed"))
    }
}
pub trait ArgumentI64: PTBArgument {
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
impl ArgumentI64 for I64 {}
impl ArgumentI64 for Argument {}
/// Returns: `0xff00000000000001::i64::I64`.
pub async fn new(
    b: &mut PtbBuilder,
    arg0: impl PureU64,
    arg1: impl PureBool,
) -> Argument {
    let a0 = arg0.into_argument(b).await;
    let a1 = arg1.into_argument(b).await;
    b.move_call(b.package_id::<super::Package>(), "i64", "new", Vec::new(), vec![a0, a1])
}
/// Returns: `bool`.
pub async fn get_is_negative(b: &mut PtbBuilder, arg0: impl ArgumentI64) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "i64",
        "get_is_negative",
        Vec::new(),
        vec![a0],
    )
}
/// Returns: `u64`.
pub async fn get_magnitude_if_positive(
    b: &mut PtbBuilder,
    arg0: impl ArgumentI64,
) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "i64",
        "get_magnitude_if_positive",
        Vec::new(),
        vec![a0],
    )
}
/// Returns: `u64`.
pub async fn get_magnitude_if_negative(
    b: &mut PtbBuilder,
    arg0: impl ArgumentI64,
) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "i64",
        "get_magnitude_if_negative",
        Vec::new(),
        vec![a0],
    )
}
/// Returns: `0xff00000000000001::i64::I64`.
pub async fn from_u64(b: &mut PtbBuilder, arg0: impl PureU64) -> Argument {
    let a0 = arg0.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "i64",
        "from_u64",
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
    pub fn i64_tag(&self) -> TypeTag {
        <I64 as MoveType>::type_tag_at(self.package)
    }
}
