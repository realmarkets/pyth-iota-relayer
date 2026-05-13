#![allow(unused_imports, non_snake_case, non_upper_case_globals)]
//! Provides a way to get address length since it's a
//! platform-specific parameter.
use move_bindgen_runtime::*;
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;
/// Should be converted to a native function.
/// Current implementation only works for IOTA.
///
/// Returns: `u64`.
pub async fn length(b: &mut PtbBuilder) -> Argument {
    b.move_call(
        b.package_id::<super::Package>(),
        "address",
        "length",
        Vec::new(),
        vec![],
    )
}
