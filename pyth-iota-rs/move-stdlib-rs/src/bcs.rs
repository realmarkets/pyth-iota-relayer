#![allow(unused_imports, non_snake_case, non_upper_case_globals)]
//! Utility for converting a Move value to its binary representation in BCS (Binary Canonical
//! Serialization). BCS is the binary encoding for Move resources and other non-module values
//! published on-chain. See https://github.com/diem/bcs#binary-canonical-serialization-bcs for more
//! details on BCS.
use move_bindgen_runtime::*;
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;
/// Returns: `vector<u8>`.
pub async fn to_bytes<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentObject<()>,
) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "bcs",
        "to_bytes",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![a0],
    )
}
