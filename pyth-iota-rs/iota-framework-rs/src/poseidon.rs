#![allow(unused_imports, non_snake_case, non_upper_case_globals)]
use move_bindgen_runtime::*;
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;
pub const ENonCanonicalInput: u64 = 0u64;
pub const EEmptyInput: u64 = 1u64;
/// Returns: `u256`.
pub async fn poseidon_bn254(b: &mut PtbBuilder, arg0: impl PureVec<U256>) -> Argument {
    let a0 = arg0.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "poseidon",
        "poseidon_bn254",
        Vec::new(),
        vec![a0],
    )
}
