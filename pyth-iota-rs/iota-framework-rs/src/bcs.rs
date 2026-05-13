#![allow(unused_imports, non_snake_case, non_upper_case_globals)]
//! This module implements BCS (de)serialization in Move.
//! Full specification can be found here: https://github.com/diem/bcs
//!
//! Short summary (for Move-supported types):
//!
//! - address - sequence of X bytes
//! - bool - byte with 0 or 1
//! - u8 - a single u8 byte
//! - u16 / u32 / u64 / u128 / u256 - LE bytes
//! - vector - ULEB128 length + LEN elements
//! - option - first byte bool: None (0) or Some (1), then value
//!
//! Usage example:
//! ```
//! /// This function reads u8 and u64 value from the input
//! /// and returns the rest of the bytes.
//! fun deserialize(bytes: vector<u8>): (u8, u64, vector<u8>) {
//!     use iota::bcs::{Self, BCS};
//!
//!     let prepared: BCS = bcs::new(bytes);
//!     let (u8_value, u64_value) = (
//!         prepared.peel_u8(),
//!         prepared.peel_u64()
//!     );
//!
//!     // unpack bcs struct
//!     let leftovers = prepared.into_remainder_bytes();
//!
//!     (u8_value, u64_value, leftovers)
//! }
//! ```
use move_bindgen_runtime::*;
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;
/// For when bytes length is less than required for deserialization.
pub const EOutOfRange: u64 = 0u64;
/// For when the boolean value different than `0` or `1`.
pub const ENotBool: u64 = 1u64;
/// For when ULEB byte is out of range (or not found).
pub const ELenOutOfRange: u64 = 2u64;
/// A helper struct that saves resources on operations. For better
/// vector performance, it stores reversed bytes of the BCS and
/// enables use of `vector::pop_back`.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BCS {
    pub bytes: Vec<u8>,
}
impl MoveType for BCS {
    type Package = super::Package;
    const MODULE: &'static str = "bcs";
    const NAME: &'static str = "BCS";
}
impl MoveArg for BCS {
    fn pure_bytes(self) -> PureBytes {
        PureBytes(::bcs::to_bytes(&self).expect("bcs serialization failed"))
    }
}
pub trait ArgumentBCS: PTBArgument {
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
impl ArgumentBCS for BCS {}
impl ArgumentBCS for Argument {}
/// Get BCS serialized bytes for any value.
/// Re-exports stdlib `bcs::to_bytes`.
///
/// Returns: `vector<u8>`.
pub async fn to_bytes<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentObject<()>,
) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "bcs",
        "to_bytes",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![a0],
    )
}
/// Creates a new instance of BCS wrapper that holds inversed
/// bytes for better performance.
///
/// Returns: `0x2::bcs::BCS`.
pub async fn new(b: &mut PtbBuilder, arg0: impl PureVec<u8>) -> Argument {
    let a0 = arg0.into_argument(b).await;
    b.move_call(b.package_id::<super::Package>(), "bcs", "new", Vec::new(), vec![a0])
}
/// Unpack the `BCS` struct returning the leftover bytes.
/// Useful for passing the data further after partial deserialization.
///
/// Returns: `vector<u8>`.
pub async fn into_remainder_bytes(
    b: &mut PtbBuilder,
    arg0: impl ArgumentBCS,
) -> Argument {
    let a0 = arg0.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "bcs",
        "into_remainder_bytes",
        Vec::new(),
        vec![a0],
    )
}
/// Read address from the bcs-serialized bytes.
///
/// Returns: `address`.
pub async fn peel_address(b: &mut PtbBuilder, arg0: impl ArgumentBCS) -> Argument {
    let a0 = arg0.into_argument_mut(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "bcs",
        "peel_address",
        Vec::new(),
        vec![a0],
    )
}
/// Read a `bool` value from bcs-serialized bytes.
///
/// Returns: `bool`.
pub async fn peel_bool(b: &mut PtbBuilder, arg0: impl ArgumentBCS) -> Argument {
    let a0 = arg0.into_argument_mut(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "bcs",
        "peel_bool",
        Vec::new(),
        vec![a0],
    )
}
/// Read `u8` value from bcs-serialized bytes.
///
/// Returns: `u8`.
pub async fn peel_u8(b: &mut PtbBuilder, arg0: impl ArgumentBCS) -> Argument {
    let a0 = arg0.into_argument_mut(b).await;
    b.move_call(b.package_id::<super::Package>(), "bcs", "peel_u8", Vec::new(), vec![a0])
}
/// Read `u16` value from bcs-serialized bytes.
///
/// Returns: `u16`.
pub async fn peel_u16(b: &mut PtbBuilder, arg0: impl ArgumentBCS) -> Argument {
    let a0 = arg0.into_argument_mut(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "bcs",
        "peel_u16",
        Vec::new(),
        vec![a0],
    )
}
/// Read `u32` value from bcs-serialized bytes.
///
/// Returns: `u32`.
pub async fn peel_u32(b: &mut PtbBuilder, arg0: impl ArgumentBCS) -> Argument {
    let a0 = arg0.into_argument_mut(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "bcs",
        "peel_u32",
        Vec::new(),
        vec![a0],
    )
}
/// Read `u64` value from bcs-serialized bytes.
///
/// Returns: `u64`.
pub async fn peel_u64(b: &mut PtbBuilder, arg0: impl ArgumentBCS) -> Argument {
    let a0 = arg0.into_argument_mut(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "bcs",
        "peel_u64",
        Vec::new(),
        vec![a0],
    )
}
/// Read `u128` value from bcs-serialized bytes.
///
/// Returns: `u128`.
pub async fn peel_u128(b: &mut PtbBuilder, arg0: impl ArgumentBCS) -> Argument {
    let a0 = arg0.into_argument_mut(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "bcs",
        "peel_u128",
        Vec::new(),
        vec![a0],
    )
}
/// Read `u256` value from bcs-serialized bytes.
///
/// Returns: `u256`.
pub async fn peel_u256(b: &mut PtbBuilder, arg0: impl ArgumentBCS) -> Argument {
    let a0 = arg0.into_argument_mut(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "bcs",
        "peel_u256",
        Vec::new(),
        vec![a0],
    )
}
/// Read ULEB bytes expecting a vector length. Result should
/// then be used to perform `peel_*` operation LEN times.
///
/// In BCS `vector` length is implemented with ULEB128;
/// See more here: https://en.wikipedia.org/wiki/LEB128
///
/// Returns: `u64`.
pub async fn peel_vec_length(b: &mut PtbBuilder, arg0: impl ArgumentBCS) -> Argument {
    let a0 = arg0.into_argument_mut(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "bcs",
        "peel_vec_length",
        Vec::new(),
        vec![a0],
    )
}
/// Peel a vector of `address` from serialized bytes.
///
/// Returns: `vector<address>`.
pub async fn peel_vec_address(b: &mut PtbBuilder, arg0: impl ArgumentBCS) -> Argument {
    let a0 = arg0.into_argument_mut(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "bcs",
        "peel_vec_address",
        Vec::new(),
        vec![a0],
    )
}
/// Peel a vector of `address` from serialized bytes.
///
/// Returns: `vector<bool>`.
pub async fn peel_vec_bool(b: &mut PtbBuilder, arg0: impl ArgumentBCS) -> Argument {
    let a0 = arg0.into_argument_mut(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "bcs",
        "peel_vec_bool",
        Vec::new(),
        vec![a0],
    )
}
/// Peel a vector of `u8` (eg string) from serialized bytes.
///
/// Returns: `vector<u8>`.
pub async fn peel_vec_u8(b: &mut PtbBuilder, arg0: impl ArgumentBCS) -> Argument {
    let a0 = arg0.into_argument_mut(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "bcs",
        "peel_vec_u8",
        Vec::new(),
        vec![a0],
    )
}
/// Peel a `vector<vector<u8>>` (eg vec of string) from serialized bytes.
///
/// Returns: `vector<vector<u8>>`.
pub async fn peel_vec_vec_u8(b: &mut PtbBuilder, arg0: impl ArgumentBCS) -> Argument {
    let a0 = arg0.into_argument_mut(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "bcs",
        "peel_vec_vec_u8",
        Vec::new(),
        vec![a0],
    )
}
/// Peel a vector of `u16` from serialized bytes.
///
/// Returns: `vector<u16>`.
pub async fn peel_vec_u16(b: &mut PtbBuilder, arg0: impl ArgumentBCS) -> Argument {
    let a0 = arg0.into_argument_mut(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "bcs",
        "peel_vec_u16",
        Vec::new(),
        vec![a0],
    )
}
/// Peel a vector of `u32` from serialized bytes.
///
/// Returns: `vector<u32>`.
pub async fn peel_vec_u32(b: &mut PtbBuilder, arg0: impl ArgumentBCS) -> Argument {
    let a0 = arg0.into_argument_mut(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "bcs",
        "peel_vec_u32",
        Vec::new(),
        vec![a0],
    )
}
/// Peel a vector of `u64` from serialized bytes.
///
/// Returns: `vector<u64>`.
pub async fn peel_vec_u64(b: &mut PtbBuilder, arg0: impl ArgumentBCS) -> Argument {
    let a0 = arg0.into_argument_mut(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "bcs",
        "peel_vec_u64",
        Vec::new(),
        vec![a0],
    )
}
/// Peel a vector of `u128` from serialized bytes.
///
/// Returns: `vector<u128>`.
pub async fn peel_vec_u128(b: &mut PtbBuilder, arg0: impl ArgumentBCS) -> Argument {
    let a0 = arg0.into_argument_mut(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "bcs",
        "peel_vec_u128",
        Vec::new(),
        vec![a0],
    )
}
/// Peel a vector of `u256` from serialized bytes.
///
/// Returns: `vector<u256>`.
pub async fn peel_vec_u256(b: &mut PtbBuilder, arg0: impl ArgumentBCS) -> Argument {
    let a0 = arg0.into_argument_mut(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "bcs",
        "peel_vec_u256",
        Vec::new(),
        vec![a0],
    )
}
/// Peel `Option<address>` from serialized bytes.
///
/// Returns: `0x1::option::Option<address>`.
pub async fn peel_option_address(
    b: &mut PtbBuilder,
    arg0: impl ArgumentBCS,
) -> Argument {
    let a0 = arg0.into_argument_mut(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "bcs",
        "peel_option_address",
        Vec::new(),
        vec![a0],
    )
}
/// Peel `Option<bool>` from serialized bytes.
///
/// Returns: `0x1::option::Option<bool>`.
pub async fn peel_option_bool(b: &mut PtbBuilder, arg0: impl ArgumentBCS) -> Argument {
    let a0 = arg0.into_argument_mut(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "bcs",
        "peel_option_bool",
        Vec::new(),
        vec![a0],
    )
}
/// Peel `Option<u8>` from serialized bytes.
///
/// Returns: `0x1::option::Option<u8>`.
pub async fn peel_option_u8(b: &mut PtbBuilder, arg0: impl ArgumentBCS) -> Argument {
    let a0 = arg0.into_argument_mut(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "bcs",
        "peel_option_u8",
        Vec::new(),
        vec![a0],
    )
}
/// Peel `Option<u16>` from serialized bytes.
///
/// Returns: `0x1::option::Option<u16>`.
pub async fn peel_option_u16(b: &mut PtbBuilder, arg0: impl ArgumentBCS) -> Argument {
    let a0 = arg0.into_argument_mut(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "bcs",
        "peel_option_u16",
        Vec::new(),
        vec![a0],
    )
}
/// Peel `Option<u32>` from serialized bytes.
///
/// Returns: `0x1::option::Option<u32>`.
pub async fn peel_option_u32(b: &mut PtbBuilder, arg0: impl ArgumentBCS) -> Argument {
    let a0 = arg0.into_argument_mut(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "bcs",
        "peel_option_u32",
        Vec::new(),
        vec![a0],
    )
}
/// Peel `Option<u64>` from serialized bytes.
///
/// Returns: `0x1::option::Option<u64>`.
pub async fn peel_option_u64(b: &mut PtbBuilder, arg0: impl ArgumentBCS) -> Argument {
    let a0 = arg0.into_argument_mut(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "bcs",
        "peel_option_u64",
        Vec::new(),
        vec![a0],
    )
}
/// Peel `Option<u128>` from serialized bytes.
///
/// Returns: `0x1::option::Option<u128>`.
pub async fn peel_option_u128(b: &mut PtbBuilder, arg0: impl ArgumentBCS) -> Argument {
    let a0 = arg0.into_argument_mut(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "bcs",
        "peel_option_u128",
        Vec::new(),
        vec![a0],
    )
}
/// Peel `Option<u256>` from serialized bytes.
///
/// Returns: `0x1::option::Option<u256>`.
pub async fn peel_option_u256(b: &mut PtbBuilder, arg0: impl ArgumentBCS) -> Argument {
    let a0 = arg0.into_argument_mut(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "bcs",
        "peel_option_u256",
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
    pub fn bcs_tag(&self) -> TypeTag {
        <BCS as MoveType>::type_tag_at(self.package)
    }
}
