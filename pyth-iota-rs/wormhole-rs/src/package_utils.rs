#![allow(unused_imports, non_snake_case, non_upper_case_globals)]
use move_bindgen_runtime::*;
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;
pub const E_INVALID_UPGRADE_CAP: u64 = 0u64;
pub const E_NOT_CURRENT_VERSION: u64 = 1u64;
pub const E_INCORRECT_OLD_VERSION: u64 = 2u64;
pub const E_SAME_VERSION: u64 = 3u64;
pub const E_TYPE_NOT_ALLOWED: u64 = 4u64;
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CurrentVersion {
    pub dummy_field: bool,
}
impl MoveType for CurrentVersion {
    type Package = super::Package;
    const MODULE: &'static str = "package_utils";
    const NAME: &'static str = "CurrentVersion";
}
impl MoveArg for CurrentVersion {
    fn pure_bytes(self) -> PureBytes {
        PureBytes(::bcs::to_bytes(&self).expect("bcs serialization failed"))
    }
}
pub trait ArgumentCurrentVersion: PTBArgument {
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
impl ArgumentCurrentVersion for CurrentVersion {}
impl ArgumentCurrentVersion for Argument {}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CurrentPackage {
    pub dummy_field: bool,
}
impl MoveType for CurrentPackage {
    type Package = super::Package;
    const MODULE: &'static str = "package_utils";
    const NAME: &'static str = "CurrentPackage";
}
impl MoveArg for CurrentPackage {
    fn pure_bytes(self) -> PureBytes {
        PureBytes(::bcs::to_bytes(&self).expect("bcs serialization failed"))
    }
}
pub trait ArgumentCurrentPackage: PTBArgument {
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
impl ArgumentCurrentPackage for CurrentPackage {}
impl ArgumentCurrentPackage for Argument {}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PendingPackage {
    pub dummy_field: bool,
}
impl MoveType for PendingPackage {
    type Package = super::Package;
    const MODULE: &'static str = "package_utils";
    const NAME: &'static str = "PendingPackage";
}
impl MoveArg for PendingPackage {
    fn pure_bytes(self) -> PureBytes {
        PureBytes(::bcs::to_bytes(&self).expect("bcs serialization failed"))
    }
}
pub trait ArgumentPendingPackage: PTBArgument {
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
impl ArgumentPendingPackage for PendingPackage {}
impl ArgumentPendingPackage for Argument {}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PackageInfo {
    pub package: ID,
    pub digest: super::bytes32::Bytes32,
}
impl MoveType for PackageInfo {
    type Package = super::Package;
    const MODULE: &'static str = "package_utils";
    const NAME: &'static str = "PackageInfo";
}
impl MoveArg for PackageInfo {
    fn pure_bytes(self) -> PureBytes {
        PureBytes(::bcs::to_bytes(&self).expect("bcs serialization failed"))
    }
}
pub trait ArgumentPackageInfo: PTBArgument {
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
impl ArgumentPackageInfo for PackageInfo {}
impl ArgumentPackageInfo for Argument {}
/// Returns: `0x2::object::ID`.
pub async fn current_package(
    b: &mut PtbBuilder,
    arg0: impl ArgumentObject<()>,
) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "package_utils",
        "current_package",
        Vec::new(),
        vec![a0],
    )
}
/// Returns: `0xff00000000000002::bytes32::Bytes32`.
pub async fn current_digest(
    b: &mut PtbBuilder,
    arg0: impl ArgumentObject<()>,
) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "package_utils",
        "current_digest",
        Vec::new(),
        vec![a0],
    )
}
/// Returns: `0x2::object::ID`.
pub async fn committed_package(
    b: &mut PtbBuilder,
    arg0: impl ArgumentObject<()>,
) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "package_utils",
        "committed_package",
        Vec::new(),
        vec![a0],
    )
}
/// Returns: `0xff00000000000002::bytes32::Bytes32`.
pub async fn authorized_digest(
    b: &mut PtbBuilder,
    arg0: impl ArgumentObject<()>,
) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "package_utils",
        "authorized_digest",
        Vec::new(),
        vec![a0],
    )
}
pub async fn assert_package_upgrade_cap<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ::iota_framework_rs::package::ArgumentUpgradeCap,
    arg1: impl PureU8,
    arg2: impl PureU64,
) {
    let a0 = arg0.into_argument_ref(b).await;
    let a1 = arg1.into_argument(b).await;
    let a2 = arg2.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "package_utils",
        "assert_package_upgrade_cap",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![a0, a1, a2],
    );
}
pub async fn assert_version<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentObject<()>,
    arg1: impl ArgumentObject<()>,
) {
    let a0 = arg0.into_argument_ref(b).await;
    let a1 = arg1.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "package_utils",
        "assert_version",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![a0, a1],
    );
}
/// Returns: `0x1::type_name::TypeName`.
pub async fn type_of_version<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentObject<()>,
) -> Argument {
    let a0 = arg0.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "package_utils",
        "type_of_version",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![a0],
    )
}
pub async fn init_package_info<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentObject<()>,
    arg1: impl ArgumentObject<()>,
    arg2: impl ::iota_framework_rs::package::ArgumentUpgradeCap,
) {
    let a0 = arg0.into_argument_mut(b).await;
    let a1 = arg1.into_argument(b).await;
    let a2 = arg2.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "package_utils",
        "init_package_info",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![a0, a1, a2],
    );
}
pub async fn migrate_version<T0: MoveType, T1: MoveType>(
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
        "package_utils",
        "migrate_version",
        vec![< T0 as MoveType > ::type_tag(b), < T1 as MoveType > ::type_tag(b)],
        vec![a0, a1, a2],
    );
}
/// Returns: `0x2::package::UpgradeTicket`.
pub async fn authorize_upgrade(
    b: &mut PtbBuilder,
    arg0: impl ArgumentObject<()>,
    arg1: impl ::iota_framework_rs::package::ArgumentUpgradeCap,
    arg2: impl super::bytes32::ArgumentBytes32,
) -> Argument {
    let a0 = arg0.into_argument_mut(b).await;
    let a1 = arg1.into_argument_mut(b).await;
    let a2 = arg2.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "package_utils",
        "authorize_upgrade",
        Vec::new(),
        vec![a0, a1, a2],
    )
}
/// Returns: `(0x2::object::ID, 0x2::object::ID)`.
pub async fn commit_upgrade(
    b: &mut PtbBuilder,
    arg0: impl ArgumentObject<()>,
    arg1: impl ::iota_framework_rs::package::ArgumentUpgradeCap,
    arg2: impl ::iota_framework_rs::package::ArgumentUpgradeReceipt,
) -> (Argument, Argument) {
    let a0 = arg0.into_argument_mut(b).await;
    let a1 = arg1.into_argument_mut(b).await;
    let a2 = arg2.into_argument(b).await;
    let __r = b
        .move_call_n(
            b.package_id::<super::Package>(),
            "package_utils",
            "commit_upgrade",
            Vec::new(),
            vec![a0, a1, a2],
            2u16,
        );
    (__r[0], __r[1])
}
/// Read-only handle bound to a runtime package address. Use
/// via `super::Package::at(addr).<module>()` and chain
/// `<type>_tag()` to build a [`TypeTag`] without spinning up
/// a `PtbBuilder`.
pub struct ModuleAt {
    pub(crate) package: Address,
}
impl ModuleAt {
    pub fn currentversion_tag(&self) -> TypeTag {
        <CurrentVersion as MoveType>::type_tag_at(self.package)
    }
    pub fn currentpackage_tag(&self) -> TypeTag {
        <CurrentPackage as MoveType>::type_tag_at(self.package)
    }
    pub fn pendingpackage_tag(&self) -> TypeTag {
        <PendingPackage as MoveType>::type_tag_at(self.package)
    }
    pub fn packageinfo_tag(&self) -> TypeTag {
        <PackageInfo as MoveType>::type_tag_at(self.package)
    }
}
