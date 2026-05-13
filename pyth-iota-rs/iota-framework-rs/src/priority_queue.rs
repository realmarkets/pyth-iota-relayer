#![allow(unused_imports, non_snake_case, non_upper_case_globals)]
//! Priority queue implemented using a max heap.
use move_bindgen_runtime::*;
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;
/// For when heap is empty and there's no data to pop.
pub const EPopFromEmptyHeap: u64 = 0u64;
/// Struct representing a priority queue. The `entries` vector represents a max
/// heap structure, where entries[0] is the root, entries[1] and entries[2] are the
/// left child and right child of the root, etc. More generally, the children of
/// entries[i] are at i * 2 + 1 and i * 2 + 2. The max heap should have the invariant
/// that the parent node's priority is always higher than its child nodes' priorities.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PriorityQueue<T0: MoveType> {
    pub entries: Vec<Entry<T0>>,
}
impl<T0: MoveType> MoveType for PriorityQueue<T0> {
    type Package = super::Package;
    const MODULE: &'static str = "priority_queue";
    const NAME: &'static str = "PriorityQueue";
    fn type_params(addrs: &impl PackageAddrs) -> Vec<TypeTag> {
        vec![< T0 as MoveType > ::type_tag(addrs)]
    }
}
impl<T0: MoveType + ::serde::Serialize> MoveArg for PriorityQueue<T0> {
    fn pure_bytes(self) -> PureBytes {
        PureBytes(::bcs::to_bytes(&self).expect("bcs serialization failed"))
    }
}
pub trait ArgumentPriorityQueue<T0: MoveType>: PTBArgument {
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
impl<T0: MoveType + ::serde::Serialize> ArgumentPriorityQueue<T0> for PriorityQueue<T0> {}
impl<T0: MoveType> ArgumentPriorityQueue<T0> for Argument {}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Entry<T0: MoveType> {
    pub priority: u64,
    pub value: T0,
}
impl<T0: MoveType> MoveType for Entry<T0> {
    type Package = super::Package;
    const MODULE: &'static str = "priority_queue";
    const NAME: &'static str = "Entry";
    fn type_params(addrs: &impl PackageAddrs) -> Vec<TypeTag> {
        vec![< T0 as MoveType > ::type_tag(addrs)]
    }
}
impl<T0: MoveType + ::serde::Serialize> MoveArg for Entry<T0> {
    fn pure_bytes(self) -> PureBytes {
        PureBytes(::bcs::to_bytes(&self).expect("bcs serialization failed"))
    }
}
pub trait ArgumentEntry<T0: MoveType>: PTBArgument {
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
impl<T0: MoveType + ::serde::Serialize> ArgumentEntry<T0> for Entry<T0> {}
impl<T0: MoveType> ArgumentEntry<T0> for Argument {}
/// Create a new priority queue from the input entry vectors.
///
/// Returns: `0x2::priority_queue::PriorityQueue<T0>`.
pub async fn new<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl PureVec<Entry<T0>>,
) -> Argument {
    let a0 = arg0.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "priority_queue",
        "new",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![a0],
    )
}
/// Pop the entry with the highest priority value.
///
/// Returns: `(u64, T0)`.
pub async fn pop_max<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentPriorityQueue<T0>,
) -> (Argument, Argument) {
    let a0 = arg0.into_argument_mut(b).await;
    let __r = b
        .move_call_n(
            b.package_id::<super::Package>(),
            "priority_queue",
            "pop_max",
            vec![< T0 as MoveType > ::type_tag(b)],
            vec![a0],
            2u16,
        );
    (__r[0], __r[1])
}
/// Insert a new entry into the queue.
pub async fn insert<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentPriorityQueue<T0>,
    arg1: impl PureU64,
    arg2: impl ArgumentObject<()>,
) {
    let a0 = arg0.into_argument_mut(b).await;
    let a1 = arg1.into_argument(b).await;
    let a2 = arg2.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "priority_queue",
        "insert",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![a0, a1, a2],
    );
}
/// Returns: `0x2::priority_queue::Entry<T0>`.
pub async fn new_entry<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl PureU64,
    arg1: impl ArgumentObject<()>,
) -> Argument {
    let a0 = arg0.into_argument(b).await;
    let a1 = arg1.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "priority_queue",
        "new_entry",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![a0, a1],
    )
}
/// Returns: `vector<0x2::priority_queue::Entry<T0>>`.
pub async fn create_entries<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl PureVec<u64>,
    arg1: impl PureVec<T0>,
) -> Argument {
    let a0 = arg0.into_argument(b).await;
    let a1 = arg1.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "priority_queue",
        "create_entries",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![a0, a1],
    )
}
/// Returns: `vector<u64>`.
pub async fn priorities<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentPriorityQueue<T0>,
) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "priority_queue",
        "priorities",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![a0],
    )
}
