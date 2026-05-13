#![allow(unused_imports, non_snake_case, non_upper_case_globals)]
//! Functions for operating on Move packages from within Move:
//! - Creating proof-of-publish objects from one-time witnesses
//! - Administering package upgrades through upgrade policies.
use move_bindgen_runtime::*;
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;
/// Tried to create a `Publisher` using a type that isn't a
/// one-time witness.
pub const ENotOneTimeWitness: u64 = 0u64;
/// Tried to set a less restrictive policy than currently in place.
pub const ETooPermissive: u64 = 1u64;
/// This `UpgradeCap` has already authorized a pending upgrade.
pub const EAlreadyAuthorized: u64 = 2u64;
/// This `UpgradeCap` has not authorized an upgrade.
pub const ENotAuthorized: u64 = 3u64;
/// Trying to commit an upgrade to the wrong `UpgradeCap`.
pub const EWrongUpgradeCap: u64 = 4u64;
/// Update any part of the package (function implementations, add new
/// functions or types, change dependencies)
pub const COMPATIBLE: u8 = 0u8;
/// Add new functions or types, or change dependencies, existing
/// functions can't change.
pub const ADDITIVE: u8 = 128u8;
/// Only be able to change dependencies.
pub const DEP_ONLY: u8 = 192u8;
/// This type can only be created in the transaction that
/// generates a module, by consuming its one-time witness, so it
/// can be used to identify the address that published the package
/// a type originated from.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Publisher {
    pub id: UID,
    pub package: String,
    pub module_name: String,
}
impl MoveType for Publisher {
    type Package = super::Package;
    const MODULE: &'static str = "package";
    const NAME: &'static str = "Publisher";
}
pub trait ArgumentPublisher: PTBArgument {
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
impl ArgumentPublisher for Argument {}
impl ArgumentPublisher for ObjectId {
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
impl ArgumentPublisher for ObjectReference {}
impl ArgumentPublisher for Shared<ObjectId> {}
impl ArgumentPublisher for SharedMut<ObjectId> {}
impl ArgumentPublisher for Receiving<ObjectId> {}
/// Capability controlling the ability to upgrade a package.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UpgradeCap {
    pub id: UID,
    /// (Mutable) ID of the package that can be upgraded.
    pub package: ID,
    /// (Mutable) The number of upgrades that have been applied
    /// successively to the original package.  Initially 0.
    pub version: u64,
    /// What kind of upgrades are allowed.
    pub policy: u8,
}
impl MoveType for UpgradeCap {
    type Package = super::Package;
    const MODULE: &'static str = "package";
    const NAME: &'static str = "UpgradeCap";
}
pub trait ArgumentUpgradeCap: PTBArgument {
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
impl ArgumentUpgradeCap for Argument {}
impl ArgumentUpgradeCap for ObjectId {
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
impl ArgumentUpgradeCap for ObjectReference {}
impl ArgumentUpgradeCap for Shared<ObjectId> {}
impl ArgumentUpgradeCap for SharedMut<ObjectId> {}
impl ArgumentUpgradeCap for Receiving<ObjectId> {}
/// Permission to perform a particular upgrade (for a fixed version of
/// the package, bytecode to upgrade with and transitive dependencies to
/// depend against).
///
/// An `UpgradeCap` can only issue one ticket at a time, to prevent races
/// between concurrent updates or a change in its upgrade policy after
/// issuing a ticket, so the ticket is a "Hot Potato" to preserve forward
/// progress.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UpgradeTicket {
    /// (Immutable) ID of the `UpgradeCap` this originated from.
    pub cap: ID,
    /// (Immutable) ID of the package that can be upgraded.
    pub package: ID,
    /// (Immutable) The policy regarding what kind of upgrade this ticket
    /// permits.
    pub policy: u8,
    /// (Immutable) SHA256 digest of the bytecode and transitive
    /// dependencies that will be used in the upgrade.
    pub digest: Vec<u8>,
}
impl MoveType for UpgradeTicket {
    type Package = super::Package;
    const MODULE: &'static str = "package";
    const NAME: &'static str = "UpgradeTicket";
}
impl MoveArg for UpgradeTicket {
    fn pure_bytes(self) -> PureBytes {
        PureBytes(::bcs::to_bytes(&self).expect("bcs serialization failed"))
    }
}
pub trait ArgumentUpgradeTicket: PTBArgument {
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
impl ArgumentUpgradeTicket for UpgradeTicket {}
impl ArgumentUpgradeTicket for Argument {}
/// Issued as a result of a successful upgrade, containing the
/// information to be used to update the `UpgradeCap`.  This is a "Hot
/// Potato" to ensure that it is used to update its `UpgradeCap` before
/// the end of the transaction that performed the upgrade.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UpgradeReceipt {
    /// (Immutable) ID of the `UpgradeCap` this originated from.
    pub cap: ID,
    /// (Immutable) ID of the package after it was upgraded.
    pub package: ID,
}
impl MoveType for UpgradeReceipt {
    type Package = super::Package;
    const MODULE: &'static str = "package";
    const NAME: &'static str = "UpgradeReceipt";
}
impl MoveArg for UpgradeReceipt {
    fn pure_bytes(self) -> PureBytes {
        PureBytes(::bcs::to_bytes(&self).expect("bcs serialization failed"))
    }
}
pub trait ArgumentUpgradeReceipt: PTBArgument {
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
impl ArgumentUpgradeReceipt for UpgradeReceipt {}
impl ArgumentUpgradeReceipt for Argument {}
/// Claim a Publisher object.
/// Requires a One-Time-Witness to prove ownership. Due to this
/// constraint there can be only one Publisher object per module
/// but multiple per package (!).
///
/// Returns: `0x2::package::Publisher`.
pub async fn claim<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentObject<()>,
) -> Argument {
    let a0 = arg0.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "package",
        "claim",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![a0],
    )
}
/// Claim a Publisher object and send it to transaction sender.
/// Since this function can only be called in the module initializer,
/// the sender is the publisher.
pub async fn claim_and_keep<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentObject<()>,
) {
    let a0 = arg0.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "package",
        "claim_and_keep",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![a0],
    );
}
/// Destroy a Publisher object effectively removing all privileges
/// associated with it.
pub async fn burn_publisher(b: &mut PtbBuilder, arg0: impl ArgumentPublisher) {
    let a0 = arg0.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "package",
        "burn_publisher",
        Vec::new(),
        vec![a0],
    );
}
/// Check whether type belongs to the same package as the publisher object.
///
/// Returns: `bool`.
pub async fn from_package<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentPublisher,
) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "package",
        "from_package",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![a0],
    )
}
/// Check whether a type belongs to the same module as the publisher object.
///
/// Returns: `bool`.
pub async fn from_module<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentPublisher,
) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "package",
        "from_module",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![a0],
    )
}
/// Read the name of the module.
///
/// Returns: `&0x1::ascii::String`.
pub async fn published_module(
    b: &mut PtbBuilder,
    arg0: impl ArgumentPublisher,
) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "package",
        "published_module",
        Vec::new(),
        vec![a0],
    )
}
/// Read the package address string.
///
/// Returns: `&0x1::ascii::String`.
pub async fn published_package(
    b: &mut PtbBuilder,
    arg0: impl ArgumentPublisher,
) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "package",
        "published_package",
        Vec::new(),
        vec![a0],
    )
}
/// The ID of the package that this cap authorizes upgrades for.
/// Can be `0x0` if the cap cannot currently authorize an upgrade
/// because there is already a pending upgrade in the transaction.
/// Otherwise guaranteed to be the latest version of any given
/// package.
///
/// Returns: `0x2::object::ID`.
pub async fn upgrade_package(
    b: &mut PtbBuilder,
    arg0: impl ArgumentUpgradeCap,
) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "package",
        "upgrade_package",
        Vec::new(),
        vec![a0],
    )
}
/// The most recent version of the package, increments by one for each
/// successfully applied upgrade.
///
/// Returns: `u64`.
pub async fn version(b: &mut PtbBuilder, arg0: impl ArgumentUpgradeCap) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "package",
        "version",
        Vec::new(),
        vec![a0],
    )
}
/// The most permissive kind of upgrade currently supported by this
/// `cap`.
///
/// Returns: `u8`.
pub async fn upgrade_policy(
    b: &mut PtbBuilder,
    arg0: impl ArgumentUpgradeCap,
) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "package",
        "upgrade_policy",
        Vec::new(),
        vec![a0],
    )
}
/// The package that this ticket is authorized to upgrade
///
/// Returns: `0x2::object::ID`.
pub async fn ticket_package(
    b: &mut PtbBuilder,
    arg0: impl ArgumentUpgradeTicket,
) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "package",
        "ticket_package",
        Vec::new(),
        vec![a0],
    )
}
/// The kind of upgrade that this ticket authorizes.
///
/// Returns: `u8`.
pub async fn ticket_policy(
    b: &mut PtbBuilder,
    arg0: impl ArgumentUpgradeTicket,
) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "package",
        "ticket_policy",
        Vec::new(),
        vec![a0],
    )
}
/// ID of the `UpgradeCap` that this `receipt` should be used to
/// update.
///
/// Returns: `0x2::object::ID`.
pub async fn receipt_cap(
    b: &mut PtbBuilder,
    arg0: impl ArgumentUpgradeReceipt,
) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "package",
        "receipt_cap",
        Vec::new(),
        vec![a0],
    )
}
/// ID of the package that was upgraded to: the latest version of
/// the package, as of the upgrade represented by this `receipt`.
///
/// Returns: `0x2::object::ID`.
pub async fn receipt_package(
    b: &mut PtbBuilder,
    arg0: impl ArgumentUpgradeReceipt,
) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "package",
        "receipt_package",
        Vec::new(),
        vec![a0],
    )
}
/// A hash of the package contents for the new version of the
/// package.  This ticket only authorizes an upgrade to a package
/// that matches this digest.  A package's contents are identified
/// by two things:
///
///  - modules: [[u8]]       a list of the package's module contents
///  - deps:    [[u8; 32]]   a list of 32 byte ObjectIDs of the
///                          package's transitive dependencies
///
/// A package's digest is calculated as:
///
///   sha3_256(sort(modules ++ deps))
///
/// Returns: `&vector<u8>`.
pub async fn ticket_digest(
    b: &mut PtbBuilder,
    arg0: impl ArgumentUpgradeTicket,
) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "package",
        "ticket_digest",
        Vec::new(),
        vec![a0],
    )
}
/// Expose the constants representing various upgrade policies
///
/// Returns: `u8`.
pub async fn compatible_policy(b: &mut PtbBuilder) -> Argument {
    b.move_call(
        b.package_id::<super::Package>(),
        "package",
        "compatible_policy",
        Vec::new(),
        vec![],
    )
}
/// Returns: `u8`.
pub async fn additive_policy(b: &mut PtbBuilder) -> Argument {
    b.move_call(
        b.package_id::<super::Package>(),
        "package",
        "additive_policy",
        Vec::new(),
        vec![],
    )
}
/// Returns: `u8`.
pub async fn dep_only_policy(b: &mut PtbBuilder) -> Argument {
    b.move_call(
        b.package_id::<super::Package>(),
        "package",
        "dep_only_policy",
        Vec::new(),
        vec![],
    )
}
pub async fn only_additive_upgrades(b: &mut PtbBuilder, arg0: impl ArgumentUpgradeCap) {
    let a0 = arg0.into_argument_mut(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "package",
        "only_additive_upgrades",
        Vec::new(),
        vec![a0],
    );
}
/// Restrict upgrades through this upgrade `cap` to just change
/// dependencies.
pub async fn only_dep_upgrades(b: &mut PtbBuilder, arg0: impl ArgumentUpgradeCap) {
    let a0 = arg0.into_argument_mut(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "package",
        "only_dep_upgrades",
        Vec::new(),
        vec![a0],
    );
}
/// Discard the `UpgradeCap` to make a package immutable.
pub async fn make_immutable(b: &mut PtbBuilder, arg0: impl ArgumentUpgradeCap) {
    let a0 = arg0.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "package",
        "make_immutable",
        Vec::new(),
        vec![a0],
    );
}
/// Issue a ticket authorizing an upgrade to a particular new bytecode
/// (identified by its digest).  A ticket will only be issued if one has
/// not already been issued, and if the `policy` requested is at least as
/// restrictive as the policy set out by the `cap`.
///
/// The `digest` supplied and the `policy` will both be checked by
/// validators when running the upgrade.  I.e. the bytecode supplied in
/// the upgrade must have a matching digest, and the changes relative to
/// the parent package must be compatible with the policy in the ticket
/// for the upgrade to succeed.
///
/// Returns: `0x2::package::UpgradeTicket`.
pub async fn authorize_upgrade(
    b: &mut PtbBuilder,
    arg0: impl ArgumentUpgradeCap,
    arg1: impl PureU8,
    arg2: impl PureVec<u8>,
) -> Argument {
    let a0 = arg0.into_argument_mut(b).await;
    let a1 = arg1.into_argument(b).await;
    let a2 = arg2.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "package",
        "authorize_upgrade",
        Vec::new(),
        vec![a0, a1, a2],
    )
}
/// Consume an `UpgradeReceipt` to update its `UpgradeCap`, finalizing
/// the upgrade.
pub async fn commit_upgrade(
    b: &mut PtbBuilder,
    arg0: impl ArgumentUpgradeCap,
    arg1: impl ArgumentUpgradeReceipt,
) {
    let a0 = arg0.into_argument_mut(b).await;
    let a1 = arg1.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "package",
        "commit_upgrade",
        Vec::new(),
        vec![a0, a1],
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
    pub fn publisher_tag(&self) -> TypeTag {
        <Publisher as MoveType>::type_tag_at(self.package)
    }
    pub fn upgradecap_tag(&self) -> TypeTag {
        <UpgradeCap as MoveType>::type_tag_at(self.package)
    }
    pub fn upgradeticket_tag(&self) -> TypeTag {
        <UpgradeTicket as MoveType>::type_tag_at(self.package)
    }
    pub fn upgradereceipt_tag(&self) -> TypeTag {
        <UpgradeReceipt as MoveType>::type_tag_at(self.package)
    }
}
