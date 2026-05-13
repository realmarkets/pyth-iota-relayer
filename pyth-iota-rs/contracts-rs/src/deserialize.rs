#![allow(unused_imports, non_snake_case, non_upper_case_globals)]
use move_bindgen_runtime::*;
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;
/// Returns: `vector<u8>`.
pub async fn deserialize_vector(
    b: &mut PtbBuilder,
    arg0: impl ::wormhole_rs::cursor::ArgumentCursor<u8>,
    arg1: impl PureU64,
) -> Argument {
    let a0 = arg0.into_argument_mut(b).await;
    let a1 = arg1.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "deserialize",
        "deserialize_vector",
        Vec::new(),
        vec![a0, a1],
    )
}
/// Returns: `u8`.
pub async fn deserialize_u8(
    b: &mut PtbBuilder,
    arg0: impl ::wormhole_rs::cursor::ArgumentCursor<u8>,
) -> Argument {
    let a0 = arg0.into_argument_mut(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "deserialize",
        "deserialize_u8",
        Vec::new(),
        vec![a0],
    )
}
/// Returns: `u16`.
pub async fn deserialize_u16(
    b: &mut PtbBuilder,
    arg0: impl ::wormhole_rs::cursor::ArgumentCursor<u8>,
) -> Argument {
    let a0 = arg0.into_argument_mut(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "deserialize",
        "deserialize_u16",
        Vec::new(),
        vec![a0],
    )
}
/// Returns: `u32`.
pub async fn deserialize_u32(
    b: &mut PtbBuilder,
    arg0: impl ::wormhole_rs::cursor::ArgumentCursor<u8>,
) -> Argument {
    let a0 = arg0.into_argument_mut(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "deserialize",
        "deserialize_u32",
        Vec::new(),
        vec![a0],
    )
}
/// Returns: `0xff00000000000001::i64::I64`.
pub async fn deserialize_i32(
    b: &mut PtbBuilder,
    arg0: impl ::wormhole_rs::cursor::ArgumentCursor<u8>,
) -> Argument {
    let a0 = arg0.into_argument_mut(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "deserialize",
        "deserialize_i32",
        Vec::new(),
        vec![a0],
    )
}
/// Returns: `u64`.
pub async fn deserialize_u64(
    b: &mut PtbBuilder,
    arg0: impl ::wormhole_rs::cursor::ArgumentCursor<u8>,
) -> Argument {
    let a0 = arg0.into_argument_mut(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "deserialize",
        "deserialize_u64",
        Vec::new(),
        vec![a0],
    )
}
/// Returns: `0xff00000000000001::i64::I64`.
pub async fn deserialize_i64(
    b: &mut PtbBuilder,
    arg0: impl ::wormhole_rs::cursor::ArgumentCursor<u8>,
) -> Argument {
    let a0 = arg0.into_argument_mut(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "deserialize",
        "deserialize_i64",
        Vec::new(),
        vec![a0],
    )
}
