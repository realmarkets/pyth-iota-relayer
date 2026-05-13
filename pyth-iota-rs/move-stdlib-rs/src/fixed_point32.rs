#![allow(unused_imports, non_snake_case, non_upper_case_globals)]
use move_bindgen_runtime::*;
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;
///> TODO: This is a basic constant and should be provided somewhere centrally in the framework.
pub const MAX_U64: u128 = 18446744073709551615u128;
/// The denominator provided was zero
pub const EDENOMINATOR: u64 = 65537u64;
/// The quotient value would be too large to be held in a `u64`
pub const EDIVISION: u64 = 131074u64;
/// The multiplied value would be too large to be held in a `u64`
pub const EMULTIPLICATION: u64 = 131075u64;
/// A division by zero was encountered
pub const EDIVISION_BY_ZERO: u64 = 65540u64;
/// The computed ratio when converting to a `FixedPoint32` would be unrepresentable
pub const ERATIO_OUT_OF_RANGE: u64 = 131077u64;
/// Define a fixed-point numeric type with 32 fractional bits.
/// This is just a u64 integer but it is wrapped in a struct to
/// make a unique type. This is a binary representation, so decimal
/// values may not be exactly representable, but it provides more
/// than 9 decimal digits of precision both before and after the
/// decimal point (18 digits total). For comparison, double precision
/// floating-point has less than 16 decimal digits of precision, so
/// be careful about using floating-point to convert these values to
/// decimal.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FixedPoint32 {
    pub value: u64,
}
impl MoveType for FixedPoint32 {
    type Package = super::Package;
    const MODULE: &'static str = "fixed_point32";
    const NAME: &'static str = "FixedPoint32";
}
impl MoveArg for FixedPoint32 {
    fn pure_bytes(self) -> PureBytes {
        PureBytes(::bcs::to_bytes(&self).expect("bcs serialization failed"))
    }
}
pub trait ArgumentFixedPoint32: PTBArgument {
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
impl ArgumentFixedPoint32 for FixedPoint32 {}
impl ArgumentFixedPoint32 for Argument {}
/// Multiply a u64 integer by a fixed-point number, truncating any
/// fractional part of the product. This will abort if the product
/// overflows.
///
/// Returns: `u64`.
pub async fn multiply_u64(
    b: &mut PtbBuilder,
    arg0: impl PureU64,
    arg1: impl ArgumentFixedPoint32,
) -> Argument {
    let a0 = arg0.into_argument(b).await;
    let a1 = arg1.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "fixed_point32",
        "multiply_u64",
        Vec::new(),
        vec![a0, a1],
    )
}
/// Divide a u64 integer by a fixed-point number, truncating any
/// fractional part of the quotient. This will abort if the divisor
/// is zero or if the quotient overflows.
///
/// Returns: `u64`.
pub async fn divide_u64(
    b: &mut PtbBuilder,
    arg0: impl PureU64,
    arg1: impl ArgumentFixedPoint32,
) -> Argument {
    let a0 = arg0.into_argument(b).await;
    let a1 = arg1.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "fixed_point32",
        "divide_u64",
        Vec::new(),
        vec![a0, a1],
    )
}
/// Create a fixed-point value from a rational number specified by its
/// numerator and denominator. Calling this function should be preferred
/// for using `Self::create_from_raw_value` which is also available.
/// This will abort if the denominator is zero. It will also
/// abort if the numerator is nonzero and the ratio is not in the range
/// 2^-32 .. 2^32-1. When specifying decimal fractions, be careful about
/// rounding errors: if you round to display N digits after the decimal
/// point, you can use a denominator of 10^N to avoid numbers where the
/// very small imprecision in the binary representation could change the
/// rounding, e.g., 0.0125 will round down to 0.012 instead of up to 0.013.
///
/// Returns: `0x1::fixed_point32::FixedPoint32`.
pub async fn create_from_rational(
    b: &mut PtbBuilder,
    arg0: impl PureU64,
    arg1: impl PureU64,
) -> Argument {
    let a0 = arg0.into_argument(b).await;
    let a1 = arg1.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "fixed_point32",
        "create_from_rational",
        Vec::new(),
        vec![a0, a1],
    )
}
/// Create a fixedpoint value from a raw value.
///
/// Returns: `0x1::fixed_point32::FixedPoint32`.
pub async fn create_from_raw_value(b: &mut PtbBuilder, arg0: impl PureU64) -> Argument {
    let a0 = arg0.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "fixed_point32",
        "create_from_raw_value",
        Vec::new(),
        vec![a0],
    )
}
/// Accessor for the raw u64 value. Other less common operations, such as
/// adding or subtracting FixedPoint32 values, can be done using the raw
/// values directly.
///
/// Returns: `u64`.
pub async fn get_raw_value(
    b: &mut PtbBuilder,
    arg0: impl ArgumentFixedPoint32,
) -> Argument {
    let a0 = arg0.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "fixed_point32",
        "get_raw_value",
        Vec::new(),
        vec![a0],
    )
}
/// Returns true if the ratio is zero.
///
/// Returns: `bool`.
pub async fn is_zero(b: &mut PtbBuilder, arg0: impl ArgumentFixedPoint32) -> Argument {
    let a0 = arg0.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "fixed_point32",
        "is_zero",
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
    pub fn fixedpoint32_tag(&self) -> TypeTag {
        <FixedPoint32 as MoveType>::type_tag_at(self.package)
    }
}
