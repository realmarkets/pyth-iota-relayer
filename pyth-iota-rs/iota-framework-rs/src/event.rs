#![allow(unused_imports, non_snake_case, non_upper_case_globals)]
//! Events module. Defines the `iota::event::emit` function which
//! creates and sends a custom MoveEvent as a part of the effects
//! certificate of the transaction.
//!
//! Every MoveEvent has the following properties:
//!  - sender
//!  - type signature (`T`)
//!  - event data (the value of `T`)
//!  - timestamp (local to a node)
//!  - transaction digest
//!
//! Example:
//! ```
//! module my::marketplace {
//!    use iota::event;
//!    /* ... */
//!    struct ItemPurchased has copy, drop {
//!      item_id: ID, buyer: address
//!    }
//!    entry fun buy(/* .... */) {
//!       /* ... */
//!       event::emit(ItemPurchased { item_id: ..., buyer: .... })
//!    }
//! }
//! ```
use move_bindgen_runtime::*;
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;
pub async fn emit<T0: MoveType>(b: &mut PtbBuilder, arg0: impl ArgumentObject<()>) {
    let a0 = arg0.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "event",
        "emit",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![a0],
    );
}
