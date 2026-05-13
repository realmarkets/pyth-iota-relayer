#![allow(unused_imports, non_snake_case, non_upper_case_globals)]
//! In addition to the fields declared in its type definition, an IOTA object can have dynamic fields
//! that can be added after the object has been constructed. Unlike ordinary field names
//! (which are always statically declared identifiers) a dynamic field name can be any value with
//! the `copy`, `drop`, and `store` abilities, e.g. an integer, a boolean, or a string.
//! This gives IOTA programmers the flexibility to extend objects on-the-fly, and it also serves as a
//! building block for core collection types
use move_bindgen_runtime::*;
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;
/// The object already has a dynamic field with this name (with the value and type specified)
pub const EFieldAlreadyExists: u64 = 0u64;
/// Cannot load dynamic field.
/// The object does not have a dynamic field with this name (with the value and type specified)
pub const EFieldDoesNotExist: u64 = 1u64;
/// The object has a field with that name, but the value type does not match
pub const EFieldTypeMismatch: u64 = 2u64;
/// Failed to serialize the field's name
pub const EBCSSerializationFailure: u64 = 3u64;
/// The object added as a dynamic field was previously a shared object
pub const ESharedObjectOperationNotSupported: u64 = 4u64;
/// Internal object used for storing the field and value
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Field<T0: MoveType, T1: MoveType> {
    /// Determined by the hash of the object ID, the field name value and it's type,
    /// i.e. hash(parent.id || name || Name)
    pub id: UID,
    /// The value for the name of this field
    pub name: T0,
    /// The value bound to this field
    pub value: T1,
}
impl<T0: MoveType, T1: MoveType> MoveType for Field<T0, T1> {
    type Package = super::Package;
    const MODULE: &'static str = "dynamic_field";
    const NAME: &'static str = "Field";
    fn type_params(addrs: &impl PackageAddrs) -> Vec<TypeTag> {
        vec![< T0 as MoveType > ::type_tag(addrs), < T1 as MoveType > ::type_tag(addrs)]
    }
}
pub trait ArgumentField<T0: MoveType, T1: MoveType>: PTBArgument {
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
impl<T0: MoveType, T1: MoveType> ArgumentField<T0, T1> for Argument {}
impl<T0: MoveType, T1: MoveType> ArgumentField<T0, T1> for ObjectId {
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
impl<T0: MoveType, T1: MoveType> ArgumentField<T0, T1> for ObjectReference {}
impl<T0: MoveType, T1: MoveType> ArgumentField<T0, T1> for Shared<ObjectId> {}
impl<T0: MoveType, T1: MoveType> ArgumentField<T0, T1> for SharedMut<ObjectId> {}
impl<T0: MoveType, T1: MoveType> ArgumentField<T0, T1> for Receiving<ObjectId> {}
/// Adds a dynamic field to the object `object: &mut UID` at field specified by `name: Name`.
/// Aborts with `EFieldAlreadyExists` if the object already has that field with that name.
pub async fn add<T0: MoveType, T1: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentObject<()>,
    arg1: impl ArgumentObject<()>,
    arg2: impl ArgumentObject<()>,
) {
    let a0 = arg0.into_argument_mut(b).await;
    let a1 = arg1.into_argument(b).await;
    let a2 = arg2.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "dynamic_field",
        "add",
        vec![< T0 as MoveType > ::type_tag(b), < T1 as MoveType > ::type_tag(b)],
        vec![a0, a1, a2],
    );
}
/// Immutably borrows the `object`s dynamic field with the name specified by `name: Name`.
/// Aborts with `EFieldDoesNotExist` if the object does not have a field with that name.
/// Aborts with `EFieldTypeMismatch` if the field exists, but the value does not have the specified
/// type.
///
/// Returns: `&T1`.
pub async fn borrow<T0: MoveType, T1: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentObject<()>,
    arg1: impl ArgumentObject<()>,
) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    let a1 = arg1.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "dynamic_field",
        "borrow",
        vec![< T0 as MoveType > ::type_tag(b), < T1 as MoveType > ::type_tag(b)],
        vec![a0, a1],
    )
}
/// Mutably borrows the `object`s dynamic field with the name specified by `name: Name`.
/// Aborts with `EFieldDoesNotExist` if the object does not have a field with that name.
/// Aborts with `EFieldTypeMismatch` if the field exists, but the value does not have the specified
/// type.
///
/// Returns: `&mut T1`.
pub async fn borrow_mut<T0: MoveType, T1: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentObject<()>,
    arg1: impl ArgumentObject<()>,
) -> Argument {
    let a0 = arg0.into_argument_mut(b).await;
    let a1 = arg1.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "dynamic_field",
        "borrow_mut",
        vec![< T0 as MoveType > ::type_tag(b), < T1 as MoveType > ::type_tag(b)],
        vec![a0, a1],
    )
}
/// Removes the `object`s dynamic field with the name specified by `name: Name` and returns the
/// bound value.
/// Aborts with `EFieldDoesNotExist` if the object does not have a field with that name.
/// Aborts with `EFieldTypeMismatch` if the field exists, but the value does not have the specified
/// type.
///
/// Returns: `T1`.
pub async fn remove<T0: MoveType, T1: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentObject<()>,
    arg1: impl ArgumentObject<()>,
) -> Argument {
    let a0 = arg0.into_argument_mut(b).await;
    let a1 = arg1.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "dynamic_field",
        "remove",
        vec![< T0 as MoveType > ::type_tag(b), < T1 as MoveType > ::type_tag(b)],
        vec![a0, a1],
    )
}
/// Returns true if and only if the `object` has a dynamic field with the name specified by
/// `name: Name` but without specifying the `Value` type
///
/// Returns: `bool`.
pub async fn exists_<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentObject<()>,
    arg1: impl ArgumentObject<()>,
) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    let a1 = arg1.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "dynamic_field",
        "exists_",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![a0, a1],
    )
}
/// Removes the dynamic field if it exists. Returns the `some(Value)` if it exists or none otherwise.
///
/// Returns: `0x1::option::Option<T1>`.
pub async fn remove_if_exists<T0: MoveType, T1: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentObject<()>,
    arg1: impl ArgumentObject<()>,
) -> Argument {
    let a0 = arg0.into_argument_mut(b).await;
    let a1 = arg1.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "dynamic_field",
        "remove_if_exists",
        vec![< T0 as MoveType > ::type_tag(b), < T1 as MoveType > ::type_tag(b)],
        vec![a0, a1],
    )
}
/// Returns true if and only if the `object` has a dynamic field with the name specified by
/// `name: Name` with an assigned value of type `Value`.
///
/// Returns: `bool`.
pub async fn exists_with_type<T0: MoveType, T1: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentObject<()>,
    arg1: impl ArgumentObject<()>,
) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    let a1 = arg1.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "dynamic_field",
        "exists_with_type",
        vec![< T0 as MoveType > ::type_tag(b), < T1 as MoveType > ::type_tag(b)],
        vec![a0, a1],
    )
}
