#![allow(unused_imports, non_snake_case, non_upper_case_globals)]
//! Functionality for converting Move types into values. Use with care!
use move_bindgen_runtime::*;
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;
/// ASCII Character code for the `:` (colon) symbol.
pub const ASCII_COLON: u8 = 58u8;
/// ASCII Character code for the `v` (lowercase v) symbol.
pub const ASCII_V: u8 = 118u8;
/// ASCII Character code for the `e` (lowercase e) symbol.
pub const ASCII_E: u8 = 101u8;
/// ASCII Character code for the `c` (lowercase c) symbol.
pub const ASCII_C: u8 = 99u8;
/// ASCII Character code for the `t` (lowercase t) symbol.
pub const ASCII_T: u8 = 116u8;
/// ASCII Character code for the `o` (lowercase o) symbol.
pub const ASCII_O: u8 = 111u8;
/// ASCII Character code for the `r` (lowercase r) symbol.
pub const ASCII_R: u8 = 114u8;
/// The type is not from a package/module. It is a primitive type.
pub const ENonModuleType: u64 = 0u64;
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TypeName {
    /// String representation of the type. All types are represented
    /// using their source syntax:
    /// "u8", "u64", "bool", "address", "vector", and so on for primitive types.
    /// Struct types are represented as fully qualified type names; e.g.
    /// `00000000000000000000000000000001::string::String` or
    /// `0000000000000000000000000000000a::module_name1::type_name1<0000000000000000000000000000000a::module_name2::type_name2<u64>>`
    /// Addresses are hex-encoded lowercase values of length ADDRESS_LENGTH (16, 20, or 32 depending on the Move platform)
    pub name: String,
}
impl MoveType for TypeName {
    type Package = super::Package;
    const MODULE: &'static str = "type_name";
    const NAME: &'static str = "TypeName";
}
impl MoveArg for TypeName {
    fn pure_bytes(self) -> PureBytes {
        PureBytes(::bcs::to_bytes(&self).expect("bcs serialization failed"))
    }
}
pub trait ArgumentTypeName: PTBArgument {
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
impl ArgumentTypeName for TypeName {}
impl ArgumentTypeName for Argument {}
/// Returns: `0x1::type_name::TypeName`.
pub async fn get<T0: MoveType>(b: &mut PtbBuilder) -> Argument {
    b.move_call(
        b.package_id::<super::Package>(),
        "type_name",
        "get",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![],
    )
}
/// Returns: `0x1::type_name::TypeName`.
pub async fn get_with_original_ids<T0: MoveType>(b: &mut PtbBuilder) -> Argument {
    b.move_call(
        b.package_id::<super::Package>(),
        "type_name",
        "get_with_original_ids",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![],
    )
}
/// Returns true iff the TypeName represents a primitive type, i.e. one of
/// u8, u16, u32, u64, u128, u256, bool, address, vector.
///
/// Returns: `bool`.
pub async fn is_primitive(b: &mut PtbBuilder, arg0: impl ArgumentTypeName) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "type_name",
        "is_primitive",
        Vec::new(),
        vec![a0],
    )
}
/// Get the String representation of `self`
///
/// Returns: `&0x1::ascii::String`.
pub async fn borrow_string(b: &mut PtbBuilder, arg0: impl ArgumentTypeName) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "type_name",
        "borrow_string",
        Vec::new(),
        vec![a0],
    )
}
/// Get Address string (Base16 encoded), first part of the TypeName.
/// Aborts if given a primitive type.
///
/// Returns: `0x1::ascii::String`.
pub async fn get_address(b: &mut PtbBuilder, arg0: impl ArgumentTypeName) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "type_name",
        "get_address",
        Vec::new(),
        vec![a0],
    )
}
/// Get name of the module.
/// Aborts if given a primitive type.
///
/// Returns: `0x1::ascii::String`.
pub async fn get_module(b: &mut PtbBuilder, arg0: impl ArgumentTypeName) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "type_name",
        "get_module",
        Vec::new(),
        vec![a0],
    )
}
/// Convert `self` into its inner String
///
/// Returns: `0x1::ascii::String`.
pub async fn into_string(b: &mut PtbBuilder, arg0: impl ArgumentTypeName) -> Argument {
    let a0 = arg0.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "type_name",
        "into_string",
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
    pub fn typename_tag(&self) -> TypeTag {
        <TypeName as MoveType>::type_tag_at(self.package)
    }
}
