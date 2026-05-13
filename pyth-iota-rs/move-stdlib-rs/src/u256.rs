#![allow(unused_imports, non_snake_case, non_upper_case_globals)]
use move_bindgen_runtime::*;
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;
/// Return the larger of `x` and `y`
///
/// Returns: `u256`.
pub async fn max(
    b: &mut PtbBuilder,
    arg0: impl PureU256,
    arg1: impl PureU256,
) -> Argument {
    let a0 = arg0.into_argument(b).await;
    let a1 = arg1.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "u256",
        "max",
        Vec::new(),
        vec![a0, a1],
    )
}
/// Return the smaller of `x` and `y`
///
/// Returns: `u256`.
pub async fn min(
    b: &mut PtbBuilder,
    arg0: impl PureU256,
    arg1: impl PureU256,
) -> Argument {
    let a0 = arg0.into_argument(b).await;
    let a1 = arg1.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "u256",
        "min",
        Vec::new(),
        vec![a0, a1],
    )
}
/// Return the absolute value of x - y
///
/// Returns: `u256`.
pub async fn diff(
    b: &mut PtbBuilder,
    arg0: impl PureU256,
    arg1: impl PureU256,
) -> Argument {
    let a0 = arg0.into_argument(b).await;
    let a1 = arg1.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "u256",
        "diff",
        Vec::new(),
        vec![a0, a1],
    )
}
/// Calculate x / y, but round up the result.
///
/// Returns: `u256`.
pub async fn divide_and_round_up(
    b: &mut PtbBuilder,
    arg0: impl PureU256,
    arg1: impl PureU256,
) -> Argument {
    let a0 = arg0.into_argument(b).await;
    let a1 = arg1.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "u256",
        "divide_and_round_up",
        Vec::new(),
        vec![a0, a1],
    )
}
/// Return the value of a base raised to a power
///
/// Returns: `u256`.
pub async fn pow(
    b: &mut PtbBuilder,
    arg0: impl PureU256,
    arg1: impl PureU8,
) -> Argument {
    let a0 = arg0.into_argument(b).await;
    let a1 = arg1.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "u256",
        "pow",
        Vec::new(),
        vec![a0, a1],
    )
}
