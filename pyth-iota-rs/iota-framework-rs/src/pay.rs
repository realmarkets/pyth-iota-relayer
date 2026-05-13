#![allow(unused_imports, non_snake_case, non_upper_case_globals)]
//! This module provides handy functionality for wallets and `iota::Coin` management.
use move_bindgen_runtime::*;
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;
/// For when empty vector is supplied into join function.
pub const ENoCoins: u64 = 0u64;
/// Transfer `c` to the sender of the current transaction
pub async fn keep<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl super::coin::ArgumentCoin<T0>,
) {
    let a0 = arg0.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "pay",
        "keep",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![a0],
    );
}
/// Split coin `self` to two coins, one with balance `split_amount`,
/// and the remaining balance is left is `self`.
pub async fn split<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl super::coin::ArgumentCoin<T0>,
    arg1: impl PureU64,
) {
    let a0 = arg0.into_argument_mut(b).await;
    let a1 = arg1.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "pay",
        "split",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![a0, a1],
    );
}
/// Split coin `self` into multiple coins, each with balance specified
/// in `split_amounts`. Remaining balance is left in `self`.
pub async fn split_vec<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl super::coin::ArgumentCoin<T0>,
    arg1: impl PureVec<u64>,
) {
    let a0 = arg0.into_argument_mut(b).await;
    let a1 = arg1.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "pay",
        "split_vec",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![a0, a1],
    );
}
/// Send `amount` units of `c` to `recipient`
/// Aborts with `EVALUE` if `amount` is greater than or equal to `amount`
pub async fn split_and_transfer<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl super::coin::ArgumentCoin<T0>,
    arg1: impl PureU64,
    arg2: impl PureAddress,
) {
    let a0 = arg0.into_argument_mut(b).await;
    let a1 = arg1.into_argument(b).await;
    let a2 = arg2.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "pay",
        "split_and_transfer",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![a0, a1, a2],
    );
}
/// Divide coin `self` into `n - 1` coins with equal balances. If the balance is
/// not evenly divisible by `n`, the remainder is left in `self`.
pub async fn divide_and_keep<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl super::coin::ArgumentCoin<T0>,
    arg1: impl PureU64,
) {
    let a0 = arg0.into_argument_mut(b).await;
    let a1 = arg1.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "pay",
        "divide_and_keep",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![a0, a1],
    );
}
/// Join everything in `coins` with `self`
pub async fn join_vec<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl super::coin::ArgumentCoin<T0>,
    arg1: impl PureVec<super::coin::Coin<T0>>,
) {
    let a0 = arg0.into_argument_mut(b).await;
    let a1 = arg1.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "pay",
        "join_vec",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![a0, a1],
    );
}
/// Join a vector of `Coin` into a single object and transfer it to `receiver`.
pub async fn join_vec_and_transfer<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl PureVec<super::coin::Coin<T0>>,
    arg1: impl PureAddress,
) {
    let a0 = arg0.into_argument(b).await;
    let a1 = arg1.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "pay",
        "join_vec_and_transfer",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![a0, a1],
    );
}
