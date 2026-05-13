#![allow(unused_imports, non_snake_case, non_upper_case_globals)]
use move_bindgen_runtime::*;
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;
pub const ENotOwner: u64 = 0u64;
pub const EExtensionNotAllowed: u64 = 2u64;
pub const EExtensionNotInstalled: u64 = 3u64;
pub const PLACE: u128 = 1u128;
pub const LOCK: u128 = 2u128;
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Extension {
    pub storage: super::bag::Bag,
    pub permissions: u128,
    pub is_enabled: bool,
}
impl MoveType for Extension {
    type Package = super::Package;
    const MODULE: &'static str = "kiosk_extension";
    const NAME: &'static str = "Extension";
}
impl MoveArg for Extension {
    fn pure_bytes(self) -> PureBytes {
        PureBytes(::bcs::to_bytes(&self).expect("bcs serialization failed"))
    }
}
pub trait ArgumentExtension: PTBArgument {
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
impl ArgumentExtension for Extension {}
impl ArgumentExtension for Argument {}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ExtensionKey<T0: MoveType> {
    pub dummy_field: bool,
    #[serde(skip)]
    pub _phantom_t0: PhantomData<T0>,
}
impl<T0: MoveType> MoveType for ExtensionKey<T0> {
    type Package = super::Package;
    const MODULE: &'static str = "kiosk_extension";
    const NAME: &'static str = "ExtensionKey";
    fn type_params(addrs: &impl PackageAddrs) -> Vec<TypeTag> {
        vec![< T0 as MoveType > ::type_tag(addrs)]
    }
}
impl<T0: MoveType + ::serde::Serialize> MoveArg for ExtensionKey<T0> {
    fn pure_bytes(self) -> PureBytes {
        PureBytes(::bcs::to_bytes(&self).expect("bcs serialization failed"))
    }
}
pub trait ArgumentExtensionKey<T0: MoveType>: PTBArgument {
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
impl<T0: MoveType + ::serde::Serialize> ArgumentExtensionKey<T0> for ExtensionKey<T0> {}
impl<T0: MoveType> ArgumentExtensionKey<T0> for Argument {}
pub async fn add<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentObject<()>,
    arg1: impl super::kiosk::ArgumentKiosk,
    arg2: impl super::kiosk::ArgumentKioskOwnerCap,
    arg3: impl PureU128,
) {
    let a0 = arg0.into_argument(b).await;
    let a1 = arg1.into_argument_mut(b).await;
    let a2 = arg2.into_argument_ref(b).await;
    let a3 = arg3.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "kiosk_extension",
        "add",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![a0, a1, a2, a3],
    );
}
pub async fn disable<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl super::kiosk::ArgumentKiosk,
    arg1: impl super::kiosk::ArgumentKioskOwnerCap,
) {
    let a0 = arg0.into_argument_mut(b).await;
    let a1 = arg1.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "kiosk_extension",
        "disable",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![a0, a1],
    );
}
pub async fn enable<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl super::kiosk::ArgumentKiosk,
    arg1: impl super::kiosk::ArgumentKioskOwnerCap,
) {
    let a0 = arg0.into_argument_mut(b).await;
    let a1 = arg1.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "kiosk_extension",
        "enable",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![a0, a1],
    );
}
pub async fn remove<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl super::kiosk::ArgumentKiosk,
    arg1: impl super::kiosk::ArgumentKioskOwnerCap,
) {
    let a0 = arg0.into_argument_mut(b).await;
    let a1 = arg1.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "kiosk_extension",
        "remove",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![a0, a1],
    );
}
/// Returns: `&0x2::bag::Bag`.
pub async fn storage<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentObject<()>,
    arg1: impl super::kiosk::ArgumentKiosk,
) -> Argument {
    let a0 = arg0.into_argument(b).await;
    let a1 = arg1.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "kiosk_extension",
        "storage",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![a0, a1],
    )
}
/// Returns: `&mut 0x2::bag::Bag`.
pub async fn storage_mut<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentObject<()>,
    arg1: impl super::kiosk::ArgumentKiosk,
) -> Argument {
    let a0 = arg0.into_argument(b).await;
    let a1 = arg1.into_argument_mut(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "kiosk_extension",
        "storage_mut",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![a0, a1],
    )
}
pub async fn place<T0: MoveType, T1: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentObject<()>,
    arg1: impl super::kiosk::ArgumentKiosk,
    arg2: impl ArgumentObject<()>,
    arg3: impl super::transfer_policy::ArgumentTransferPolicy<T1>,
) {
    let a0 = arg0.into_argument(b).await;
    let a1 = arg1.into_argument_mut(b).await;
    let a2 = arg2.into_argument(b).await;
    let a3 = arg3.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "kiosk_extension",
        "place",
        vec![< T0 as MoveType > ::type_tag(b), < T1 as MoveType > ::type_tag(b)],
        vec![a0, a1, a2, a3],
    );
}
pub async fn lock<T0: MoveType, T1: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentObject<()>,
    arg1: impl super::kiosk::ArgumentKiosk,
    arg2: impl ArgumentObject<()>,
    arg3: impl super::transfer_policy::ArgumentTransferPolicy<T1>,
) {
    let a0 = arg0.into_argument(b).await;
    let a1 = arg1.into_argument_mut(b).await;
    let a2 = arg2.into_argument(b).await;
    let a3 = arg3.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "kiosk_extension",
        "lock",
        vec![< T0 as MoveType > ::type_tag(b), < T1 as MoveType > ::type_tag(b)],
        vec![a0, a1, a2, a3],
    );
}
/// Returns: `bool`.
pub async fn is_installed<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl super::kiosk::ArgumentKiosk,
) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "kiosk_extension",
        "is_installed",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![a0],
    )
}
/// Returns: `bool`.
pub async fn is_enabled<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl super::kiosk::ArgumentKiosk,
) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "kiosk_extension",
        "is_enabled",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![a0],
    )
}
/// Returns: `bool`.
pub async fn can_place<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl super::kiosk::ArgumentKiosk,
) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "kiosk_extension",
        "can_place",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![a0],
    )
}
/// Returns: `bool`.
pub async fn can_lock<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl super::kiosk::ArgumentKiosk,
) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "kiosk_extension",
        "can_lock",
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
    pub fn extension_tag(&self) -> TypeTag {
        <Extension as MoveType>::type_tag_at(self.package)
    }
}
