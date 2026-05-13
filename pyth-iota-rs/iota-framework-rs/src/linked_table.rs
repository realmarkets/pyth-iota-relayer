#![allow(unused_imports, non_snake_case, non_upper_case_globals)]
//! Similar to `iota::table` but the values are linked together, allowing for ordered insertion and
//! removal
use move_bindgen_runtime::*;
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;
pub const ETableNotEmpty: u64 = 0u64;
pub const ETableIsEmpty: u64 = 1u64;
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LinkedTable<T0: MoveType, T1: MoveType> {
    /// the ID of this table
    pub id: UID,
    /// the number of key-value pairs in the table
    pub size: u64,
    /// the front of the table, i.e. the key of the first entry
    pub head: Option<T0>,
    /// the back of the table, i.e. the key of the last entry
    pub tail: Option<T0>,
    #[serde(skip)]
    pub _phantom_t1: PhantomData<T1>,
}
impl<T0: MoveType, T1: MoveType> MoveType for LinkedTable<T0, T1> {
    type Package = super::Package;
    const MODULE: &'static str = "linked_table";
    const NAME: &'static str = "LinkedTable";
    fn type_params(addrs: &impl PackageAddrs) -> Vec<TypeTag> {
        vec![< T0 as MoveType > ::type_tag(addrs), < T1 as MoveType > ::type_tag(addrs)]
    }
}
pub trait ArgumentLinkedTable<T0: MoveType, T1: MoveType>: PTBArgument {
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
impl<T0: MoveType, T1: MoveType> ArgumentLinkedTable<T0, T1> for Argument {}
impl<T0: MoveType, T1: MoveType> ArgumentLinkedTable<T0, T1> for ObjectId {
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
impl<T0: MoveType, T1: MoveType> ArgumentLinkedTable<T0, T1> for ObjectReference {}
impl<T0: MoveType, T1: MoveType> ArgumentLinkedTable<T0, T1> for Shared<ObjectId> {}
impl<T0: MoveType, T1: MoveType> ArgumentLinkedTable<T0, T1> for SharedMut<ObjectId> {}
impl<T0: MoveType, T1: MoveType> ArgumentLinkedTable<T0, T1> for Receiving<ObjectId> {}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Node<T0: MoveType, T1: MoveType> {
    /// the previous key
    pub prev: Option<T0>,
    /// the next key
    pub next: Option<T0>,
    /// the value being stored
    pub value: T1,
}
impl<T0: MoveType, T1: MoveType> MoveType for Node<T0, T1> {
    type Package = super::Package;
    const MODULE: &'static str = "linked_table";
    const NAME: &'static str = "Node";
    fn type_params(addrs: &impl PackageAddrs) -> Vec<TypeTag> {
        vec![< T0 as MoveType > ::type_tag(addrs), < T1 as MoveType > ::type_tag(addrs)]
    }
}
impl<T0: MoveType + ::serde::Serialize, T1: MoveType + ::serde::Serialize> MoveArg
for Node<T0, T1> {
    fn pure_bytes(self) -> PureBytes {
        PureBytes(::bcs::to_bytes(&self).expect("bcs serialization failed"))
    }
}
pub trait ArgumentNode<T0: MoveType, T1: MoveType>: PTBArgument {
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
impl<
    T0: MoveType + ::serde::Serialize,
    T1: MoveType + ::serde::Serialize,
> ArgumentNode<T0, T1> for Node<T0, T1> {}
impl<T0: MoveType, T1: MoveType> ArgumentNode<T0, T1> for Argument {}
/// Creates a new, empty table
///
/// Returns: `0x2::linked_table::LinkedTable<T0, T1>`.
pub async fn new<T0: MoveType, T1: MoveType>(b: &mut PtbBuilder) -> Argument {
    b.move_call(
        b.package_id::<super::Package>(),
        "linked_table",
        "new",
        vec![< T0 as MoveType > ::type_tag(b), < T1 as MoveType > ::type_tag(b)],
        vec![],
    )
}
/// Returns the key for the first element in the table, or None if the table is empty
///
/// Returns: `&0x1::option::Option<T0>`.
pub async fn front<T0: MoveType, T1: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentLinkedTable<T0, T1>,
) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "linked_table",
        "front",
        vec![< T0 as MoveType > ::type_tag(b), < T1 as MoveType > ::type_tag(b)],
        vec![a0],
    )
}
/// Returns the key for the last element in the table, or None if the table is empty
///
/// Returns: `&0x1::option::Option<T0>`.
pub async fn back<T0: MoveType, T1: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentLinkedTable<T0, T1>,
) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "linked_table",
        "back",
        vec![< T0 as MoveType > ::type_tag(b), < T1 as MoveType > ::type_tag(b)],
        vec![a0],
    )
}
/// Inserts a key-value pair at the front of the table, i.e. the newly inserted pair will be
/// the first element in the table
/// Aborts with `iota::dynamic_field::EFieldAlreadyExists` if the table already has an entry with
/// that key `k: K`.
pub async fn push_front<T0: MoveType, T1: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentLinkedTable<T0, T1>,
    arg1: impl ArgumentObject<()>,
    arg2: impl ArgumentObject<()>,
) {
    let a0 = arg0.into_argument_mut(b).await;
    let a1 = arg1.into_argument(b).await;
    let a2 = arg2.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "linked_table",
        "push_front",
        vec![< T0 as MoveType > ::type_tag(b), < T1 as MoveType > ::type_tag(b)],
        vec![a0, a1, a2],
    );
}
/// Inserts a key-value pair at the back of the table, i.e. the newly inserted pair will be
/// the last element in the table
/// Aborts with `iota::dynamic_field::EFieldAlreadyExists` if the table already has an entry with
/// that key `k: K`.
pub async fn push_back<T0: MoveType, T1: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentLinkedTable<T0, T1>,
    arg1: impl ArgumentObject<()>,
    arg2: impl ArgumentObject<()>,
) {
    let a0 = arg0.into_argument_mut(b).await;
    let a1 = arg1.into_argument(b).await;
    let a2 = arg2.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "linked_table",
        "push_back",
        vec![< T0 as MoveType > ::type_tag(b), < T1 as MoveType > ::type_tag(b)],
        vec![a0, a1, a2],
    );
}
/// Immutable borrows the value associated with the key in the table `table: &LinkedTable<K, V>`.
/// Aborts with `iota::dynamic_field::EFieldDoesNotExist` if the table does not have an entry with
/// that key `k: K`.
///
/// Returns: `&T1`.
pub async fn borrow<T0: MoveType, T1: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentLinkedTable<T0, T1>,
    arg1: impl ArgumentObject<()>,
) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    let a1 = arg1.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "linked_table",
        "borrow",
        vec![< T0 as MoveType > ::type_tag(b), < T1 as MoveType > ::type_tag(b)],
        vec![a0, a1],
    )
}
/// Mutably borrows the value associated with the key in the table `table: &mut LinkedTable<K, V>`.
/// Aborts with `iota::dynamic_field::EFieldDoesNotExist` if the table does not have an entry with
/// that key `k: K`.
///
/// Returns: `&mut T1`.
pub async fn borrow_mut<T0: MoveType, T1: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentLinkedTable<T0, T1>,
    arg1: impl ArgumentObject<()>,
) -> Argument {
    let a0 = arg0.into_argument_mut(b).await;
    let a1 = arg1.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "linked_table",
        "borrow_mut",
        vec![< T0 as MoveType > ::type_tag(b), < T1 as MoveType > ::type_tag(b)],
        vec![a0, a1],
    )
}
/// Borrows the key for the previous entry of the specified key `k: K` in the table
/// `table: &LinkedTable<K, V>`. Returns None if the entry does not have a predecessor.
/// Aborts with `iota::dynamic_field::EFieldDoesNotExist` if the table does not have an entry with
/// that key `k: K`
///
/// Returns: `&0x1::option::Option<T0>`.
pub async fn prev<T0: MoveType, T1: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentLinkedTable<T0, T1>,
    arg1: impl ArgumentObject<()>,
) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    let a1 = arg1.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "linked_table",
        "prev",
        vec![< T0 as MoveType > ::type_tag(b), < T1 as MoveType > ::type_tag(b)],
        vec![a0, a1],
    )
}
/// Borrows the key for the next entry of the specified key `k: K` in the table
/// `table: &LinkedTable<K, V>`. Returns None if the entry does not have a predecessor.
/// Aborts with `iota::dynamic_field::EFieldDoesNotExist` if the table does not have an entry with
/// that key `k: K`
///
/// Returns: `&0x1::option::Option<T0>`.
pub async fn next<T0: MoveType, T1: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentLinkedTable<T0, T1>,
    arg1: impl ArgumentObject<()>,
) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    let a1 = arg1.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "linked_table",
        "next",
        vec![< T0 as MoveType > ::type_tag(b), < T1 as MoveType > ::type_tag(b)],
        vec![a0, a1],
    )
}
/// Removes the key-value pair in the table `table: &mut LinkedTable<K, V>` and returns the value.
/// This splices the element out of the ordering.
/// Aborts with `iota::dynamic_field::EFieldDoesNotExist` if the table does not have an entry with
/// that key `k: K`. Note: this is also what happens when the table is empty.
///
/// Returns: `T1`.
pub async fn remove<T0: MoveType, T1: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentLinkedTable<T0, T1>,
    arg1: impl ArgumentObject<()>,
) -> Argument {
    let a0 = arg0.into_argument_mut(b).await;
    let a1 = arg1.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "linked_table",
        "remove",
        vec![< T0 as MoveType > ::type_tag(b), < T1 as MoveType > ::type_tag(b)],
        vec![a0, a1],
    )
}
/// Removes the front of the table `table: &mut LinkedTable<K, V>` and returns the value.
/// Aborts with `ETableIsEmpty` if the table is empty
///
/// Returns: `(T0, T1)`.
pub async fn pop_front<T0: MoveType, T1: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentLinkedTable<T0, T1>,
) -> (Argument, Argument) {
    let a0 = arg0.into_argument_mut(b).await;
    let __r = b
        .move_call_n(
            b.package_id::<super::Package>(),
            "linked_table",
            "pop_front",
            vec![< T0 as MoveType > ::type_tag(b), < T1 as MoveType > ::type_tag(b)],
            vec![a0],
            2u16,
        );
    (__r[0], __r[1])
}
/// Removes the back of the table `table: &mut LinkedTable<K, V>` and returns the value.
/// Aborts with `ETableIsEmpty` if the table is empty
///
/// Returns: `(T0, T1)`.
pub async fn pop_back<T0: MoveType, T1: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentLinkedTable<T0, T1>,
) -> (Argument, Argument) {
    let a0 = arg0.into_argument_mut(b).await;
    let __r = b
        .move_call_n(
            b.package_id::<super::Package>(),
            "linked_table",
            "pop_back",
            vec![< T0 as MoveType > ::type_tag(b), < T1 as MoveType > ::type_tag(b)],
            vec![a0],
            2u16,
        );
    (__r[0], __r[1])
}
/// Returns true iff there is a value associated with the key `k: K` in table
/// `table: &LinkedTable<K, V>`
///
/// Returns: `bool`.
pub async fn contains<T0: MoveType, T1: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentLinkedTable<T0, T1>,
    arg1: impl ArgumentObject<()>,
) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    let a1 = arg1.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "linked_table",
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
    arg0: impl ArgumentLinkedTable<T0, T1>,
) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "linked_table",
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
    arg0: impl ArgumentLinkedTable<T0, T1>,
) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "linked_table",
        "is_empty",
        vec![< T0 as MoveType > ::type_tag(b), < T1 as MoveType > ::type_tag(b)],
        vec![a0],
    )
}
/// Destroys an empty table
/// Aborts with `ETableNotEmpty` if the table still contains values
pub async fn destroy_empty<T0: MoveType, T1: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentLinkedTable<T0, T1>,
) {
    let a0 = arg0.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "linked_table",
        "destroy_empty",
        vec![< T0 as MoveType > ::type_tag(b), < T1 as MoveType > ::type_tag(b)],
        vec![a0],
    );
}
/// Drop a possibly non-empty table.
/// Usable only if the value type `V` has the `drop` ability
pub async fn drop<T0: MoveType, T1: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentLinkedTable<T0, T1>,
) {
    let a0 = arg0.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "linked_table",
        "drop",
        vec![< T0 as MoveType > ::type_tag(b), < T1 as MoveType > ::type_tag(b)],
        vec![a0],
    );
}
