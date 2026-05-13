#![allow(unused_imports, non_snake_case, non_upper_case_globals)]
//! Module providing debug functionality.
use move_bindgen_runtime::*;
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;
pub async fn print<T0: MoveType>(b: &mut PtbBuilder, arg0: impl ArgumentObject<()>) {
    let a0 = arg0.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "debug",
        "print",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![a0],
    );
}
pub async fn print_stack_trace(b: &mut PtbBuilder) {
    b.move_call(
        b.package_id::<super::Package>(),
        "debug",
        "print_stack_trace",
        Vec::new(),
        vec![],
    );
}
