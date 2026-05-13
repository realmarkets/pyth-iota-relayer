#![allow(unused_imports, non_snake_case, non_upper_case_globals)]
//! Similar to `iota::table`, an `ObjectTable<K, V>` is a map-like collection. But unlike
//! `iota::table`, the values bound to these dynamic fields _must_ be objects themselves. This allows
//! for the objects to still exist within in storage, which may be important for external tools.
//! The difference is otherwise not observable from within Move.
use move_bindgen_runtime::*;
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;
pub const ETableNotEmpty: u64 = 0u64;
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ObjectTable<T0: MoveType, T1: MoveType> {
    /// the ID of this table
    pub id: UID,
    /// the number of key-value pairs in the table
    pub size: u64,
    #[serde(skip)]
    pub _phantom_t0: PhantomData<T0>,
    #[serde(skip)]
    pub _phantom_t1: PhantomData<T1>,
}
impl<T0: MoveType, T1: MoveType> MoveType for ObjectTable<T0, T1> {
    type Package = super::Package;
    const MODULE: &'static str = "object_table";
    const NAME: &'static str = "ObjectTable";
    fn type_params(addrs: &impl PackageAddrs) -> Vec<TypeTag> {
        vec![< T0 as MoveType > ::type_tag(addrs), < T1 as MoveType > ::type_tag(addrs)]
    }
}
pub trait ArgumentObjectTable<T0: MoveType, T1: MoveType>: PTBArgument {
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
impl<T0: MoveType, T1: MoveType> ArgumentObjectTable<T0, T1> for Argument {}
impl<T0: MoveType, T1: MoveType> ArgumentObjectTable<T0, T1> for ObjectId {
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
impl<T0: MoveType, T1: MoveType> ArgumentObjectTable<T0, T1> for ObjectReference {}
impl<T0: MoveType, T1: MoveType> ArgumentObjectTable<T0, T1> for Shared<ObjectId> {}
impl<T0: MoveType, T1: MoveType> ArgumentObjectTable<T0, T1> for SharedMut<ObjectId> {}
impl<T0: MoveType, T1: MoveType> ArgumentObjectTable<T0, T1> for Receiving<ObjectId> {}
/// Creates a new, empty table
///
/// Returns: `0x2::object_table::ObjectTable<T0, T1>`.
pub async fn new<T0: MoveType, T1: MoveType>(b: &mut PtbBuilder) -> Argument {
    b.move_call(
        b.package_id::<super::Package>(),
        "object_table",
        "new",
        vec![< T0 as MoveType > ::type_tag(b), < T1 as MoveType > ::type_tag(b)],
        vec![],
    )
}
/// Adds a key-value pair to the table `table: &mut ObjectTable<K, V>`
/// Aborts with `iota::dynamic_field::EFieldAlreadyExists` if the table already has an entry with
/// that key `k: K`.
pub async fn add<T0: MoveType, T1: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentObjectTable<T0, T1>,
    arg1: impl ArgumentObject<()>,
    arg2: impl ArgumentObject<()>,
) {
    let a0 = arg0.into_argument_mut(b).await;
    let a1 = arg1.into_argument(b).await;
    let a2 = arg2.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "object_table",
        "add",
        vec![< T0 as MoveType > ::type_tag(b), < T1 as MoveType > ::type_tag(b)],
        vec![a0, a1, a2],
    );
}
/// Immutable borrows the value associated with the key in the table `table: &ObjectTable<K, V>`.
/// Aborts with `iota::dynamic_field::EFieldDoesNotExist` if the table does not have an entry with
/// that key `k: K`.
///
/// Returns: `&T1`.
pub async fn borrow<T0: MoveType, T1: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentObjectTable<T0, T1>,
    arg1: impl ArgumentObject<()>,
) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    let a1 = arg1.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "object_table",
        "borrow",
        vec![< T0 as MoveType > ::type_tag(b), < T1 as MoveType > ::type_tag(b)],
        vec![a0, a1],
    )
}
/// Mutably borrows the value associated with the key in the table `table: &mut ObjectTable<K, V>`.
/// Aborts with `iota::dynamic_field::EFieldDoesNotExist` if the table does not have an entry with
/// that key `k: K`.
///
/// Returns: `&mut T1`.
pub async fn borrow_mut<T0: MoveType, T1: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentObjectTable<T0, T1>,
    arg1: impl ArgumentObject<()>,
) -> Argument {
    let a0 = arg0.into_argument_mut(b).await;
    let a1 = arg1.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "object_table",
        "borrow_mut",
        vec![< T0 as MoveType > ::type_tag(b), < T1 as MoveType > ::type_tag(b)],
        vec![a0, a1],
    )
}
/// Removes the key-value pair in the table `table: &mut ObjectTable<K, V>` and returns the value.
/// Aborts with `iota::dynamic_field::EFieldDoesNotExist` if the table does not have an entry with
/// that key `k: K`.
///
/// Returns: `T1`.
pub async fn remove<T0: MoveType, T1: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentObjectTable<T0, T1>,
    arg1: impl ArgumentObject<()>,
) -> Argument {
    let a0 = arg0.into_argument_mut(b).await;
    let a1 = arg1.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "object_table",
        "remove",
        vec![< T0 as MoveType > ::type_tag(b), < T1 as MoveType > ::type_tag(b)],
        vec![a0, a1],
    )
}
/// Returns true iff there is a value associated with the key `k: K` in table
/// `table: &ObjectTable<K, V>`
///
/// Returns: `bool`.
pub async fn contains<T0: MoveType, T1: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentObjectTable<T0, T1>,
    arg1: impl ArgumentObject<()>,
) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    let a1 = arg1.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "object_table",
        "contains",
        vec![< T0 as MoveType > ::type_tag(b), < T1 as MoveType > ::type_tag(b)],
        vec![a0, a1],
    )
}
/// Returns the size of the table, the number of key-value pairs
///
/// Returns: `u64`.
pub async fn length<T0: MoveType, T1: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentObjectTable<T0, T1>,
) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "object_table",
        "length",
        vec![< T0 as MoveType > ::type_tag(b), < T1 as MoveType > ::type_tag(b)],
        vec![a0],
    )
}
/// Returns true iff the table is empty (if `length` returns `0`)
///
/// Returns: `bool`.
pub async fn is_empty<T0: MoveType, T1: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentObjectTable<T0, T1>,
) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "object_table",
        "is_empty",
        vec![< T0 as MoveType > ::type_tag(b), < T1 as MoveType > ::type_tag(b)],
        vec![a0],
    )
}
/// Destroys an empty table
/// Aborts with `ETableNotEmpty` if the table still contains values
pub async fn destroy_empty<T0: MoveType, T1: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentObjectTable<T0, T1>,
) {
    let a0 = arg0.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "object_table",
        "destroy_empty",
        vec![< T0 as MoveType > ::type_tag(b), < T1 as MoveType > ::type_tag(b)],
        vec![a0],
    );
}
/// Returns the ID of the object associated with the key if the table has an entry with key `k: K`
/// Returns none otherwise
///
/// Returns: `0x1::option::Option<0x2::object::ID>`.
pub async fn value_id<T0: MoveType, T1: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentObjectTable<T0, T1>,
    arg1: impl ArgumentObject<()>,
) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    let a1 = arg1.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "object_table",
        "value_id",
        vec![< T0 as MoveType > ::type_tag(b), < T1 as MoveType > ::type_tag(b)],
        vec![a0, a1],
    )
}
