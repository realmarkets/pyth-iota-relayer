#![allow(unused_imports, non_snake_case, non_upper_case_globals)]
use move_bindgen_runtime::*;
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;
pub const EInvalidHashLength: u64 = 1u64;
pub const EInvalidPublicKeyEncoding: u64 = 2u64;
pub const EInvalidProofEncoding: u64 = 3u64;
/// Returns: `bool`.
pub async fn ecvrf_verify(
    b: &mut PtbBuilder,
    arg0: impl PureVec<u8>,
    arg1: impl PureVec<u8>,
    arg2: impl PureVec<u8>,
    arg3: impl PureVec<u8>,
) -> Argument {
    let a0 = arg0.into_argument(b).await;
    let a1 = arg1.into_argument(b).await;
    let a2 = arg2.into_argument(b).await;
    let a3 = arg3.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "ecvrf",
        "ecvrf_verify",
        Vec::new(),
        vec![a0, a1, a2, a3],
    )
}
