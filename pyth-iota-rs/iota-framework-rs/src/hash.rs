#![allow(unused_imports, non_snake_case, non_upper_case_globals)]
use move_bindgen_runtime::*;
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;
/// Returns: `vector<u8>`.
pub async fn blake2b256(b: &mut PtbBuilder, arg0: impl PureVec<u8>) -> Argument {
    let a0 = arg0.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "hash",
        "blake2b256",
        Vec::new(),
        vec![a0],
    )
}
/// Returns: `vector<u8>`.
pub async fn keccak256(b: &mut PtbBuilder, arg0: impl PureVec<u8>) -> Argument {
    let a0 = arg0.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "hash",
        "keccak256",
        Vec::new(),
        vec![a0],
    )
}
