#![allow(unused_imports, non_snake_case, non_upper_case_globals)]
//! Similar to `iota::bag`, an `ObjectBag` is a heterogeneous map-like collection. But unlike
//! `iota::bag`, the values bound to these dynamic fields _must_ be objects themselves. This allows
//! for the objects to still exist in storage, which may be important for external tools.
//! The difference is otherwise not observable from within Move.
use move_bindgen_runtime::*;
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;
pub const EBagNotEmpty: u64 = 0u64;
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ObjectBag {
    /// the ID of this bag
    pub id: UID,
    /// the number of key-value pairs in the bag
    pub size: u64,
}
impl MoveType for ObjectBag {
    type Package = super::Package;
    const MODULE: &'static str = "object_bag";
    const NAME: &'static str = "ObjectBag";
}
pub trait ArgumentObjectBag: PTBArgument {
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
impl ArgumentObjectBag for Argument {}
impl ArgumentObjectBag for ObjectId {
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
impl ArgumentObjectBag for ObjectReference {}
impl ArgumentObjectBag for Shared<ObjectId> {}
impl ArgumentObjectBag for SharedMut<ObjectId> {}
impl ArgumentObjectBag for Receiving<ObjectId> {}
/// Creates a new, empty bag
///
/// Returns: `0x2::object_bag::ObjectBag`.
pub async fn new(b: &mut PtbBuilder) -> Argument {
    b.move_call(
        b.package_id::<super::Package>(),
        "object_bag",
        "new",
        Vec::new(),
        vec![],
    )
}
/// Adds a key-value pair to the bag `bag: &mut ObjectBag`
/// Aborts with `iota::dynamic_field::EFieldAlreadyExists` if the bag already has an entry with
/// that key `k: K`.
pub async fn add<T0: MoveType, T1: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentObjectBag,
    arg1: impl ArgumentObject<()>,
    arg2: impl ArgumentObject<()>,
) {
    let a0 = arg0.into_argument_mut(b).await;
    let a1 = arg1.into_argument(b).await;
    let a2 = arg2.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "object_bag",
        "add",
        vec![< T0 as MoveType > ::type_tag(b), < T1 as MoveType > ::type_tag(b)],
        vec![a0, a1, a2],
    );
}
/// Immutably borrows the value associated with the key in the bag `bag: &ObjectBag`.
/// Aborts with `iota::dynamic_field::EFieldDoesNotExist` if the bag does not have an entry with
/// that key `k: K`.
/// Aborts with `iota::dynamic_field::EFieldTypeMismatch` if the bag has an entry for the key, but
/// the value does not have the specified type.
///
/// Returns: `&T1`.
pub async fn borrow<T0: MoveType, T1: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentObjectBag,
    arg1: impl ArgumentObject<()>,
) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    let a1 = arg1.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "object_bag",
        "borrow",
        vec![< T0 as MoveType > ::type_tag(b), < T1 as MoveType > ::type_tag(b)],
        vec![a0, a1],
    )
}
/// Mutably borrows the value associated with the key in the bag `bag: &mut ObjectBag`.
/// Aborts with `iota::dynamic_field::EFieldDoesNotExist` if the bag does not have an entry with
/// that key `k: K`.
/// Aborts with `iota::dynamic_field::EFieldTypeMismatch` if the bag has an entry for the key, but
/// the value does not have the specified type.
///
/// Returns: `&mut T1`.
pub async fn borrow_mut<T0: MoveType, T1: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentObjectBag,
    arg1: impl ArgumentObject<()>,
) -> Argument {
    let a0 = arg0.into_argument_mut(b).await;
    let a1 = arg1.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "object_bag",
        "borrow_mut",
        vec![< T0 as MoveType > ::type_tag(b), < T1 as MoveType > ::type_tag(b)],
        vec![a0, a1],
    )
}
/// Mutably borrows the key-value pair in the bag `bag: &mut ObjectBag` and returns the value.
/// Aborts with `iota::dynamic_field::EFieldDoesNotExist` if the bag does not have an entry with
/// that key `k: K`.
/// Aborts with `iota::dynamic_field::EFieldTypeMismatch` if the bag has an entry for the key, but
/// the value does not have the specified type.
///
/// Returns: `T1`.
pub async fn remove<T0: MoveType, T1: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentObjectBag,
    arg1: impl ArgumentObject<()>,
) -> Argument {
    let a0 = arg0.into_argument_mut(b).await;
    let a1 = arg1.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "object_bag",
        "remove",
        vec![< T0 as MoveType > ::type_tag(b), < T1 as MoveType > ::type_tag(b)],
        vec![a0, a1],
    )
}
/// Returns true iff there is an value associated with the key `k: K` in the bag `bag: &ObjectBag`
///
/// Returns: `bool`.
pub async fn contains<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentObjectBag,
    arg1: impl ArgumentObject<()>,
) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    let a1 = arg1.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "object_bag",
        "contains",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![a0, a1],
    )
}
/// Returns true iff there is an value associated with the key `k: K` in the bag `bag: &ObjectBag`
/// with an assigned value of type `V`
///
/// Returns: `bool`.
pub async fn contains_with_type<T0: MoveType, T1: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentObjectBag,
    arg1: impl ArgumentObject<()>,
) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    let a1 = arg1.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "object_bag",
        "contains_with_type",
        vec![< T0 as MoveType > ::type_tag(b), < T1 as MoveType > ::type_tag(b)],
        vec![a0, a1],
    )
}
/// Returns the size of the bag, the number of key-value pairs
///
/// Returns: `u64`.
pub async fn length(b: &mut PtbBuilder, arg0: impl ArgumentObjectBag) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "object_bag",
        "length",
        Vec::new(),
        vec![a0],
    )
}
/// Returns true iff the bag is empty (if `length` returns `0`)
///
/// Returns: `bool`.
pub async fn is_empty(b: &mut PtbBuilder, arg0: impl ArgumentObjectBag) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "object_bag",
        "is_empty",
        Vec::new(),
        vec![a0],
    )
}
/// Destroys an empty bag
/// Aborts with `EBagNotEmpty` if the bag still contains values
pub async fn destroy_empty(b: &mut PtbBuilder, arg0: impl ArgumentObjectBag) {
    let a0 = arg0.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "object_bag",
        "destroy_empty",
        Vec::new(),
        vec![a0],
    );
}
/// Returns the ID of the object associated with the key if the bag has an entry with key `k: K`
/// Returns none otherwise
///
/// Returns: `0x1::option::Option<0x2::object::ID>`.
pub async fn value_id<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentObjectBag,
    arg1: impl ArgumentObject<()>,
) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    let a1 = arg1.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "object_bag",
        "value_id",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![a0, a1],
    )
}
/// Read-only handle bound to a runtime package address. Use
/// via `super::Package::at(addr).<module>()` and chain
/// `<type>_tag()` to build a [`TypeTag`] without spinning up
/// a `PtbBuilder`.
pub struct ModuleAt {
    pub(crate) package: Address,
}
impl ModuleAt {
    pub fn objectbag_tag(&self) -> TypeTag {
        <ObjectBag as MoveType>::type_tag_at(self.package)
    }
}
