#![allow(unused_imports, non_snake_case, non_upper_case_globals)]
//! A simple library that enables hot-potato-locked borrow mechanics.
//!
//! With Programmable transactions, it is possible to borrow a value within
//! a transaction, use it and put back in the end. Hot-potato `Borrow` makes
//! sure the object is returned and was not swapped for another one.
use move_bindgen_runtime::*;
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;
/// The `Borrow` does not match the `Referent`.
pub const EWrongBorrow: u64 = 0u64;
/// An attempt to swap the `Referent.value` with another object of the same type.
pub const EWrongValue: u64 = 1u64;
/// An object wrapping a `T` and providing the borrow API.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Referent<T0: MoveType> {
    pub id: Address,
    pub value: Option<T0>,
}
impl<T0: MoveType> MoveType for Referent<T0> {
    type Package = super::Package;
    const MODULE: &'static str = "borrow";
    const NAME: &'static str = "Referent";
    fn type_params(addrs: &impl PackageAddrs) -> Vec<TypeTag> {
        vec![< T0 as MoveType > ::type_tag(addrs)]
    }
}
impl<T0: MoveType + ::serde::Serialize> MoveArg for Referent<T0> {
    fn pure_bytes(self) -> PureBytes {
        PureBytes(::bcs::to_bytes(&self).expect("bcs serialization failed"))
    }
}
pub trait ArgumentReferent<T0: MoveType>: PTBArgument {
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
impl<T0: MoveType + ::serde::Serialize> ArgumentReferent<T0> for Referent<T0> {}
impl<T0: MoveType> ArgumentReferent<T0> for Argument {}
/// A hot potato making sure the object is put back once borrowed.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Borrow {
    pub r#ref: Address,
    pub obj: ID,
}
impl MoveType for Borrow {
    type Package = super::Package;
    const MODULE: &'static str = "borrow";
    const NAME: &'static str = "Borrow";
}
impl MoveArg for Borrow {
    fn pure_bytes(self) -> PureBytes {
        PureBytes(::bcs::to_bytes(&self).expect("bcs serialization failed"))
    }
}
pub trait ArgumentBorrow: PTBArgument {
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
impl ArgumentBorrow for Borrow {}
impl ArgumentBorrow for Argument {}
/// Create a new `Referent` struct
///
/// Returns: `0x2::borrow::Referent<T0>`.
pub async fn new<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentObject<()>,
) -> Argument {
    let a0 = arg0.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "borrow",
        "new",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![a0],
    )
}
/// Borrow the `T` from the `Referent` receiving the `T` and a `Borrow`
/// hot potato.
///
/// Returns: `(T0, 0x2::borrow::Borrow)`.
pub async fn borrow<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentReferent<T0>,
) -> (Argument, Argument) {
    let a0 = arg0.into_argument_mut(b).await;
    let __r = b
        .move_call_n(
            b.package_id::<super::Package>(),
            "borrow",
            "borrow",
            vec![< T0 as MoveType > ::type_tag(b)],
            vec![a0],
            2u16,
        );
    (__r[0], __r[1])
}
/// Put an object and the `Borrow` hot potato back.
pub async fn put_back<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentReferent<T0>,
    arg1: impl ArgumentObject<()>,
    arg2: impl ArgumentBorrow,
) {
    let a0 = arg0.into_argument_mut(b).await;
    let a1 = arg1.into_argument(b).await;
    let a2 = arg2.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "borrow",
        "put_back",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![a0, a1, a2],
    );
}
/// Unpack the `Referent` struct and return the value.
///
/// Returns: `T0`.
pub async fn destroy<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentReferent<T0>,
) -> Argument {
    let a0 = arg0.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "borrow",
        "destroy",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![a0],
    )
}
/// Read-only handle bound to a runtime package address. Use
/// via `super::Package::at(addr).<module>()` and chain
/// `<type>_tag()` to build a [`TypeTag`] without spinning up
/// a `PtbBuilder`.
pub struct ModuleAt {
    pub(crate) package: Address,
}
impl ModuleAt {
    pub fn borrow_tag(&self) -> TypeTag {
        <Borrow as MoveType>::type_tag_at(self.package)
    }
}
