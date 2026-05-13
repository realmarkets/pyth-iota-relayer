#![allow(unused_imports, non_snake_case, non_upper_case_globals)]
//! HEX (Base16) encoding utility.
use move_bindgen_runtime::*;
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;
pub const EInvalidHexLength: u64 = 0u64;
pub const ENotValidHexCharacter: u64 = 1u64;
/// Encode `bytes` in lowercase hex
///
/// Returns: `vector<u8>`.
pub async fn encode(b: &mut PtbBuilder, arg0: impl PureVec<u8>) -> Argument {
    let a0 = arg0.into_argument(b).await;
    b.move_call(b.package_id::<super::Package>(), "hex", "encode", Vec::new(), vec![a0])
}
/// Decode hex into `bytes`
/// Takes a hex string (no 0x prefix) (e.g. b"0f3a")
/// Returns vector of `bytes` that represents the hex string (e.g. x"0f3a")
/// Hex string can be case insensitive (e.g. b"0F3A" and b"0f3a" both return x"0f3a")
/// Aborts if the hex string does not have an even number of characters (as each hex character is 2 characters long)
/// Aborts if the hex string contains non-valid hex characters (valid characters are 0 - 9, a - f, A - F)
///
/// Returns: `vector<u8>`.
pub async fn decode(b: &mut PtbBuilder, arg0: impl PureVec<u8>) -> Argument {
    let a0 = arg0.into_argument(b).await;
    b.move_call(b.package_id::<super::Package>(), "hex", "decode", Vec::new(), vec![a0])
}
