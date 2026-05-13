#![allow(unused_imports, non_snake_case, non_upper_case_globals)]
//! Defines the `DenyList` type. The `DenyList` shared object is used to restrict access to
//! instances of certain core types from being used as inputs by specified addresses in the deny
//! list.
use move_bindgen_runtime::*;
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;
/// Trying to create a deny list object when not called by the system address.
pub const ENotSystemAddress: u64 = 0u64;
/// A shared object that stores the addresses that are blocked for a given core type.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DenyList {
    pub id: UID,
    /// The individual deny lists.
    pub lists: super::bag::Bag,
}
impl MoveType for DenyList {
    type Package = super::Package;
    const MODULE: &'static str = "deny_list";
    const NAME: &'static str = "DenyList";
}
pub trait ArgumentDenyList: PTBArgument {
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
impl ArgumentDenyList for Argument {}
impl ArgumentDenyList for ObjectId {
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
impl ArgumentDenyList for ObjectReference {}
impl ArgumentDenyList for Shared<ObjectId> {}
impl ArgumentDenyList for SharedMut<ObjectId> {}
impl ArgumentDenyList for Receiving<ObjectId> {}
/// The capability used to write to the deny list config. Ensures that the Configs for the
/// DenyList are modified only by this module.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ConfigWriteCap {
    pub dummy_field: bool,
}
impl MoveType for ConfigWriteCap {
    type Package = super::Package;
    const MODULE: &'static str = "deny_list";
    const NAME: &'static str = "ConfigWriteCap";
}
impl MoveArg for ConfigWriteCap {
    fn pure_bytes(self) -> PureBytes {
        PureBytes(::bcs::to_bytes(&self).expect("bcs serialization failed"))
    }
}
pub trait ArgumentConfigWriteCap: PTBArgument {
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
impl ArgumentConfigWriteCap for ConfigWriteCap {}
impl ArgumentConfigWriteCap for Argument {}
/// The dynamic object field key used to store the `Config` for a given type, essentially a
/// `(per_type_index, per_type_key)` pair.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ConfigKey {
    pub per_type_index: u64,
    pub per_type_key: Vec<u8>,
}
impl MoveType for ConfigKey {
    type Package = super::Package;
    const MODULE: &'static str = "deny_list";
    const NAME: &'static str = "ConfigKey";
}
impl MoveArg for ConfigKey {
    fn pure_bytes(self) -> PureBytes {
        PureBytes(::bcs::to_bytes(&self).expect("bcs serialization failed"))
    }
}
pub trait ArgumentConfigKey: PTBArgument {
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
impl ArgumentConfigKey for ConfigKey {}
impl ArgumentConfigKey for Argument {}
/// The setting key used to store the deny list for a given address in the `Config`.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AddressKey {
    pub pos0: Address,
}
impl MoveType for AddressKey {
    type Package = super::Package;
    const MODULE: &'static str = "deny_list";
    const NAME: &'static str = "AddressKey";
}
impl MoveArg for AddressKey {
    fn pure_bytes(self) -> PureBytes {
        PureBytes(::bcs::to_bytes(&self).expect("bcs serialization failed"))
    }
}
pub trait ArgumentAddressKey: PTBArgument {
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
impl ArgumentAddressKey for AddressKey {}
impl ArgumentAddressKey for Argument {}
/// The setting key used to store the global pause setting in the `Config`.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GlobalPauseKey {
    pub dummy_field: bool,
}
impl MoveType for GlobalPauseKey {
    type Package = super::Package;
    const MODULE: &'static str = "deny_list";
    const NAME: &'static str = "GlobalPauseKey";
}
impl MoveArg for GlobalPauseKey {
    fn pure_bytes(self) -> PureBytes {
        PureBytes(::bcs::to_bytes(&self).expect("bcs serialization failed"))
    }
}
pub trait ArgumentGlobalPauseKey: PTBArgument {
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
impl ArgumentGlobalPauseKey for GlobalPauseKey {}
impl ArgumentGlobalPauseKey for Argument {}
/// The event emitted when a new `Config` is created for a given type. This can be useful for
/// tracking the `ID` of a type's `Config` object.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PerTypeConfigCreated {
    pub key: ConfigKey,
    pub config_id: ID,
}
impl MoveType for PerTypeConfigCreated {
    type Package = super::Package;
    const MODULE: &'static str = "deny_list";
    const NAME: &'static str = "PerTypeConfigCreated";
}
impl MoveArg for PerTypeConfigCreated {
    fn pure_bytes(self) -> PureBytes {
        PureBytes(::bcs::to_bytes(&self).expect("bcs serialization failed"))
    }
}
pub trait ArgumentPerTypeConfigCreated: PTBArgument {
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
impl ArgumentPerTypeConfigCreated for PerTypeConfigCreated {}
impl ArgumentPerTypeConfigCreated for Argument {}
/// Read-only handle bound to a runtime package address. Use
/// via `super::Package::at(addr).<module>()` and chain
/// `<type>_tag()` to build a [`TypeTag`] without spinning up
/// a `PtbBuilder`.
pub struct ModuleAt {
    pub(crate) package: Address,
}
impl ModuleAt {
    pub fn denylist_tag(&self) -> TypeTag {
        <DenyList as MoveType>::type_tag_at(self.package)
    }
    pub fn configwritecap_tag(&self) -> TypeTag {
        <ConfigWriteCap as MoveType>::type_tag_at(self.package)
    }
    pub fn configkey_tag(&self) -> TypeTag {
        <ConfigKey as MoveType>::type_tag_at(self.package)
    }
    pub fn addresskey_tag(&self) -> TypeTag {
        <AddressKey as MoveType>::type_tag_at(self.package)
    }
    pub fn globalpausekey_tag(&self) -> TypeTag {
        <GlobalPauseKey as MoveType>::type_tag_at(self.package)
    }
    pub fn pertypeconfigcreated_tag(&self) -> TypeTag {
        <PerTypeConfigCreated as MoveType>::type_tag_at(self.package)
    }
}
