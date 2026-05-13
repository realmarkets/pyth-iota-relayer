#![allow(unused_imports, non_snake_case, non_upper_case_globals)]
//! Defines a LabelerCap used for creating labels in a ``iota::timelock::Timelock`` object.
//! The LabelerCap can be created only be consuming an OTW, making then labels unique for each cap.
use move_bindgen_runtime::*;
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;
/// Error code for when a type passed to the `create_labeler_cap` function is not a one-time witness.
pub const ENotOneTimeWitness: u64 = 0u64;
/// `LabelerCap` allows to create labels of the specific type `L`.
/// Can be publicly transferred like any other object.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LabelerCap<T0: MoveType> {
    pub id: UID,
    #[serde(skip)]
    pub _phantom_t0: PhantomData<T0>,
}
impl<T0: MoveType> MoveType for LabelerCap<T0> {
    type Package = super::Package;
    const MODULE: &'static str = "labeler";
    const NAME: &'static str = "LabelerCap";
    fn type_params(addrs: &impl PackageAddrs) -> Vec<TypeTag> {
        vec![< T0 as MoveType > ::type_tag(addrs)]
    }
}
pub trait ArgumentLabelerCap<T0: MoveType>: PTBArgument {
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
impl<T0: MoveType> ArgumentLabelerCap<T0> for Argument {}
impl<T0: MoveType> ArgumentLabelerCap<T0> for ObjectId {
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
impl<T0: MoveType> ArgumentLabelerCap<T0> for ObjectReference {}
impl<T0: MoveType> ArgumentLabelerCap<T0> for Shared<ObjectId> {}
impl<T0: MoveType> ArgumentLabelerCap<T0> for SharedMut<ObjectId> {}
impl<T0: MoveType> ArgumentLabelerCap<T0> for Receiving<ObjectId> {}
/// Create a `LabelerCap` instance.
/// Can be created only by consuming a one time witness.
///
/// Returns: `0x2::labeler::LabelerCap<T0>`.
pub async fn create_labeler_cap<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentObject<()>,
) -> Argument {
    let a0 = arg0.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "labeler",
        "create_labeler_cap",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![a0],
    )
}
/// Delete a `LabelerCap` instance.
/// If a capability is destroyed, it is impossible to add the related labels.
pub async fn destroy_labeler_cap<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentLabelerCap<T0>,
) {
    let a0 = arg0.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "labeler",
        "destroy_labeler_cap",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![a0],
    );
}
