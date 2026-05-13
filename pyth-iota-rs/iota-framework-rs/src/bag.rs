#![allow(unused_imports, non_snake_case, non_upper_case_globals)]
//! A bag is a heterogeneous map-like collection. The collection is similar to `iota::table` in that
//! its keys and values are not stored within the `Bag` value, but instead are stored using IOTA's
//! object system. The `Bag` struct acts only as a handle into the object system to retrieve those
//! keys and values.
//! Note that this means that `Bag` values with exactly the same key-value mapping will not be
//! equal, with `==`, at runtime. For example
//! ```
//! let bag1 = bag::new();
//! let bag2 = bag::new();
//! bag::add(&mut bag1, 0, false);
//! bag::add(&mut bag1, 1, true);
//! bag::add(&mut bag2, 0, false);
//! bag::add(&mut bag2, 1, true);
//! // bag1 does not equal bag2, despite having the same entries
//! assert!(&bag1 != &bag2);
//! ```
//! At it's core, `iota::bag` is a wrapper around `UID` that allows for access to
//! `iota::dynamic_field` while preventing accidentally stranding field values. A `UID` can be
//! deleted, even if it has dynamic fields associated with it, but a bag, on the other hand, must be
//! empty to be destroyed.
use move_bindgen_runtime::*;
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;
pub const EBagNotEmpty: u64 = 0u64;
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Bag {
    /// the ID of this bag
    pub id: UID,
    /// the number of key-value pairs in the bag
    pub size: u64,
}
impl MoveType for Bag {
    type Package = super::Package;
    const MODULE: &'static str = "bag";
    const NAME: &'static str = "Bag";
}
pub trait ArgumentBag: PTBArgument {
    #[allow(async_fn_in_trait)]
    async fn into_argument(self, b: &mut PtbBuilder) -> Argument
    where
        Self: Sized,
    {
        b.inner.apply_argument(self)
    }
    #[allow(async_fn_in_trait)]
    async fn into_argument_ref(self, b: &mut PtbBuilder) -> Argument
    where
        Self: Sized,
    {
        self.into_argument(b).await
    }
    #[allow(async_fn_in_trait)]
    async fn into_argument_mut(self, b: &mut PtbBuilder) -> Argument
    where
        Self: Sized,
    {
        self.into_argument(b).await
    }
}
impl ArgumentBag for Argument {}
impl ArgumentBag for ObjectId {
    async fn into_argument(self, b: &mut PtbBuilder) -> Argument {
        b.resolve_object(self).await
    }
    async fn into_argument_ref(self, b: &mut PtbBuilder) -> Argument {
        b.resolve_object_shared(self, false).await
    }
    async fn into_argument_mut(self, b: &mut PtbBuilder) -> Argument {
        b.resolve_object_shared(self, true).await
    }
}
impl ArgumentBag for ObjectReference {}
impl ArgumentBag for Shared<ObjectId> {}
impl ArgumentBag for SharedMut<ObjectId> {}
impl ArgumentBag for Receiving<ObjectId> {}
/// Creates a new, empty bag
///
/// Returns: `0x2::bag::Bag`.
pub async fn new(b: &mut PtbBuilder) -> Argument {
    b.move_call(b.package_id::<super::Package>(), "bag", "new", Vec::new(), vec![])
}
/// Adds a key-value pair to the bag `bag: &mut Bag`
/// Aborts with `iota::dynamic_field::EFieldAlreadyExists` if the bag already has an entry with
/// that key `k: K`.
pub async fn add<T0: MoveType, T1: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentBag,
    arg1: impl ArgumentObject<()>,
    arg2: impl ArgumentObject<()>,
) {
    let a0 = arg0.into_argument_mut(b).await;
    let a1 = arg1.into_argument(b).await;
    let a2 = arg2.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "bag",
        "add",
        vec![< T0 as MoveType > ::type_tag(b), < T1 as MoveType > ::type_tag(b)],
        vec![a0, a1, a2],
    );
}
/// Immutable borrows the value associated with the key in the bag `bag: &Bag`.
/// Aborts with `iota::dynamic_field::EFieldDoesNotExist` if the bag does not have an entry with
/// that key `k: K`.
/// Aborts with `iota::dynamic_field::EFieldTypeMismatch` if the bag has an entry for the key, but
/// the value does not have the specified type.
///
/// Returns: `&T1`.
pub async fn borrow<T0: MoveType, T1: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentBag,
    arg1: impl ArgumentObject<()>,
) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    let a1 = arg1.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "bag",
        "borrow",
        vec![< T0 as MoveType > ::type_tag(b), < T1 as MoveType > ::type_tag(b)],
        vec![a0, a1],
    )
}
/// Mutably borrows the value associated with the key in the bag `bag: &mut Bag`.
/// Aborts with `iota::dynamic_field::EFieldDoesNotExist` if the bag does not have an entry with
/// that key `k: K`.
/// Aborts with `iota::dynamic_field::EFieldTypeMismatch` if the bag has an entry for the key, but
/// the value does not have the specified type.
///
/// Returns: `&mut T1`.
pub async fn borrow_mut<T0: MoveType, T1: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentBag,
    arg1: impl ArgumentObject<()>,
) -> Argument {
    let a0 = arg0.into_argument_mut(b).await;
    let a1 = arg1.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "bag",
        "borrow_mut",
        vec![< T0 as MoveType > ::type_tag(b), < T1 as MoveType > ::type_tag(b)],
        vec![a0, a1],
    )
}
/// Mutably borrows the key-value pair in the bag `bag: &mut Bag` and returns the value.
/// Aborts with `iota::dynamic_field::EFieldDoesNotExist` if the bag does not have an entry with
/// that key `k: K`.
/// Aborts with `iota::dynamic_field::EFieldTypeMismatch` if the bag has an entry for the key, but
/// the value does not have the specified type.
///
/// Returns: `T1`.
pub async fn remove<T0: MoveType, T1: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentBag,
    arg1: impl ArgumentObject<()>,
) -> Argument {
    let a0 = arg0.into_argument_mut(b).await;
    let a1 = arg1.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "bag",
        "remove",
        vec![< T0 as MoveType > ::type_tag(b), < T1 as MoveType > ::type_tag(b)],
        vec![a0, a1],
    )
}
/// Returns true iff there is an value associated with the key `k: K` in the bag `bag: &Bag`
///
/// Returns: `bool`.
pub async fn contains<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentBag,
    arg1: impl ArgumentObject<()>,
) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    let a1 = arg1.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "bag",
        "contains",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![a0, a1],
    )
}
/// Returns true iff there is an value associated with the key `k: K` in the bag `bag: &Bag`
/// with an assigned value of type `V`
///
/// Returns: `bool`.
pub async fn contains_with_type<T0: MoveType, T1: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentBag,
    arg1: impl ArgumentObject<()>,
) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    let a1 = arg1.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "bag",
        "contains_with_type",
        vec![< T0 as MoveType > ::type_tag(b), < T1 as MoveType > ::type_tag(b)],
        vec![a0, a1],
    )
}
/// Returns the size of the bag, the number of key-value pairs
///
/// Returns: `u64`.
pub async fn length(b: &mut PtbBuilder, arg0: impl ArgumentBag) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    b.move_call(b.package_id::<super::Package>(), "bag", "length", Vec::new(), vec![a0])
}
/// Returns true iff the bag is empty (if `length` returns `0`)
///
/// Returns: `bool`.
pub async fn is_empty(b: &mut PtbBuilder, arg0: impl ArgumentBag) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "bag",
        "is_empty",
        Vec::new(),
        vec![a0],
    )
}
/// Destroys an empty bag
/// Aborts with `EBagNotEmpty` if the bag still contains values
pub async fn destroy_empty(b: &mut PtbBuilder, arg0: impl ArgumentBag) {
    let a0 = arg0.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "bag",
        "destroy_empty",
        Vec::new(),
        vec![a0],
    );
}
/// Read-only handle bound to a runtime package address. Use
/// via `super::Package::at(addr).<module>()` and chain
/// `<type>_tag()` to build a [`TypeTag`] without spinning up
/// a `PtbBuilder`.
pub struct ModuleAt {
    pub(crate) package: Address,
}
impl ModuleAt {
    pub fn bag_tag(&self) -> TypeTag {
        <Bag as MoveType>::type_tag_at(self.package)
    }
}
