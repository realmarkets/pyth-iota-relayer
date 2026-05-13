#![allow(unused_imports, non_snake_case, non_upper_case_globals)]
use move_bindgen_runtime::*;
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;
pub const MERKLE_LEAF_PREFIX: u8 = 0u8;
pub const MERKLE_NODE_PREFIX: u8 = 1u8;
pub const MERKLE_EMPTY_LEAF_PREFIX: u8 = 2u8;
pub const E_DEPTH_NOT_LARGE_ENOUGH_FOR_MESSAGES: u64 = 1212121u64;
/// Returns: `bool`.
pub async fn is_proof_valid(
    b: &mut PtbBuilder,
    arg0: impl ::wormhole_rs::cursor::ArgumentCursor<u8>,
    arg1: impl ::wormhole_rs::bytes20::ArgumentBytes20,
    arg2: impl PureVec<u8>,
) -> Argument {
    let a0 = arg0.into_argument_mut(b).await;
    let a1 = arg1.into_argument(b).await;
    let a2 = arg2.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "merkle_tree",
        "is_proof_valid",
        Vec::new(),
        vec![a0, a1, a2],
    )
}
/// Returns: `(0xff00000000000002::bytes20::Bytes20, vector<u8>)`.
pub async fn construct_proofs(
    b: &mut PtbBuilder,
    arg0: impl PureVec<Vec<u8>>,
    arg1: impl PureU8,
) -> (Argument, Argument) {
    let a0 = arg0.into_argument(b).await;
    let a1 = arg1.into_argument(b).await;
    let __r = b
        .move_call_n(
            b.package_id::<super::Package>(),
            "merkle_tree",
            "construct_proofs",
            Vec::new(),
            vec![a0, a1],
            2u16,
        );
    (__r[0], __r[1])
}
