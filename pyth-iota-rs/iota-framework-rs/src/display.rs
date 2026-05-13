#![allow(unused_imports, non_snake_case, non_upper_case_globals)]
//! Defines a Display struct which defines the way an Object
//! should be displayed. The intention is to keep data as independent
//! from its display as possible, protecting the development process
//! and keeping it separate from the ecosystem agreements.
//!
//! Each of the fields of the Display object should allow for pattern
//! substitution and filling-in the pieces using the data from the object T.
//!
//! More entry functions might be added in the future depending on the use cases.
use move_bindgen_runtime::*;
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;
/// For when T does not belong to the package `Publisher`.
pub const ENotOwner: u64 = 0u64;
/// For when vectors passed into one of the multiple insert functions
/// don't match in their lengths.
pub const EVecLengthMismatch: u64 = 1u64;
/// The Display<T> object. Defines the way a T instance should be
/// displayed. Display object can only be created and modified with
/// a PublisherCap, making sure that the rules are set by the owner
/// of the type.
///
/// Each of the display properties should support patterns outside
/// of the system, making it simpler to customize Display based
/// on the property values of an Object.
/// ```
/// // Example of a display object
/// Display<0x107A::nft::Nft> {
///  fields:
///    <name, "IOTEST Nft">
///    <link, "https://iotestnft.com/nft/{ id }">
///    <image, "https://api.iotestnft.com/nft/{ id }/svg">
///    <description, "One of many Iotest Nfts">
/// }
/// ```
///
/// Uses only String type due to external-facing nature of the object,
/// the property names have a priority over their types.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Display<T0: MoveType> {
    pub id: UID,
    /// Contains fields for display. Currently supported
    /// fields are: name, link, image and description.
    pub fields: super::vec_map::VecMap<String, String>,
    /// Version that can only be updated manually by the Publisher.
    pub version: u16,
    #[serde(skip)]
    pub _phantom_t0: PhantomData<T0>,
}
impl<T0: MoveType> MoveType for Display<T0> {
    type Package = super::Package;
    const MODULE: &'static str = "display";
    const NAME: &'static str = "Display";
    fn type_params(addrs: &impl PackageAddrs) -> Vec<TypeTag> {
        vec![< T0 as MoveType > ::type_tag(addrs)]
    }
}
pub trait ArgumentDisplay<T0: MoveType>: PTBArgument {
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
impl<T0: MoveType> ArgumentDisplay<T0> for Argument {}
impl<T0: MoveType> ArgumentDisplay<T0> for ObjectId {
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
impl<T0: MoveType> ArgumentDisplay<T0> for ObjectReference {}
impl<T0: MoveType> ArgumentDisplay<T0> for Shared<ObjectId> {}
impl<T0: MoveType> ArgumentDisplay<T0> for SharedMut<ObjectId> {}
impl<T0: MoveType> ArgumentDisplay<T0> for Receiving<ObjectId> {}
/// Event: emitted when a new Display object has been created for type T.
/// Type signature of the event corresponds to the type while id serves for
/// the discovery.
///
/// Since IOTA RPC supports querying events by type, finding a Display for the T
/// would be as simple as looking for the first event with `Display<T>`.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DisplayCreated<T0: MoveType> {
    pub id: ID,
    #[serde(skip)]
    pub _phantom_t0: PhantomData<T0>,
}
impl<T0: MoveType> MoveType for DisplayCreated<T0> {
    type Package = super::Package;
    const MODULE: &'static str = "display";
    const NAME: &'static str = "DisplayCreated";
    fn type_params(addrs: &impl PackageAddrs) -> Vec<TypeTag> {
        vec![< T0 as MoveType > ::type_tag(addrs)]
    }
}
impl<T0: MoveType + ::serde::Serialize> MoveArg for DisplayCreated<T0> {
    fn pure_bytes(self) -> PureBytes {
        PureBytes(::bcs::to_bytes(&self).expect("bcs serialization failed"))
    }
}
pub trait ArgumentDisplayCreated<T0: MoveType>: PTBArgument {
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
impl<T0: MoveType + ::serde::Serialize> ArgumentDisplayCreated<T0>
for DisplayCreated<T0> {}
impl<T0: MoveType> ArgumentDisplayCreated<T0> for Argument {}
/// Version of Display got updated -
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct VersionUpdated<T0: MoveType> {
    pub id: ID,
    pub version: u16,
    pub fields: super::vec_map::VecMap<String, String>,
    #[serde(skip)]
    pub _phantom_t0: PhantomData<T0>,
}
impl<T0: MoveType> MoveType for VersionUpdated<T0> {
    type Package = super::Package;
    const MODULE: &'static str = "display";
    const NAME: &'static str = "VersionUpdated";
    fn type_params(addrs: &impl PackageAddrs) -> Vec<TypeTag> {
        vec![< T0 as MoveType > ::type_tag(addrs)]
    }
}
impl<T0: MoveType + ::serde::Serialize> MoveArg for VersionUpdated<T0> {
    fn pure_bytes(self) -> PureBytes {
        PureBytes(::bcs::to_bytes(&self).expect("bcs serialization failed"))
    }
}
pub trait ArgumentVersionUpdated<T0: MoveType>: PTBArgument {
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
impl<T0: MoveType + ::serde::Serialize> ArgumentVersionUpdated<T0>
for VersionUpdated<T0> {}
impl<T0: MoveType> ArgumentVersionUpdated<T0> for Argument {}
/// Create an empty Display object. It can either be shared empty or filled
/// with data right away via cheaper `set_owned` method.
///
/// Returns: `0x2::display::Display<T0>`.
pub async fn new<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl super::package::ArgumentPublisher,
) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "display",
        "new",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![a0],
    )
}
/// Create a new Display<T> object with a set of fields.
///
/// Returns: `0x2::display::Display<T0>`.
pub async fn new_with_fields<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl super::package::ArgumentPublisher,
    arg1: impl PureVec<String>,
    arg2: impl PureVec<String>,
) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    let a1 = arg1.into_argument(b).await;
    let a2 = arg2.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "display",
        "new_with_fields",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![a0, a1, a2],
    )
}
/// Create a new empty Display<T> object and keep it.
pub async fn create_and_keep<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl super::package::ArgumentPublisher,
) {
    let a0 = arg0.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "display",
        "create_and_keep",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![a0],
    );
}
/// Manually bump the version and emit an event with the updated version's contents.
pub async fn update_version<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentDisplay<T0>,
) {
    let a0 = arg0.into_argument_mut(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "display",
        "update_version",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![a0],
    );
}
/// Sets a custom `name` field with the `value`.
pub async fn add<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentDisplay<T0>,
    arg1: impl PureString,
    arg2: impl PureString,
) {
    let a0 = arg0.into_argument_mut(b).await;
    let a1 = arg1.into_argument(b).await;
    let a2 = arg2.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "display",
        "add",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![a0, a1, a2],
    );
}
/// Sets multiple `fields` with `values`.
pub async fn add_multiple<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentDisplay<T0>,
    arg1: impl PureVec<String>,
    arg2: impl PureVec<String>,
) {
    let a0 = arg0.into_argument_mut(b).await;
    let a1 = arg1.into_argument(b).await;
    let a2 = arg2.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "display",
        "add_multiple",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![a0, a1, a2],
    );
}
/// Change the value of the field.
/// TODO (long run): version changes;
pub async fn edit<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentDisplay<T0>,
    arg1: impl PureString,
    arg2: impl PureString,
) {
    let a0 = arg0.into_argument_mut(b).await;
    let a1 = arg1.into_argument(b).await;
    let a2 = arg2.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "display",
        "edit",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![a0, a1, a2],
    );
}
/// Remove the key from the Display.
pub async fn remove<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentDisplay<T0>,
    arg1: impl PureString,
) {
    let a0 = arg0.into_argument_mut(b).await;
    let a1 = arg1.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "display",
        "remove",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![a0, a1],
    );
}
/// Authorization check; can be performed externally to implement protection rules for Display.
///
/// Returns: `bool`.
pub async fn is_authorized<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl super::package::ArgumentPublisher,
) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "display",
        "is_authorized",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![a0],
    )
}
/// Read the `version` field.
///
/// Returns: `u16`.
pub async fn version<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentDisplay<T0>,
) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "display",
        "version",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![a0],
    )
}
/// Read the `fields` field.
///
/// Returns: `&0x2::vec_map::VecMap<0x1::string::String, 0x1::string::String>`.
pub async fn fields<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentDisplay<T0>,
) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "display",
        "fields",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![a0],
    )
}
