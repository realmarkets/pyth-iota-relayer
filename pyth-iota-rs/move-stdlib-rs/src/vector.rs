#![allow(unused_imports, non_snake_case, non_upper_case_globals)]
//! A variable-sized container that can hold any type. Indexing is 0-based, and
//! vectors are growable. This module has many native functions.
use move_bindgen_runtime::*;
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;
/// The index into the vector is out of bounds
pub const EINDEX_OUT_OF_BOUNDS: u64 = 131072u64;
/// Returns: `vector<T0>`.
pub async fn empty<T0: MoveType>(b: &mut PtbBuilder) -> Argument {
    b.move_call(
        b.package_id::<super::Package>(),
        "vector",
        "empty",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![],
    )
}
/// Returns: `u64`.
pub async fn length<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl PureVec<T0>,
) -> Argument {
    let a0 = arg0.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "vector",
        "length",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![a0],
    )
}
/// Returns: `&T0`.
pub async fn borrow<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl PureVec<T0>,
    arg1: impl PureU64,
) -> Argument {
    let a0 = arg0.into_argument(b).await;
    let a1 = arg1.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "vector",
        "borrow",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![a0, a1],
    )
}
pub async fn push_back<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl PureVec<T0>,
    arg1: impl ArgumentObject<()>,
) {
    let a0 = arg0.into_argument(b).await;
    let a1 = arg1.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "vector",
        "push_back",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![a0, a1],
    );
}
/// Returns: `&mut T0`.
pub async fn borrow_mut<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl PureVec<T0>,
    arg1: impl PureU64,
) -> Argument {
    let a0 = arg0.into_argument(b).await;
    let a1 = arg1.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "vector",
        "borrow_mut",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![a0, a1],
    )
}
/// Returns: `T0`.
pub async fn pop_back<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl PureVec<T0>,
) -> Argument {
    let a0 = arg0.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "vector",
        "pop_back",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![a0],
    )
}
pub async fn destroy_empty<T0: MoveType>(b: &mut PtbBuilder, arg0: impl PureVec<T0>) {
    let a0 = arg0.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "vector",
        "destroy_empty",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![a0],
    );
}
pub async fn swap<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl PureVec<T0>,
    arg1: impl PureU64,
    arg2: impl PureU64,
) {
    let a0 = arg0.into_argument(b).await;
    let a1 = arg1.into_argument(b).await;
    let a2 = arg2.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "vector",
        "swap",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![a0, a1, a2],
    );
}
/// Return an vector of size one containing element `e`.
///
/// Returns: `vector<T0>`.
pub async fn singleton<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentObject<()>,
) -> Argument {
    let a0 = arg0.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "vector",
        "singleton",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![a0],
    )
}
/// Reverses the order of the elements in the vector `v` in place.
pub async fn reverse<T0: MoveType>(b: &mut PtbBuilder, arg0: impl PureVec<T0>) {
    let a0 = arg0.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "vector",
        "reverse",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![a0],
    );
}
/// Pushes all of the elements of the `other` vector into the `lhs` vector.
pub async fn append<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl PureVec<T0>,
    arg1: impl PureVec<T0>,
) {
    let a0 = arg0.into_argument(b).await;
    let a1 = arg1.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "vector",
        "append",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![a0, a1],
    );
}
/// Return `true` if the vector `v` has no elements and `false` otherwise.
///
/// Returns: `bool`.
pub async fn is_empty<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl PureVec<T0>,
) -> Argument {
    let a0 = arg0.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "vector",
        "is_empty",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![a0],
    )
}
/// Return true if `e` is in the vector `v`.
/// Otherwise, returns false.
///
/// Returns: `bool`.
pub async fn contains<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl PureVec<T0>,
    arg1: impl ArgumentObject<()>,
) -> Argument {
    let a0 = arg0.into_argument(b).await;
    let a1 = arg1.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "vector",
        "contains",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![a0, a1],
    )
}
/// Return `(true, i)` if `e` is in the vector `v` at index `i`.
/// Otherwise, returns `(false, 0)`.
///
/// Returns: `(bool, u64)`.
pub async fn index_of<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl PureVec<T0>,
    arg1: impl ArgumentObject<()>,
) -> (Argument, Argument) {
    let a0 = arg0.into_argument(b).await;
    let a1 = arg1.into_argument_ref(b).await;
    let __r = b
        .move_call_n(
            b.package_id::<super::Package>(),
            "vector",
            "index_of",
            vec![< T0 as MoveType > ::type_tag(b)],
            vec![a0, a1],
            2u16,
        );
    (__r[0], __r[1])
}
/// Remove the `i`th element of the vector `v`, shifting all subsequent elements.
/// This is O(n) and preserves ordering of elements in the vector.
/// Aborts if `i` is out of bounds.
///
/// Returns: `T0`.
pub async fn remove<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl PureVec<T0>,
    arg1: impl PureU64,
) -> Argument {
    let a0 = arg0.into_argument(b).await;
    let a1 = arg1.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "vector",
        "remove",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![a0, a1],
    )
}
/// Insert `e` at position `i` in the vector `v`.
/// If `i` is in bounds, this shifts the old `v[i]` and all subsequent elements to the right.
/// If `i == v.length()`, this adds `e` to the end of the vector.
/// This is O(n) and preserves ordering of elements in the vector.
/// Aborts if `i > v.length()`
pub async fn insert<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl PureVec<T0>,
    arg1: impl ArgumentObject<()>,
    arg2: impl PureU64,
) {
    let a0 = arg0.into_argument(b).await;
    let a1 = arg1.into_argument(b).await;
    let a2 = arg2.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "vector",
        "insert",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![a0, a1, a2],
    );
}
/// Swap the `i`th element of the vector `v` with the last element and then pop the vector.
/// This is O(1), but does not preserve ordering of elements in the vector.
/// Aborts if `i` is out of bounds.
///
/// Returns: `T0`.
pub async fn swap_remove<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl PureVec<T0>,
    arg1: impl PureU64,
) -> Argument {
    let a0 = arg0.into_argument(b).await;
    let a1 = arg1.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "vector",
        "swap_remove",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![a0, a1],
    )
}
