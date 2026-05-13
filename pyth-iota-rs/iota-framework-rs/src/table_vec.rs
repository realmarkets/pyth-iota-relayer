#![allow(unused_imports, non_snake_case, non_upper_case_globals)]
//! A basic scalable vector library implemented using `Table`.
use move_bindgen_runtime::*;
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;
pub const EIndexOutOfBound: u64 = 0u64;
pub const ETableNonEmpty: u64 = 1u64;
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TableVec<T0: MoveType> {
    /// The contents of the table vector.
    pub contents: super::table::Table<u64, T0>,
    #[serde(skip)]
    pub _phantom_t0: PhantomData<T0>,
}
impl<T0: MoveType> MoveType for TableVec<T0> {
    type Package = super::Package;
    const MODULE: &'static str = "table_vec";
    const NAME: &'static str = "TableVec";
    fn type_params(addrs: &impl PackageAddrs) -> Vec<TypeTag> {
        vec![< T0 as MoveType > ::type_tag(addrs)]
    }
}
impl<T0: MoveType + ::serde::Serialize> MoveArg for TableVec<T0> {
    fn pure_bytes(self) -> PureBytes {
        PureBytes(::bcs::to_bytes(&self).expect("bcs serialization failed"))
    }
}
pub trait ArgumentTableVec<T0: MoveType>: PTBArgument {
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
impl<T0: MoveType + ::serde::Serialize> ArgumentTableVec<T0> for TableVec<T0> {}
impl<T0: MoveType> ArgumentTableVec<T0> for Argument {}
/// Create an empty TableVec.
///
/// Returns: `0x2::table_vec::TableVec<T0>`.
pub async fn empty<T0: MoveType>(b: &mut PtbBuilder) -> Argument {
    b.move_call(
        b.package_id::<super::Package>(),
        "table_vec",
        "empty",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![],
    )
}
/// Return a TableVec of size one containing element `e`.
///
/// Returns: `0x2::table_vec::TableVec<T0>`.
pub async fn singleton<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentObject<()>,
) -> Argument {
    let a0 = arg0.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "table_vec",
        "singleton",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![a0],
    )
}
/// Return the length of the TableVec.
///
/// Returns: `u64`.
pub async fn length<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentTableVec<T0>,
) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "table_vec",
        "length",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![a0],
    )
}
/// Return if the TableVec is empty or not.
///
/// Returns: `bool`.
pub async fn is_empty<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentTableVec<T0>,
) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "table_vec",
        "is_empty",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![a0],
    )
}
/// Acquire an immutable reference to the `i`th element of the TableVec `t`.
/// Aborts if `i` is out of bounds.
///
/// Returns: `&T0`.
pub async fn borrow<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentTableVec<T0>,
    arg1: impl PureU64,
) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    let a1 = arg1.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "table_vec",
        "borrow",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![a0, a1],
    )
}
/// Add element `e` to the end of the TableVec `t`.
pub async fn push_back<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentTableVec<T0>,
    arg1: impl ArgumentObject<()>,
) {
    let a0 = arg0.into_argument_mut(b).await;
    let a1 = arg1.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "table_vec",
        "push_back",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![a0, a1],
    );
}
/// Return a mutable reference to the `i`th element in the TableVec `t`.
/// Aborts if `i` is out of bounds.
///
/// Returns: `&mut T0`.
pub async fn borrow_mut<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentTableVec<T0>,
    arg1: impl PureU64,
) -> Argument {
    let a0 = arg0.into_argument_mut(b).await;
    let a1 = arg1.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "table_vec",
        "borrow_mut",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![a0, a1],
    )
}
/// Pop an element from the end of TableVec `t`.
/// Aborts if `t` is empty.
///
/// Returns: `T0`.
pub async fn pop_back<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentTableVec<T0>,
) -> Argument {
    let a0 = arg0.into_argument_mut(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "table_vec",
        "pop_back",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![a0],
    )
}
/// Destroy the TableVec `t`.
/// Aborts if `t` is not empty.
pub async fn destroy_empty<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentTableVec<T0>,
) {
    let a0 = arg0.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "table_vec",
        "destroy_empty",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![a0],
    );
}
/// Drop a possibly non-empty TableVec `t`.
/// Usable only if the value type `Element` has the `drop` ability
pub async fn drop<T0: MoveType>(b: &mut PtbBuilder, arg0: impl ArgumentTableVec<T0>) {
    let a0 = arg0.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "table_vec",
        "drop",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![a0],
    );
}
/// Swaps the elements at the `i`th and `j`th indices in the TableVec `t`.
/// Aborts if `i` or `j` is out of bounds.
pub async fn swap<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentTableVec<T0>,
    arg1: impl PureU64,
    arg2: impl PureU64,
) {
    let a0 = arg0.into_argument_mut(b).await;
    let a1 = arg1.into_argument(b).await;
    let a2 = arg2.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "table_vec",
        "swap",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![a0, a1, a2],
    );
}
/// Swap the `i`th element of the TableVec `t` with the last element and then pop the TableVec.
/// This is O(1), but does not preserve ordering of elements in the TableVec.
/// Aborts if `i` is out of bounds.
///
/// Returns: `T0`.
pub async fn swap_remove<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentTableVec<T0>,
    arg1: impl PureU64,
) -> Argument {
    let a0 = arg0.into_argument_mut(b).await;
    let a1 = arg1.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "table_vec",
        "swap_remove",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![a0, a1],
    )
}
