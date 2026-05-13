#![allow(unused_imports, non_snake_case, non_upper_case_globals)]
use move_bindgen_runtime::*;
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;
/// Return the larger of `x` and `y`
///
/// Returns: `u8`.
pub async fn max(b: &mut PtbBuilder, arg0: impl PureU8, arg1: impl PureU8) -> Argument {
    let a0 = arg0.into_argument(b).await;
    let a1 = arg1.into_argument(b).await;
    b.move_call(b.package_id::<super::Package>(), "u8", "max", Vec::new(), vec![a0, a1])
}
/// Return the smaller of `x` and `y`
///
/// Returns: `u8`.
pub async fn min(b: &mut PtbBuilder, arg0: impl PureU8, arg1: impl PureU8) -> Argument {
    let a0 = arg0.into_argument(b).await;
    let a1 = arg1.into_argument(b).await;
    b.move_call(b.package_id::<super::Package>(), "u8", "min", Vec::new(), vec![a0, a1])
}
/// Return the absolute value of x - y
///
/// Returns: `u8`.
pub async fn diff(b: &mut PtbBuilder, arg0: impl PureU8, arg1: impl PureU8) -> Argument {
    let a0 = arg0.into_argument(b).await;
    let a1 = arg1.into_argument(b).await;
    b.move_call(b.package_id::<super::Package>(), "u8", "diff", Vec::new(), vec![a0, a1])
}
/// Calculate x / y, but round up the result.
///
/// Returns: `u8`.
pub async fn divide_and_round_up(
    b: &mut PtbBuilder,
    arg0: impl PureU8,
    arg1: impl PureU8,
) -> Argument {
    let a0 = arg0.into_argument(b).await;
    let a1 = arg1.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "u8",
        "divide_and_round_up",
        Vec::new(),
        vec![a0, a1],
    )
}
/// Return the value of a base raised to a power
///
/// Returns: `u8`.
pub async fn pow(b: &mut PtbBuilder, arg0: impl PureU8, arg1: impl PureU8) -> Argument {
    let a0 = arg0.into_argument(b).await;
    let a1 = arg1.into_argument(b).await;
    b.move_call(b.package_id::<super::Package>(), "u8", "pow", Vec::new(), vec![a0, a1])
}
/// Get a nearest lower integer Square Root for `x`. Given that this
/// function can only operate with integers, it is impossible
/// to get perfect (or precise) integer square root for some numbers.
///
/// Example:
/// ```
/// math::sqrt(9) => 3
/// math::sqrt(8) => 2 // the nearest lower square root is 4;
/// ```
///
/// In integer math, one of the possible ways to get results with more
/// precision is to use higher values or temporarily multiply the
/// value by some bigger number. Ideally if this is a square of 10 or 100.
///
/// Example:
/// ```
/// math::sqrt(8) => 2;
/// math::sqrt(8 * 10000) => 282;
/// // now we can use this value as if it was 2.82;
/// // but to get the actual result, this value needs
/// // to be divided by 100 (because sqrt(10000)).
///
///
/// math::sqrt(8 * 1000000) => 2828; // same as above, 2828 / 1000 (2.828)
/// ```
///
/// Returns: `u8`.
pub async fn sqrt(b: &mut PtbBuilder, arg0: impl PureU8) -> Argument {
    let a0 = arg0.into_argument(b).await;
    b.move_call(b.package_id::<super::Package>(), "u8", "sqrt", Vec::new(), vec![a0])
}
