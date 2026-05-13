#![allow(unused_imports, non_snake_case, non_upper_case_globals)]
//! A system admin capability implementation.
use move_bindgen_runtime::*;
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;
/// The `new` function was called at a non-genesis epoch.
pub const ENotCalledAtGenesis: u64 = 0u64;
/// Sender is not @0x0 the system address.
pub const ENotSystemAddress: u64 = 1u64;
/// `IotaSystemAdminCap` allows to perform privileged IOTA system operations.
/// For example, packing and unpacking `TimeLock`s during staking, etc.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct IotaSystemAdminCap {
    pub dummy_field: bool,
}
impl MoveType for IotaSystemAdminCap {
    type Package = super::Package;
    const MODULE: &'static str = "system_admin_cap";
    const NAME: &'static str = "IotaSystemAdminCap";
}
impl MoveArg for IotaSystemAdminCap {
    fn pure_bytes(self) -> PureBytes {
        PureBytes(::bcs::to_bytes(&self).expect("bcs serialization failed"))
    }
}
pub trait ArgumentIotaSystemAdminCap: PTBArgument {
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
impl ArgumentIotaSystemAdminCap for IotaSystemAdminCap {}
impl ArgumentIotaSystemAdminCap for Argument {}
/// Read-only handle bound to a runtime package address. Use
/// via `super::Package::at(addr).<module>()` and chain
/// `<type>_tag()` to build a [`TypeTag`] without spinning up
/// a `PtbBuilder`.
pub struct ModuleAt {
    pub(crate) package: Address,
}
impl ModuleAt {
    pub fn iotasystemadmincap_tag(&self) -> TypeTag {
        <IotaSystemAdminCap as MoveType>::type_tag_at(self.package)
    }
}
