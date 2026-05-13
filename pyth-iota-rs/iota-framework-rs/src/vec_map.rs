#![allow(unused_imports, non_snake_case, non_upper_case_globals)]
use move_bindgen_runtime::*;
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;
/// This key already exists in the map
pub const EKeyAlreadyExists: u64 = 0u64;
/// This key does not exist in the map
pub const EKeyDoesNotExist: u64 = 1u64;
/// Trying to destroy a map that is not empty
pub const EMapNotEmpty: u64 = 2u64;
/// Trying to access an element of the map at an invalid index
pub const EIndexOutOfBounds: u64 = 3u64;
/// Trying to pop from a map that is empty
pub const EMapEmpty: u64 = 4u64;
/// Trying to construct a map from keys and values of different lengths
pub const EUnequalLengths: u64 = 5u64;
/// A map data structure backed by a vector. The map is guaranteed not to contain duplicate keys, but entries
/// are *not* sorted by key. Entries are included in insertion order.
/// All operations are O(N) in the size of the map. The intention of this data structure is only to provide
/// the convenience of programming against a map API.
/// Large maps should use handwritten parent/child relationships instead.
/// Maps that need sorted iteration rather than insertion order iteration should also be handwritten.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct VecMap<T0: MoveType, T1: MoveType> {
    pub contents: Vec<Entry<T0, T1>>,
}
impl<T0: MoveType, T1: MoveType> MoveType for VecMap<T0, T1> {
    type Package = super::Package;
    const MODULE: &'static str = "vec_map";
    const NAME: &'static str = "VecMap";
    fn type_params(addrs: &impl PackageAddrs) -> Vec<TypeTag> {
        vec![< T0 as MoveType > ::type_tag(addrs), < T1 as MoveType > ::type_tag(addrs)]
    }
}
impl<T0: MoveType + ::serde::Serialize, T1: MoveType + ::serde::Serialize> MoveArg
for VecMap<T0, T1> {
    fn pure_bytes(self) -> PureBytes {
        PureBytes(::bcs::to_bytes(&self).expect("bcs serialization failed"))
    }
}
pub trait ArgumentVecMap<T0: MoveType, T1: MoveType>: PTBArgument {
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
> ArgumentVecMap<T0, T1> for VecMap<T0, T1> {}
impl<T0: MoveType, T1: MoveType> ArgumentVecMap<T0, T1> for Argument {}
/// An entry in the map
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Entry<T0: MoveType, T1: MoveType> {
    pub key: T0,
    pub value: T1,
}
impl<T0: MoveType, T1: MoveType> MoveType for Entry<T0, T1> {
    type Package = super::Package;
    const MODULE: &'static str = "vec_map";
    const NAME: &'static str = "Entry";
    fn type_params(addrs: &impl PackageAddrs) -> Vec<TypeTag> {
        vec![< T0 as MoveType > ::type_tag(addrs), < T1 as MoveType > ::type_tag(addrs)]
    }
}
impl<T0: MoveType + ::serde::Serialize, T1: MoveType + ::serde::Serialize> MoveArg
for Entry<T0, T1> {
    fn pure_bytes(self) -> PureBytes {
        PureBytes(::bcs::to_bytes(&self).expect("bcs serialization failed"))
    }
}
pub trait ArgumentEntry<T0: MoveType, T1: MoveType>: PTBArgument {
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
> ArgumentEntry<T0, T1> for Entry<T0, T1> {}
impl<T0: MoveType, T1: MoveType> ArgumentEntry<T0, T1> for Argument {}
/// Create an empty `VecMap`
///
/// Returns: `0x2::vec_map::VecMap<T0, T1>`.
pub async fn empty<T0: MoveType, T1: MoveType>(b: &mut PtbBuilder) -> Argument {
    b.move_call(
        b.package_id::<super::Package>(),
        "vec_map",
        "empty",
        vec![< T0 as MoveType > ::type_tag(b), < T1 as MoveType > ::type_tag(b)],
        vec![],
    )
}
/// Insert the entry `key` |-> `value` into `self`.
/// Aborts if `key` is already bound in `self`.
pub async fn insert<T0: MoveType, T1: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentVecMap<T0, T1>,
    arg1: impl ArgumentObject<()>,
    arg2: impl ArgumentObject<()>,
) {
    let a0 = arg0.into_argument_mut(b).await;
    let a1 = arg1.into_argument(b).await;
    let a2 = arg2.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "vec_map",
        "insert",
        vec![< T0 as MoveType > ::type_tag(b), < T1 as MoveType > ::type_tag(b)],
        vec![a0, a1, a2],
    );
}
/// Remove the entry `key` |-> `value` from self. Aborts if `key` is not bound in `self`.
///
/// Returns: `(T0, T1)`.
pub async fn remove<T0: MoveType, T1: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentVecMap<T0, T1>,
    arg1: impl ArgumentObject<()>,
) -> (Argument, Argument) {
    let a0 = arg0.into_argument_mut(b).await;
    let a1 = arg1.into_argument_ref(b).await;
    let __r = b
        .move_call_n(
            b.package_id::<super::Package>(),
            "vec_map",
            "remove",
            vec![< T0 as MoveType > ::type_tag(b), < T1 as MoveType > ::type_tag(b)],
            vec![a0, a1],
            2u16,
        );
    (__r[0], __r[1])
}
/// Pop the most recently inserted entry from the map. Aborts if the map is empty.
///
/// Returns: `(T0, T1)`.
pub async fn pop<T0: MoveType, T1: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentVecMap<T0, T1>,
) -> (Argument, Argument) {
    let a0 = arg0.into_argument_mut(b).await;
    let __r = b
        .move_call_n(
            b.package_id::<super::Package>(),
            "vec_map",
            "pop",
            vec![< T0 as MoveType > ::type_tag(b), < T1 as MoveType > ::type_tag(b)],
            vec![a0],
            2u16,
        );
    (__r[0], __r[1])
}
/// Get a mutable reference to the value bound to `key` in `self`.
/// Aborts if `key` is not bound in `self`.
///
/// Returns: `&mut T1`.
pub async fn get_mut<T0: MoveType, T1: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentVecMap<T0, T1>,
    arg1: impl ArgumentObject<()>,
) -> Argument {
    let a0 = arg0.into_argument_mut(b).await;
    let a1 = arg1.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "vec_map",
        "get_mut",
        vec![< T0 as MoveType > ::type_tag(b), < T1 as MoveType > ::type_tag(b)],
        vec![a0, a1],
    )
}
/// Get a reference to the value bound to `key` in `self`.
/// Aborts if `key` is not bound in `self`.
///
/// Returns: `&T1`.
pub async fn get<T0: MoveType, T1: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentVecMap<T0, T1>,
    arg1: impl ArgumentObject<()>,
) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    let a1 = arg1.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "vec_map",
        "get",
        vec![< T0 as MoveType > ::type_tag(b), < T1 as MoveType > ::type_tag(b)],
        vec![a0, a1],
    )
}
/// Safely try borrow a value bound to `key` in `self`.
/// Return Some(V) if the value exists, None otherwise.
/// Only works for a "copyable" value as references cannot be stored in `vector`.
///
/// Returns: `0x1::option::Option<T1>`.
pub async fn try_get<T0: MoveType, T1: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentVecMap<T0, T1>,
    arg1: impl ArgumentObject<()>,
) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    let a1 = arg1.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "vec_map",
        "try_get",
        vec![< T0 as MoveType > ::type_tag(b), < T1 as MoveType > ::type_tag(b)],
        vec![a0, a1],
    )
}
/// Return true if `self` contains an entry for `key`, false otherwise
///
/// Returns: `bool`.
pub async fn contains<T0: MoveType, T1: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentVecMap<T0, T1>,
    arg1: impl ArgumentObject<()>,
) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    let a1 = arg1.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "vec_map",
        "contains",
        vec![< T0 as MoveType > ::type_tag(b), < T1 as MoveType > ::type_tag(b)],
        vec![a0, a1],
    )
}
/// Return the number of entries in `self`
///
/// Returns: `u64`.
pub async fn size<T0: MoveType, T1: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentVecMap<T0, T1>,
) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "vec_map",
        "size",
        vec![< T0 as MoveType > ::type_tag(b), < T1 as MoveType > ::type_tag(b)],
        vec![a0],
    )
}
/// Return true if `self` has 0 elements, false otherwise
///
/// Returns: `bool`.
pub async fn is_empty<T0: MoveType, T1: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentVecMap<T0, T1>,
) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "vec_map",
        "is_empty",
        vec![< T0 as MoveType > ::type_tag(b), < T1 as MoveType > ::type_tag(b)],
        vec![a0],
    )
}
/// Destroy an empty map. Aborts if `self` is not empty
pub async fn destroy_empty<T0: MoveType, T1: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentVecMap<T0, T1>,
) {
    let a0 = arg0.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "vec_map",
        "destroy_empty",
        vec![< T0 as MoveType > ::type_tag(b), < T1 as MoveType > ::type_tag(b)],
        vec![a0],
    );
}
/// Unpack `self` into vectors of its keys and values.
/// The output keys and values are stored in insertion order, *not* sorted by key.
///
/// Returns: `(vector<T0>, vector<T1>)`.
pub async fn into_keys_values<T0: MoveType, T1: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentVecMap<T0, T1>,
) -> (Argument, Argument) {
    let a0 = arg0.into_argument(b).await;
    let __r = b
        .move_call_n(
            b.package_id::<super::Package>(),
            "vec_map",
            "into_keys_values",
            vec![< T0 as MoveType > ::type_tag(b), < T1 as MoveType > ::type_tag(b)],
            vec![a0],
            2u16,
        );
    (__r[0], __r[1])
}
/// Construct a new `VecMap` from two vectors, one for keys and one for values.
/// The key value pairs are associated via their indices in the vectors, e.g. the key at index i
/// in `keys` is associated with the value at index i in `values`.
/// The key value pairs are stored in insertion order (the original vectors ordering)
/// and are *not* sorted.
///
/// Returns: `0x2::vec_map::VecMap<T0, T1>`.
pub async fn from_keys_values<T0: MoveType, T1: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl PureVec<T0>,
    arg1: impl PureVec<T1>,
) -> Argument {
    let a0 = arg0.into_argument(b).await;
    let a1 = arg1.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "vec_map",
        "from_keys_values",
        vec![< T0 as MoveType > ::type_tag(b), < T1 as MoveType > ::type_tag(b)],
        vec![a0, a1],
    )
}
/// Returns a list of keys in the map.
/// Do not assume any particular ordering.
///
/// Returns: `vector<T0>`.
pub async fn keys<T0: MoveType, T1: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentVecMap<T0, T1>,
) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "vec_map",
        "keys",
        vec![< T0 as MoveType > ::type_tag(b), < T1 as MoveType > ::type_tag(b)],
        vec![a0],
    )
}
/// Find the index of `key` in `self`. Return `None` if `key` is not in `self`.
/// Note that map entries are stored in insertion order, *not* sorted by key.
///
/// Returns: `0x1::option::Option<u64>`.
pub async fn get_idx_opt<T0: MoveType, T1: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentVecMap<T0, T1>,
    arg1: impl ArgumentObject<()>,
) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    let a1 = arg1.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "vec_map",
        "get_idx_opt",
        vec![< T0 as MoveType > ::type_tag(b), < T1 as MoveType > ::type_tag(b)],
        vec![a0, a1],
    )
}
/// Find the index of `key` in `self`. Aborts if `key` is not in `self`.
/// Note that map entries are stored in insertion order, *not* sorted by key.
///
/// Returns: `u64`.
pub async fn get_idx<T0: MoveType, T1: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentVecMap<T0, T1>,
    arg1: impl ArgumentObject<()>,
) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    let a1 = arg1.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "vec_map",
        "get_idx",
        vec![< T0 as MoveType > ::type_tag(b), < T1 as MoveType > ::type_tag(b)],
        vec![a0, a1],
    )
}
/// Return a reference to the `idx`th entry of `self`. This gives direct access into the backing array of the map--use with caution.
/// Note that map entries are stored in insertion order, *not* sorted by key.
/// Aborts if `idx` is greater than or equal to `size(self)`
///
/// Returns: `(&T0, &T1)`.
pub async fn get_entry_by_idx<T0: MoveType, T1: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentVecMap<T0, T1>,
    arg1: impl PureU64,
) -> (Argument, Argument) {
    let a0 = arg0.into_argument_ref(b).await;
    let a1 = arg1.into_argument(b).await;
    let __r = b
        .move_call_n(
            b.package_id::<super::Package>(),
            "vec_map",
            "get_entry_by_idx",
            vec![< T0 as MoveType > ::type_tag(b), < T1 as MoveType > ::type_tag(b)],
            vec![a0, a1],
            2u16,
        );
    (__r[0], __r[1])
}
/// Return a mutable reference to the `idx`th entry of `self`. This gives direct access into the backing array of the map--use with caution.
/// Note that map entries are stored in insertion order, *not* sorted by key.
/// Aborts if `idx` is greater than or equal to `size(self)`
///
/// Returns: `(&T0, &mut T1)`.
pub async fn get_entry_by_idx_mut<T0: MoveType, T1: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentVecMap<T0, T1>,
    arg1: impl PureU64,
) -> (Argument, Argument) {
    let a0 = arg0.into_argument_mut(b).await;
    let a1 = arg1.into_argument(b).await;
    let __r = b
        .move_call_n(
            b.package_id::<super::Package>(),
            "vec_map",
            "get_entry_by_idx_mut",
            vec![< T0 as MoveType > ::type_tag(b), < T1 as MoveType > ::type_tag(b)],
            vec![a0, a1],
            2u16,
        );
    (__r[0], __r[1])
}
/// Remove the entry at index `idx` from self.
/// Aborts if `idx` is greater than or equal to `size(self)`
///
/// Returns: `(T0, T1)`.
pub async fn remove_entry_by_idx<T0: MoveType, T1: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentVecMap<T0, T1>,
    arg1: impl PureU64,
) -> (Argument, Argument) {
    let a0 = arg0.into_argument_mut(b).await;
    let a1 = arg1.into_argument(b).await;
    let __r = b
        .move_call_n(
            b.package_id::<super::Package>(),
            "vec_map",
            "remove_entry_by_idx",
            vec![< T0 as MoveType > ::type_tag(b), < T1 as MoveType > ::type_tag(b)],
            vec![a0, a1],
            2u16,
        );
    (__r[0], __r[1])
}
