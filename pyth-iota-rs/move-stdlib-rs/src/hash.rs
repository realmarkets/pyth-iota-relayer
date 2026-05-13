#![allow(unused_imports, non_snake_case, non_upper_case_globals)]
//! Module which defines SHA hashes for byte vectors.
//!
//! The functions in this module are natively declared both in the Move runtime
//! as in the Move prover's prelude.
use move_bindgen_runtime::*;
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;
/// Returns: `vector<u8>`.
pub async fn sha2_256(b: &mut PtbBuilder, arg0: impl PureVec<u8>) -> Argument {
    let a0 = arg0.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "hash",
        "sha2_256",
        Vec::new(),
        vec![a0],
    )
}
/// Returns: `vector<u8>`.
pub async fn sha3_256(b: &mut PtbBuilder, arg0: impl PureVec<u8>) -> Argument {
    let a0 = arg0.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "hash",
        "sha3_256",
        Vec::new(),
        vec![a0],
    )
}
