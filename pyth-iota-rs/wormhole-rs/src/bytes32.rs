#![allow(unused_imports, non_snake_case, non_upper_case_globals)]
use move_bindgen_runtime::*;
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;
pub const E_INVALID_BYTES32: u64 = 0u64;
pub const E_CANNOT_TRIM_NONZERO: u64 = 1u64;
pub const E_U64_OVERFLOW: u64 = 2u64;
pub const LEN: u64 = 32u64;
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Bytes32 {
    pub data: Vec<u8>,
}
impl MoveType for Bytes32 {
    type Package = super::Package;
    const MODULE: &'static str = "bytes32";
    const NAME: &'static str = "Bytes32";
}
impl MoveArg for Bytes32 {
    fn pure_bytes(self) -> PureBytes {
        PureBytes(::bcs::to_bytes(&self).expect("bcs serialization failed"))
    }
}
pub trait ArgumentBytes32: PTBArgument {
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
impl ArgumentBytes32 for Bytes32 {}
impl ArgumentBytes32 for Argument {}
/// Returns: `u64`.
pub async fn length(b: &mut PtbBuilder) -> Argument {
    b.move_call(
        b.package_id::<super::Package>(),
        "bytes32",
        "length",
        Vec::new(),
        vec![],
    )
}
/// Returns: `0xff00000000000002::bytes32::Bytes32`.
pub async fn new(b: &mut PtbBuilder, arg0: impl PureVec<u8>) -> Argument {
    let a0 = arg0.into_argument(b).await;
    b.move_call(b.package_id::<super::Package>(), "bytes32", "new", Vec::new(), vec![a0])
}
/// Returns: `0xff00000000000002::bytes32::Bytes32`.
pub async fn default(b: &mut PtbBuilder) -> Argument {
    b.move_call(
        b.package_id::<super::Package>(),
        "bytes32",
        "default",
        Vec::new(),
        vec![],
    )
}
/// Returns: `vector<u8>`.
pub async fn data(b: &mut PtbBuilder, arg0: impl ArgumentBytes32) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "bytes32",
        "data",
        Vec::new(),
        vec![a0],
    )
}
/// Returns: `0xff00000000000002::bytes32::Bytes32`.
pub async fn from_u256_be(b: &mut PtbBuilder, arg0: impl PureU256) -> Argument {
    let a0 = arg0.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "bytes32",
        "from_u256_be",
        Vec::new(),
        vec![a0],
    )
}
/// Returns: `u256`.
pub async fn to_u256_be(b: &mut PtbBuilder, arg0: impl ArgumentBytes32) -> Argument {
    let a0 = arg0.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "bytes32",
        "to_u256_be",
        Vec::new(),
        vec![a0],
    )
}
/// Returns: `0xff00000000000002::bytes32::Bytes32`.
pub async fn from_u64_be(b: &mut PtbBuilder, arg0: impl PureU64) -> Argument {
    let a0 = arg0.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "bytes32",
        "from_u64_be",
        Vec::new(),
        vec![a0],
    )
}
/// Returns: `u64`.
pub async fn to_u64_be(b: &mut PtbBuilder, arg0: impl ArgumentBytes32) -> Argument {
    let a0 = arg0.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "bytes32",
        "to_u64_be",
        Vec::new(),
        vec![a0],
    )
}
/// Returns: `0xff00000000000002::bytes32::Bytes32`.
pub async fn from_bytes(b: &mut PtbBuilder, arg0: impl PureVec<u8>) -> Argument {
    let a0 = arg0.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "bytes32",
        "from_bytes",
        Vec::new(),
        vec![a0],
    )
}
/// Returns: `vector<u8>`.
pub async fn to_bytes(b: &mut PtbBuilder, arg0: impl ArgumentBytes32) -> Argument {
    let a0 = arg0.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "bytes32",
        "to_bytes",
        Vec::new(),
        vec![a0],
    )
}
/// Returns: `0xff00000000000002::bytes32::Bytes32`.
pub async fn take_bytes(
    b: &mut PtbBuilder,
    arg0: impl super::cursor::ArgumentCursor<u8>,
) -> Argument {
    let a0 = arg0.into_argument_mut(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "bytes32",
        "take_bytes",
        Vec::new(),
        vec![a0],
    )
}
/// Returns: `address`.
pub async fn to_address(b: &mut PtbBuilder, arg0: impl ArgumentBytes32) -> Argument {
    let a0 = arg0.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "bytes32",
        "to_address",
        Vec::new(),
        vec![a0],
    )
}
/// Returns: `0xff00000000000002::bytes32::Bytes32`.
pub async fn from_address(b: &mut PtbBuilder, arg0: impl PureAddress) -> Argument {
    let a0 = arg0.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "bytes32",
        "from_address",
        Vec::new(),
        vec![a0],
    )
}
/// Returns: `0xff00000000000002::bytes32::Bytes32`.
pub async fn from_utf8(b: &mut PtbBuilder, arg0: impl PureString) -> Argument {
    let a0 = arg0.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "bytes32",
        "from_utf8",
        Vec::new(),
        vec![a0],
    )
}
/// Returns: `0x1::string::String`.
pub async fn to_utf8(b: &mut PtbBuilder, arg0: impl ArgumentBytes32) -> Argument {
    let a0 = arg0.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "bytes32",
        "to_utf8",
        Vec::new(),
        vec![a0],
    )
}
/// Returns: `bool`.
pub async fn is_nonzero(b: &mut PtbBuilder, arg0: impl ArgumentBytes32) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "bytes32",
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
    pub fn bytes32_tag(&self) -> TypeTag {
        <Bytes32 as MoveType>::type_tag_at(self.package)
    }
}
