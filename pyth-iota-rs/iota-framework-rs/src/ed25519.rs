#![allow(unused_imports, non_snake_case, non_upper_case_globals)]
use move_bindgen_runtime::*;
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;
/// Returns: `bool`.
pub async fn ed25519_verify(
    b: &mut PtbBuilder,
    arg0: impl PureVec<u8>,
    arg1: impl PureVec<u8>,
    arg2: impl PureVec<u8>,
) -> Argument {
    let a0 = arg0.into_argument(b).await;
    let a1 = arg1.into_argument(b).await;
    let a2 = arg2.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "ed25519",
        "ed25519_verify",
        Vec::new(),
        vec![a0, a1, a2],
    )
}
