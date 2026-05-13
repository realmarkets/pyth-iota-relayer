#![allow(unused_imports, non_snake_case, non_upper_case_globals)]
use move_bindgen_runtime::*;
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;
pub const E_INVALID_BYTES20: u64 = 0u64;
pub const E_CANNOT_TRIM_NONZERO: u64 = 1u64;
pub const LEN: u64 = 20u64;
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Bytes20 {
    pub data: Vec<u8>,
}
impl MoveType for Bytes20 {
    type Package = super::Package;
    const MODULE: &'static str = "bytes20";
    const NAME: &'static str = "Bytes20";
}
impl MoveArg for Bytes20 {
    fn pure_bytes(self) -> PureBytes {
        PureBytes(::bcs::to_bytes(&self).expect("bcs serialization failed"))
    }
}
pub trait ArgumentBytes20: PTBArgument {
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
impl ArgumentBytes20 for Bytes20 {}
impl ArgumentBytes20 for Argument {}
/// Returns: `u64`.
pub async fn length(b: &mut PtbBuilder) -> Argument {
    b.move_call(
        b.package_id::<super::Package>(),
        "bytes20",
        "length",
        Vec::new(),
        vec![],
    )
}
/// Returns: `0xff00000000000002::bytes20::Bytes20`.
pub async fn new(b: &mut PtbBuilder, arg0: impl PureVec<u8>) -> Argument {
    let a0 = arg0.into_argument(b).await;
    b.move_call(b.package_id::<super::Package>(), "bytes20", "new", Vec::new(), vec![a0])
}
/// Returns: `0xff00000000000002::bytes20::Bytes20`.
pub async fn default(b: &mut PtbBuilder) -> Argument {
    b.move_call(
        b.package_id::<super::Package>(),
        "bytes20",
        "default",
        Vec::new(),
        vec![],
    )
}
/// Returns: `vector<u8>`.
pub async fn data(b: &mut PtbBuilder, arg0: impl ArgumentBytes20) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "bytes20",
        "data",
        Vec::new(),
        vec![a0],
    )
}
/// Returns: `0xff00000000000002::bytes20::Bytes20`.
pub async fn from_bytes(b: &mut PtbBuilder, arg0: impl PureVec<u8>) -> Argument {
    let a0 = arg0.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "bytes20",
        "from_bytes",
        Vec::new(),
        vec![a0],
    )
}
/// Returns: `vector<u8>`.
pub async fn to_bytes(b: &mut PtbBuilder, arg0: impl ArgumentBytes20) -> Argument {
    let a0 = arg0.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "bytes20",
        "to_bytes",
        Vec::new(),
        vec![a0],
    )
}
/// Returns: `0xff00000000000002::bytes20::Bytes20`.
pub async fn take(
    b: &mut PtbBuilder,
    arg0: impl super::cursor::ArgumentCursor<u8>,
) -> Argument {
    let a0 = arg0.into_argument_mut(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "bytes20",
        "take",
        Vec::new(),
        vec![a0],
    )
}
/// Returns: `bool`.
pub async fn is_nonzero(b: &mut PtbBuilder, arg0: impl ArgumentBytes20) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "bytes20",
        "is_nonzero",
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
    pub fn bytes20_tag(&self) -> TypeTag {
        <Bytes20 as MoveType>::type_tag_at(self.package)
    }
}
