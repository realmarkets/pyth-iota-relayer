#![allow(unused_imports, non_snake_case, non_upper_case_globals)]
//! IOTA types helpers and utilities
use move_bindgen_runtime::*;
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;
/// Returns: `bool`.
pub async fn is_one_time_witness<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentObject<()>,
) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "types",
        "is_one_time_witness",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![a0],
    )
}
