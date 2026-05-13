#![allow(unused_imports, non_snake_case, non_upper_case_globals)]
use move_bindgen_runtime::*;
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;
/// The length of an address, in bytes
pub const LENGTH: u64 = 32u64;
/// Error from `from_bytes` when it is supplied too many or too few bytes.
pub const EAddressParseError: u64 = 0u64;
/// Returns: `u256`.
pub async fn to_u256(b: &mut PtbBuilder, arg0: impl PureAddress) -> Argument {
    let a0 = arg0.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "address",
        "to_u256",
        Vec::new(),
        vec![a0],
    )
}
/// Returns: `address`.
pub async fn from_u256(b: &mut PtbBuilder, arg0: impl PureU256) -> Argument {
    let a0 = arg0.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "address",
        "from_u256",
        Vec::new(),
        vec![a0],
    )
}
/// Returns: `address`.
pub async fn from_bytes(b: &mut PtbBuilder, arg0: impl PureVec<u8>) -> Argument {
    let a0 = arg0.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "address",
        "from_bytes",
        Vec::new(),
        vec![a0],
    )
}
/// Convert `a` into BCS-encoded bytes.
///
/// Returns: `vector<u8>`.
pub async fn to_bytes(b: &mut PtbBuilder, arg0: impl PureAddress) -> Argument {
    let a0 = arg0.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "address",
        "to_bytes",
        Vec::new(),
        vec![a0],
    )
}
/// Convert `a` to a hex-encoded ASCII string
///
/// Returns: `0x1::ascii::String`.
pub async fn to_ascii_string(b: &mut PtbBuilder, arg0: impl PureAddress) -> Argument {
    let a0 = arg0.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "address",
        "to_ascii_string",
        Vec::new(),
        vec![a0],
    )
}
/// Convert `a` to a hex-encoded string
///
/// Returns: `0x1::string::String`.
pub async fn to_string(b: &mut PtbBuilder, arg0: impl PureAddress) -> Argument {
    let a0 = arg0.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "address",
        "to_string",
        Vec::new(),
        vec![a0],
    )
}
/// Converts an ASCII string to an address, taking the numerical value for each character. The
/// string must be Base16 encoded, and thus exactly 64 characters long.
/// For example, the string "00000000000000000000000000000000000000000000000000000000DEADB33F"
/// will be converted to the address @0xDEADB33F.
/// Aborts with `EAddressParseError` if the length of `s` is not 64,
/// or if an invalid character is encountered.
///
/// Returns: `address`.
pub async fn from_ascii_bytes(b: &mut PtbBuilder, arg0: impl PureVec<u8>) -> Argument {
    let a0 = arg0.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "address",
        "from_ascii_bytes",
        Vec::new(),
        vec![a0],
    )
}
/// Length of an IOTA address in bytes
///
/// Returns: `u64`.
pub async fn length(b: &mut PtbBuilder) -> Argument {
    b.move_call(
        b.package_id::<super::Package>(),
        "address",
        "length",
        Vec::new(),
        vec![],
    )
}
/// Largest possible address
///
/// Returns: `u256`.
pub async fn max(b: &mut PtbBuilder) -> Argument {
    b.move_call(b.package_id::<super::Package>(), "address", "max", Vec::new(), vec![])
}
