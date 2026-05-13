#![allow(unused_imports, non_snake_case, non_upper_case_globals)]
//! A table is a map-like collection. But unlike a traditional collection, it's keys and values are
//! not stored within the `Table` value, but instead are stored using IOTA's object system. The
//! `Table` struct acts only as a handle into the object system to retrieve those keys and values.
//! Note that this means that `Table` values with exactly the same key-value mapping will not be
//! equal, with `==`, at runtime. For example
//! ```
//! let table1 = table::new<u64, bool>();
//! let table2 = table::new<u64, bool>();
//! table::add(&mut table1, 0, false);
//! table::add(&mut table1, 1, true);
//! table::add(&mut table2, 0, false);
//! table::add(&mut table2, 1, true);
//! // table1 does not equal table2, despite having the same entries
//! assert!(&table1 != &table2);
//! ```
use move_bindgen_runtime::*;
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;
pub const ETableNotEmpty: u64 = 0u64;
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Table<T0: MoveType, T1: MoveType> {
    /// the ID of this table
    pub id: UID,
    /// the number of key-value pairs in the table
    pub size: u64,
    #[serde(skip)]
    pub _phantom_t0: PhantomData<T0>,
    #[serde(skip)]
    pub _phantom_t1: PhantomData<T1>,
}
impl<T0: MoveType, T1: MoveType> MoveType for Table<T0, T1> {
    type Package = super::Package;
    const MODULE: &'static str = "table";
    const NAME: &'static str = "Table";
    fn type_params(addrs: &impl PackageAddrs) -> Vec<TypeTag> {
        vec![< T0 as MoveType > ::type_tag(addrs), < T1 as MoveType > ::type_tag(addrs)]
    }
}
pub trait ArgumentTable<T0: MoveType, T1: MoveType>: PTBArgument {
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
impl<T0: MoveType, T1: MoveType> ArgumentTable<T0, T1> for Argument {}
impl<T0: MoveType, T1: MoveType> ArgumentTable<T0, T1> for ObjectId {
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
impl<T0: MoveType, T1: MoveType> ArgumentTable<T0, T1> for ObjectReference {}
impl<T0: MoveType, T1: MoveType> ArgumentTable<T0, T1> for Shared<ObjectId> {}
impl<T0: MoveType, T1: MoveType> ArgumentTable<T0, T1> for SharedMut<ObjectId> {}
impl<T0: MoveType, T1: MoveType> ArgumentTable<T0, T1> for Receiving<ObjectId> {}
/// Creates a new, empty table
///
/// Returns: `0x2::table::Table<T0, T1>`.
pub async fn new<T0: MoveType, T1: MoveType>(b: &mut PtbBuilder) -> Argument {
    b.move_call(
        b.package_id::<super::Package>(),
        "table",
        "new",
        vec![< T0 as MoveType > ::type_tag(b), < T1 as MoveType > ::type_tag(b)],
        vec![],
    )
}
/// Adds a key-value pair to the table `table: &mut Table<K, V>`
/// Aborts with `iota::dynamic_field::EFieldAlreadyExists` if the table already has an entry with
/// that key `k: K`.
pub async fn add<T0: MoveType, T1: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentTable<T0, T1>,
    arg1: impl ArgumentObject<()>,
    arg2: impl ArgumentObject<()>,
) {
    let a0 = arg0.into_argument_mut(b).await;
    let a1 = arg1.into_argument(b).await;
    let a2 = arg2.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "table",
        "add",
        vec![< T0 as MoveType > ::type_tag(b), < T1 as MoveType > ::type_tag(b)],
        vec![a0, a1, a2],
    );
}
/// Immutable borrows the value associated with the key in the table `table: &Table<K, V>`.
/// Aborts with `iota::dynamic_field::EFieldDoesNotExist` if the table does not have an entry with
/// that key `k: K`.
///
/// Returns: `&T1`.
pub async fn borrow<T0: MoveType, T1: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentTable<T0, T1>,
    arg1: impl ArgumentObject<()>,
) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    let a1 = arg1.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "table",
        "borrow",
        vec![< T0 as MoveType > ::type_tag(b), < T1 as MoveType > ::type_tag(b)],
        vec![a0, a1],
    )
}
/// Mutably borrows the value associated with the key in the table `table: &mut Table<K, V>`.
/// Aborts with `iota::dynamic_field::EFieldDoesNotExist` if the table does not have an entry with
/// that key `k: K`.
///
/// Returns: `&mut T1`.
pub async fn borrow_mut<T0: MoveType, T1: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentTable<T0, T1>,
    arg1: impl ArgumentObject<()>,
) -> Argument {
    let a0 = arg0.into_argument_mut(b).await;
    let a1 = arg1.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "table",
        "borrow_mut",
        vec![< T0 as MoveType > ::type_tag(b), < T1 as MoveType > ::type_tag(b)],
        vec![a0, a1],
    )
}
/// Removes the key-value pair in the table `table: &mut Table<K, V>` and returns the value.
/// Aborts with `iota::dynamic_field::EFieldDoesNotExist` if the table does not have an entry with
/// that key `k: K`.
///
/// Returns: `T1`.
pub async fn remove<T0: MoveType, T1: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentTable<T0, T1>,
    arg1: impl ArgumentObject<()>,
) -> Argument {
    let a0 = arg0.into_argument_mut(b).await;
    let a1 = arg1.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "table",
        "remove",
        vec![< T0 as MoveType > ::type_tag(b), < T1 as MoveType > ::type_tag(b)],
        vec![a0, a1],
    )
}
/// Returns true iff there is a value associated with the key `k: K` in table `table: &Table<K, V>`
///
/// Returns: `bool`.
pub async fn contains<T0: MoveType, T1: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentTable<T0, T1>,
    arg1: impl ArgumentObject<()>,
) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    let a1 = arg1.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "table",
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
    arg0: impl ArgumentTable<T0, T1>,
) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "table",
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
    arg0: impl ArgumentTable<T0, T1>,
) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "table",
        "is_empty",
        vec![< T0 as MoveType > ::type_tag(b), < T1 as MoveType > ::type_tag(b)],
        vec![a0],
    )
}
/// Destroys an empty table
/// Aborts with `ETableNotEmpty` if the table still contains values
pub async fn destroy_empty<T0: MoveType, T1: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentTable<T0, T1>,
) {
    let a0 = arg0.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "table",
        "destroy_empty",
        vec![< T0 as MoveType > ::type_tag(b), < T1 as MoveType > ::type_tag(b)],
        vec![a0],
    );
}
/// Drop a possibly non-empty table.
/// Usable only if the value type `V` has the `drop` ability
pub async fn drop<T0: MoveType, T1: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentTable<T0, T1>,
) {
    let a0 = arg0.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "table",
        "drop",
        vec![< T0 as MoveType > ::type_tag(b), < T1 as MoveType > ::type_tag(b)],
        vec![a0],
    );
}
