#![allow(unused_imports, non_snake_case, non_upper_case_globals)]
use move_bindgen_runtime::*;
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;
pub const EFailToRecoverPubKey: u64 = 0u64;
pub const EInvalidSignature: u64 = 1u64;
pub const KECCAK256: u8 = 0u8;
pub const SHA256: u8 = 1u8;
/// Returns: `vector<u8>`.
pub async fn secp256r1_ecrecover(
    b: &mut PtbBuilder,
    arg0: impl PureVec<u8>,
    arg1: impl PureVec<u8>,
    arg2: impl PureU8,
) -> Argument {
    let a0 = arg0.into_argument(b).await;
    let a1 = arg1.into_argument(b).await;
    let a2 = arg2.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "ecdsa_r1",
        "secp256r1_ecrecover",
        Vec::new(),
        vec![a0, a1, a2],
    )
}
/// Returns: `bool`.
pub async fn secp256r1_verify(
    b: &mut PtbBuilder,
    arg0: impl PureVec<u8>,
    arg1: impl PureVec<u8>,
    arg2: impl PureVec<u8>,
    arg3: impl PureU8,
) -> Argument {
    let a0 = arg0.into_argument(b).await;
    let a1 = arg1.into_argument(b).await;
    let a2 = arg2.into_argument(b).await;
    let a3 = arg3.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "ecdsa_r1",
        "secp256r1_verify",
        Vec::new(),
        vec![a0, a1, a2, a3],
    )
}
