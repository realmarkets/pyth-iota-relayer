#![allow(unused_imports, non_snake_case, non_upper_case_globals)]
//! A set data structure.
use move_bindgen_runtime::*;
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;
/// Empty struct. Used as the value type in mappings to encode a set
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Unit {
    pub dummy_field: bool,
}
impl MoveType for Unit {
    type Package = super::Package;
    const MODULE: &'static str = "set";
    const NAME: &'static str = "Unit";
}
impl MoveArg for Unit {
    fn pure_bytes(self) -> PureBytes {
        PureBytes(::bcs::to_bytes(&self).expect("bcs serialization failed"))
    }
}
pub trait ArgumentUnit: PTBArgument {
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
impl ArgumentUnit for Unit {}
impl ArgumentUnit for Argument {}
/// A set containing elements of type `A` with support for membership
/// checking.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Set<T0: MoveType> {
    pub keys: Vec<T0>,
    pub elems: ::iota_framework_rs::table::Table<T0, Unit>,
}
impl<T0: MoveType> MoveType for Set<T0> {
    type Package = super::Package;
    const MODULE: &'static str = "set";
    const NAME: &'static str = "Set";
    fn type_params(addrs: &impl PackageAddrs) -> Vec<TypeTag> {
        vec![< T0 as MoveType > ::type_tag(addrs)]
    }
}
impl<T0: MoveType + ::serde::Serialize> MoveArg for Set<T0> {
    fn pure_bytes(self) -> PureBytes {
        PureBytes(::bcs::to_bytes(&self).expect("bcs serialization failed"))
    }
}
pub trait ArgumentSet<T0: MoveType>: PTBArgument {
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
impl<T0: MoveType + ::serde::Serialize> ArgumentSet<T0> for Set<T0> {}
impl<T0: MoveType> ArgumentSet<T0> for Argument {}
/// Create a new Set.
///
/// Returns: `0xff00000000000001::set::Set<T0>`.
pub async fn new<T0: MoveType>(b: &mut PtbBuilder) -> Argument {
    b.move_call(
        b.package_id::<super::Package>(),
        "set",
        "new",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![],
    )
}
/// Add a new element to the set.
/// Aborts if the element already exists
pub async fn add<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentSet<T0>,
    arg1: impl ArgumentObject<()>,
) {
    let a0 = arg0.into_argument_mut(b).await;
    let a1 = arg1.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "set",
        "add",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![a0, a1],
    );
}
/// Returns true iff `set` contains an entry for `key`.
///
/// Returns: `bool`.
pub async fn contains<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentSet<T0>,
    arg1: impl ArgumentObject<()>,
) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    let a1 = arg1.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "set",
        "contains",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![a0, a1],
    )
}
/// Removes all elements from the set
pub async fn empty<T0: MoveType>(b: &mut PtbBuilder, arg0: impl ArgumentSet<T0>) {
    let a0 = arg0.into_argument_mut(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "set",
        "empty",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![a0],
    );
}
/// Read-only handle bound to a runtime package address. Use
/// via `super::Package::at(addr).<module>()` and chain
/// `<type>_tag()` to build a [`TypeTag`] without spinning up
/// a `PtbBuilder`.
pub struct ModuleAt {
    pub(crate) package: Address,
}
impl ModuleAt {
    pub fn unit_tag(&self) -> TypeTag {
        <Unit as MoveType>::type_tag_at(self.package)
    }
}
