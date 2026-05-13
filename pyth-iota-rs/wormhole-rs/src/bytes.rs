#![allow(unused_imports, non_snake_case, non_upper_case_globals)]
use move_bindgen_runtime::*;
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;
pub async fn push_u8(b: &mut PtbBuilder, arg0: impl PureVec<u8>, arg1: impl PureU8) {
    let a0 = arg0.into_argument(b).await;
    let a1 = arg1.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "bytes",
        "push_u8",
        Vec::new(),
        vec![a0, a1],
    );
}
pub async fn push_u16_be(
    b: &mut PtbBuilder,
    arg0: impl PureVec<u8>,
    arg1: impl PureU16,
) {
    let a0 = arg0.into_argument(b).await;
    let a1 = arg1.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "bytes",
        "push_u16_be",
        Vec::new(),
        vec![a0, a1],
    );
}
pub async fn push_u32_be(
    b: &mut PtbBuilder,
    arg0: impl PureVec<u8>,
    arg1: impl PureU32,
) {
    let a0 = arg0.into_argument(b).await;
    let a1 = arg1.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "bytes",
        "push_u32_be",
        Vec::new(),
        vec![a0, a1],
    );
}
pub async fn push_u64_be(
    b: &mut PtbBuilder,
    arg0: impl PureVec<u8>,
    arg1: impl PureU64,
) {
    let a0 = arg0.into_argument(b).await;
    let a1 = arg1.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "bytes",
        "push_u64_be",
        Vec::new(),
        vec![a0, a1],
    );
}
pub async fn push_u128_be(
    b: &mut PtbBuilder,
    arg0: impl PureVec<u8>,
    arg1: impl PureU128,
) {
    let a0 = arg0.into_argument(b).await;
    let a1 = arg1.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "bytes",
        "push_u128_be",
        Vec::new(),
        vec![a0, a1],
    );
}
pub async fn push_u256_be(
    b: &mut PtbBuilder,
    arg0: impl PureVec<u8>,
    arg1: impl PureU256,
) {
    let a0 = arg0.into_argument(b).await;
    let a1 = arg1.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "bytes",
        "push_u256_be",
        Vec::new(),
        vec![a0, a1],
    );
}
/// Returns: `u8`.
pub async fn take_u8(
    b: &mut PtbBuilder,
    arg0: impl super::cursor::ArgumentCursor<u8>,
) -> Argument {
    let a0 = arg0.into_argument_mut(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "bytes",
        "take_u8",
        Vec::new(),
        vec![a0],
    )
}
/// Returns: `u16`.
pub async fn take_u16_be(
    b: &mut PtbBuilder,
    arg0: impl super::cursor::ArgumentCursor<u8>,
) -> Argument {
    let a0 = arg0.into_argument_mut(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "bytes",
        "take_u16_be",
        Vec::new(),
        vec![a0],
    )
}
/// Returns: `u32`.
pub async fn take_u32_be(
    b: &mut PtbBuilder,
    arg0: impl super::cursor::ArgumentCursor<u8>,
) -> Argument {
    let a0 = arg0.into_argument_mut(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "bytes",
        "take_u32_be",
        Vec::new(),
        vec![a0],
    )
}
/// Returns: `u64`.
pub async fn take_u64_be(
    b: &mut PtbBuilder,
    arg0: impl super::cursor::ArgumentCursor<u8>,
) -> Argument {
    let a0 = arg0.into_argument_mut(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "bytes",
        "take_u64_be",
        Vec::new(),
        vec![a0],
    )
}
/// Returns: `u128`.
pub async fn take_u128_be(
    b: &mut PtbBuilder,
    arg0: impl super::cursor::ArgumentCursor<u8>,
) -> Argument {
    let a0 = arg0.into_argument_mut(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "bytes",
        "take_u128_be",
        Vec::new(),
        vec![a0],
    )
}
/// Returns: `u256`.
pub async fn take_u256_be(
    b: &mut PtbBuilder,
    arg0: impl super::cursor::ArgumentCursor<u8>,
) -> Argument {
    let a0 = arg0.into_argument_mut(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "bytes",
        "take_u256_be",
        Vec::new(),
        vec![a0],
    )
}
/// Returns: `vector<u8>`.
pub async fn take_bytes(
    b: &mut PtbBuilder,
    arg0: impl super::cursor::ArgumentCursor<u8>,
    arg1: impl PureU64,
) -> Argument {
    let a0 = arg0.into_argument_mut(b).await;
    let a1 = arg1.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "bytes",
        "take_bytes",
        Vec::new(),
        vec![a0, a1],
    )
}
