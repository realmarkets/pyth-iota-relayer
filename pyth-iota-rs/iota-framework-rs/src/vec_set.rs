#![allow(unused_imports, non_snake_case, non_upper_case_globals)]
use move_bindgen_runtime::*;
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;
/// This key already exists in the map
pub const EKeyAlreadyExists: u64 = 0u64;
/// This key does not exist in the map
pub const EKeyDoesNotExist: u64 = 1u64;
/// A set data structure backed by a vector. The set is guaranteed not to
/// contain duplicate keys. All operations are O(N) in the size of the set.
/// The intention of this data structure is only to provide the convenience
/// of programming against a set API. Sets that need sorted iteration rather
/// than insertion order iteration should be handwritten.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct VecSet<T0: MoveType> {
    pub contents: Vec<T0>,
}
impl<T0: MoveType> MoveType for VecSet<T0> {
    type Package = super::Package;
    const MODULE: &'static str = "vec_set";
    const NAME: &'static str = "VecSet";
    fn type_params(addrs: &impl PackageAddrs) -> Vec<TypeTag> {
        vec![< T0 as MoveType > ::type_tag(addrs)]
    }
}
impl<T0: MoveType + ::serde::Serialize> MoveArg for VecSet<T0> {
    fn pure_bytes(self) -> PureBytes {
        PureBytes(::bcs::to_bytes(&self).expect("bcs serialization failed"))
    }
}
pub trait ArgumentVecSet<T0: MoveType>: PTBArgument {
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
impl<T0: MoveType + ::serde::Serialize> ArgumentVecSet<T0> for VecSet<T0> {}
impl<T0: MoveType> ArgumentVecSet<T0> for Argument {}
/// Create an empty `VecSet`
///
/// Returns: `0x2::vec_set::VecSet<T0>`.
pub async fn empty<T0: MoveType>(b: &mut PtbBuilder) -> Argument {
    b.move_call(
        b.package_id::<super::Package>(),
        "vec_set",
        "empty",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![],
    )
}
/// Create a singleton `VecSet` that only contains one element.
///
/// Returns: `0x2::vec_set::VecSet<T0>`.
pub async fn singleton<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentObject<()>,
) -> Argument {
    let a0 = arg0.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "vec_set",
        "singleton",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![a0],
    )
}
/// Insert a `key` into self.
/// Aborts if `key` is already present in `self`.
pub async fn insert<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentVecSet<T0>,
    arg1: impl ArgumentObject<()>,
) {
    let a0 = arg0.into_argument_mut(b).await;
    let a1 = arg1.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "vec_set",
        "insert",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![a0, a1],
    );
}
/// Remove the entry `key` from self. Aborts if `key` is not present in `self`.
pub async fn remove<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentVecSet<T0>,
    arg1: impl ArgumentObject<()>,
) {
    let a0 = arg0.into_argument_mut(b).await;
    let a1 = arg1.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "vec_set",
        "remove",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![a0, a1],
    );
}
/// Return true if `self` contains an entry for `key`, false otherwise
///
/// Returns: `bool`.
pub async fn contains<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentVecSet<T0>,
    arg1: impl ArgumentObject<()>,
) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    let a1 = arg1.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "vec_set",
        "contains",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![a0, a1],
    )
}
/// Return the number of entries in `self`
///
/// Returns: `u64`.
pub async fn size<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentVecSet<T0>,
) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "vec_set",
        "size",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![a0],
    )
}
/// Return true if `self` has 0 elements, false otherwise
///
/// Returns: `bool`.
pub async fn is_empty<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentVecSet<T0>,
) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "vec_set",
        "is_empty",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![a0],
    )
}
/// Unpack `self` into vectors of keys.
/// The output keys are stored in insertion order, *not* sorted.
///
/// Returns: `vector<T0>`.
pub async fn into_keys<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentVecSet<T0>,
) -> Argument {
    let a0 = arg0.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "vec_set",
        "into_keys",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![a0],
    )
}
/// Construct a new `VecSet` from a vector of keys.
/// The keys are stored in insertion order (the original `keys` ordering)
/// and are *not* sorted.
///
/// Returns: `0x2::vec_set::VecSet<T0>`.
pub async fn from_keys<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl PureVec<T0>,
) -> Argument {
    let a0 = arg0.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "vec_set",
        "from_keys",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![a0],
    )
}
/// Borrow the `contents` of the `VecSet` to access content by index
/// without unpacking. The contents are stored in insertion order,
/// *not* sorted.
///
/// Returns: `&vector<T0>`.
pub async fn keys<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentVecSet<T0>,
) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "vec_set",
        "keys",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![a0],
    )
}
