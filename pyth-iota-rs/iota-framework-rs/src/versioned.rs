#![allow(unused_imports, non_snake_case, non_upper_case_globals)]
use move_bindgen_runtime::*;
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;
/// Failed to upgrade the inner object due to invalid capability or new version.
pub const EInvalidUpgrade: u64 = 0u64;
/// A wrapper type that supports versioning of the inner type.
/// The inner type is a dynamic field of the Versioned object, and is keyed using version.
/// User of this type could load the inner object using corresponding type based on the version.
/// You can also upgrade the inner object to a new type version.
/// If you want to support lazy upgrade of the inner type, one caveat is that all APIs would have
/// to use mutable reference even if it's a read-only API.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Versioned {
    pub id: UID,
    pub version: u64,
}
impl MoveType for Versioned {
    type Package = super::Package;
    const MODULE: &'static str = "versioned";
    const NAME: &'static str = "Versioned";
}
pub trait ArgumentVersioned: PTBArgument {
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
impl ArgumentVersioned for Argument {}
impl ArgumentVersioned for ObjectId {
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
impl ArgumentVersioned for ObjectReference {}
impl ArgumentVersioned for Shared<ObjectId> {}
impl ArgumentVersioned for SharedMut<ObjectId> {}
impl ArgumentVersioned for Receiving<ObjectId> {}
/// Represents a hot potato object generated when we take out the dynamic field.
/// This is to make sure that we always put a new value back.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct VersionChangeCap {
    pub versioned_id: ID,
    pub old_version: u64,
}
impl MoveType for VersionChangeCap {
    type Package = super::Package;
    const MODULE: &'static str = "versioned";
    const NAME: &'static str = "VersionChangeCap";
}
impl MoveArg for VersionChangeCap {
    fn pure_bytes(self) -> PureBytes {
        PureBytes(::bcs::to_bytes(&self).expect("bcs serialization failed"))
    }
}
pub trait ArgumentVersionChangeCap: PTBArgument {
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
impl ArgumentVersionChangeCap for VersionChangeCap {}
impl ArgumentVersionChangeCap for Argument {}
/// Create a new Versioned object that contains a initial value of type `T` with an initial version.
///
/// Returns: `0x2::versioned::Versioned`.
pub async fn create<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl PureU64,
    arg1: impl ArgumentObject<()>,
) -> Argument {
    let a0 = arg0.into_argument(b).await;
    let a1 = arg1.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "versioned",
        "create",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![a0, a1],
    )
}
/// Get the current version of the inner type.
///
/// Returns: `u64`.
pub async fn version(b: &mut PtbBuilder, arg0: impl ArgumentVersioned) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "versioned",
        "version",
        Vec::new(),
        vec![a0],
    )
}
/// Load the inner value based on the current version. Caller specifies an expected type T.
/// If the type mismatch, the load will fail.
///
/// Returns: `&T0`.
pub async fn load_value<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentVersioned,
) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "versioned",
        "load_value",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![a0],
    )
}
/// Similar to load_value, but return a mutable reference.
///
/// Returns: `&mut T0`.
pub async fn load_value_mut<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentVersioned,
) -> Argument {
    let a0 = arg0.into_argument_mut(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "versioned",
        "load_value_mut",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![a0],
    )
}
/// Take the inner object out for upgrade. To ensure we always upgrade properly, a capability object is returned
/// and must be used when we upgrade.
///
/// Returns: `(T0, 0x2::versioned::VersionChangeCap)`.
pub async fn remove_value_for_upgrade<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentVersioned,
) -> (Argument, Argument) {
    let a0 = arg0.into_argument_mut(b).await;
    let __r = b
        .move_call_n(
            b.package_id::<super::Package>(),
            "versioned",
            "remove_value_for_upgrade",
            vec![< T0 as MoveType > ::type_tag(b)],
            vec![a0],
            2u16,
        );
    (__r[0], __r[1])
}
/// Upgrade the inner object with a new version and new value. Must use the capability returned
/// by calling remove_value_for_upgrade.
pub async fn upgrade<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentVersioned,
    arg1: impl PureU64,
    arg2: impl ArgumentObject<()>,
    arg3: impl ArgumentVersionChangeCap,
) {
    let a0 = arg0.into_argument_mut(b).await;
    let a1 = arg1.into_argument(b).await;
    let a2 = arg2.into_argument(b).await;
    let a3 = arg3.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "versioned",
        "upgrade",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![a0, a1, a2, a3],
    );
}
/// Destroy this Versioned container, and return the inner object.
///
/// Returns: `T0`.
pub async fn destroy<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentVersioned,
) -> Argument {
    let a0 = arg0.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "versioned",
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
    pub fn versioned_tag(&self) -> TypeTag {
        <Versioned as MoveType>::type_tag_at(self.package)
    }
    pub fn versionchangecap_tag(&self) -> TypeTag {
        <VersionChangeCap as MoveType>::type_tag_at(self.package)
    }
}
