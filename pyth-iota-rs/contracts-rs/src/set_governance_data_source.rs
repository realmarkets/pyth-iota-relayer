#![allow(unused_imports, non_snake_case, non_upper_case_globals)]
use move_bindgen_runtime::*;
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GovernanceDataSource {
    pub emitter_chain_id: u64,
    pub emitter_address: ::wormhole_rs::external_address::ExternalAddress,
    pub initial_sequence: u64,
}
impl MoveType for GovernanceDataSource {
    type Package = super::Package;
    const MODULE: &'static str = "set_governance_data_source";
    const NAME: &'static str = "GovernanceDataSource";
}
impl MoveArg for GovernanceDataSource {
    fn pure_bytes(self) -> PureBytes {
        PureBytes(::bcs::to_bytes(&self).expect("bcs serialization failed"))
    }
}
pub trait ArgumentGovernanceDataSource: PTBArgument {
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
impl ArgumentGovernanceDataSource for GovernanceDataSource {}
impl ArgumentGovernanceDataSource for Argument {}
/// Read-only handle bound to a runtime package address. Use
/// via `super::Package::at(addr).<module>()` and chain
/// `<type>_tag()` to build a [`TypeTag`] without spinning up
/// a `PtbBuilder`.
pub struct ModuleAt {
    pub(crate) package: Address,
}
impl ModuleAt {
    pub fn governancedatasource_tag(&self) -> TypeTag {
        <GovernanceDataSource as MoveType>::type_tag_at(self.package)
    }
}
