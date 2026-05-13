#![allow(unused_imports, non_snake_case, non_upper_case_globals)]
use move_bindgen_runtime::*;
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;
/// Shared an object that was previously created. Shared objects must currently
/// be constructed in the transaction they are created.
pub const ESharedNonNewObject: u64 = 0u64;
/// Serialization of the object failed.
pub const EBCSSerializationFailure: u64 = 1u64;
/// The object being received is not of the expected type.
pub const EReceivingObjectTypeMismatch: u64 = 2u64;
/// Represents both the case where the object does not exist and the case where the object is not
/// able to be accessed through the parent that is passed-in.
pub const EUnableToReceiveObject: u64 = 3u64;
/// Shared object operations such as wrapping, freezing, and converting to owned are not allowed.
pub const ESharedObjectOperationNotSupported: u64 = 4u64;
/// This represents the ability to `receive` an object of type `T`.
/// This type is ephemeral per-transaction and cannot be stored on-chain.
/// This does not represent the obligation to receive the object that it
/// references, but simply the ability to receive the object with object ID
/// `id` at version `version` if you can prove mutable access to the parent
/// object during the transaction.
/// Internals of this struct are opaque outside this module.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Receiving<T0: MoveType> {
    pub id: ID,
    pub version: u64,
    #[serde(skip)]
    pub _phantom_t0: PhantomData<T0>,
}
impl<T0: MoveType> MoveType for Receiving<T0> {
    type Package = super::Package;
    const MODULE: &'static str = "transfer";
    const NAME: &'static str = "Receiving";
    fn type_params(addrs: &impl PackageAddrs) -> Vec<TypeTag> {
        vec![< T0 as MoveType > ::type_tag(addrs)]
    }
}
impl<T0: MoveType + ::serde::Serialize> MoveArg for Receiving<T0> {
    fn pure_bytes(self) -> PureBytes {
        PureBytes(::bcs::to_bytes(&self).expect("bcs serialization failed"))
    }
}
pub trait ArgumentReceiving<T0: MoveType>: PTBArgument {
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
impl<T0: MoveType + ::serde::Serialize> ArgumentReceiving<T0> for Receiving<T0> {}
impl<T0: MoveType> ArgumentReceiving<T0> for Argument {}
/// Transfer ownership of `obj` to `recipient`. `obj` must have the `key` attribute,
/// which (in turn) ensures that `obj` has a globally unique ID. Note that if the recipient
/// address represents an object ID, the `obj` sent will be inaccessible after the transfer
/// (though they will be retrievable at a future date once new features are added).
/// This function has custom rules performed by the IOTA Move bytecode verifier that ensures
/// that `T` is an object defined in the module where `transfer` is invoked. Use
/// `public_transfer` to transfer an object with `store` outside of its module.
pub async fn transfer<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentObject<()>,
    arg1: impl PureAddress,
) {
    let a0 = arg0.into_argument(b).await;
    let a1 = arg1.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "transfer",
        "transfer",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![a0, a1],
    );
}
/// Transfer ownership of `obj` to `recipient`. `obj` must have the `key` attribute,
/// which (in turn) ensures that `obj` has a globally unique ID. Note that if the recipient
/// address represents an object ID, the `obj` sent will be inaccessible after the transfer
/// (though they will be retrievable at a future date once new features are added).
/// The object must have `store` to be transferred outside of its module.
pub async fn public_transfer<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentObject<()>,
    arg1: impl PureAddress,
) {
    let a0 = arg0.into_argument(b).await;
    let a1 = arg1.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "transfer",
        "public_transfer",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![a0, a1],
    );
}
/// Freeze `obj`. After freezing `obj` becomes immutable and can no longer be transferred or
/// mutated.
/// This function has custom rules performed by the IOTA Move bytecode verifier that ensures
/// that `T` is an object defined in the module where `freeze_object` is invoked. Use
/// `public_freeze_object` to freeze an object with `store` outside of its module.
pub async fn freeze_object<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentObject<()>,
) {
    let a0 = arg0.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "transfer",
        "freeze_object",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![a0],
    );
}
/// Freeze `obj`. After freezing `obj` becomes immutable and can no longer be transferred or
/// mutated.
/// The object must have `store` to be frozen outside of its module.
pub async fn public_freeze_object<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentObject<()>,
) {
    let a0 = arg0.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "transfer",
        "public_freeze_object",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![a0],
    );
}
/// Turn the given object into a mutable shared object that everyone can access and mutate.
/// This is irreversible, i.e. once an object is shared, it will stay shared forever.
/// Aborts with `ESharedNonNewObject` of the object being shared was not created in this
/// transaction. This restriction may be relaxed in the future.
/// This function has custom rules performed by the IOTA Move bytecode verifier that ensures
/// that `T` is an object defined in the module where `share_object` is invoked. Use
/// `public_share_object` to share an object with `store` outside of its module.
pub async fn share_object<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentObject<()>,
) {
    let a0 = arg0.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "transfer",
        "share_object",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![a0],
    );
}
/// Turn the given object into a mutable shared object that everyone can access and mutate.
/// This is irreversible, i.e. once an object is shared, it will stay shared forever.
/// Aborts with `ESharedNonNewObject` of the object being shared was not created in this
/// transaction. This restriction may be relaxed in the future.
/// The object must have `store` to be shared outside of its module.
pub async fn public_share_object<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentObject<()>,
) {
    let a0 = arg0.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "transfer",
        "public_share_object",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![a0],
    );
}
/// Given mutable (i.e., locked) access to the `parent` and a `Receiving` argument
/// referencing an object of type `T` owned by `parent` use the `to_receive`
/// argument to receive and return the referenced owned object of type `T`.
/// This function has custom rules performed by the IOTA Move bytecode verifier that ensures
/// that `T` is an object defined in the module where `receive` is invoked. Use
/// `public_receive` to receivne an object with `store` outside of its module.
///
/// Returns: `T0`.
pub async fn receive<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentObject<()>,
    arg1: impl ArgumentReceiving<T0>,
) -> Argument {
    let a0 = arg0.into_argument_mut(b).await;
    let a1 = arg1.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "transfer",
        "receive",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![a0, a1],
    )
}
/// Given mutable (i.e., locked) access to the `parent` and a `Receiving` argument
/// referencing an object of type `T` owned by `parent` use the `to_receive`
/// argument to receive and return the referenced owned object of type `T`.
/// The object must have `store` to be received outside of its defining module.
///
/// Returns: `T0`.
pub async fn public_receive<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentObject<()>,
    arg1: impl ArgumentReceiving<T0>,
) -> Argument {
    let a0 = arg0.into_argument_mut(b).await;
    let a1 = arg1.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "transfer",
        "public_receive",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![a0, a1],
    )
}
/// Return the object ID that the given `Receiving` argument references.
///
/// Returns: `0x2::object::ID`.
pub async fn receiving_object_id<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentReceiving<T0>,
) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "transfer",
        "receiving_object_id",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![a0],
    )
}
