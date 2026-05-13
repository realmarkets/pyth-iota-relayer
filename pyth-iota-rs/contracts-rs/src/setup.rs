#![allow(unused_imports, non_snake_case, non_upper_case_globals)]
use move_bindgen_runtime::*;
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;
/// Capability created at `init`, which will be destroyed once
/// `init_and_share_state` is called. This ensures only the deployer can
/// create the shared `State`.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DeployerCap {
    pub id: UID,
}
impl MoveType for DeployerCap {
    type Package = super::Package;
    const MODULE: &'static str = "setup";
    const NAME: &'static str = "DeployerCap";
}
pub trait ArgumentDeployerCap: PTBArgument {
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
impl ArgumentDeployerCap for Argument {}
impl ArgumentDeployerCap for ObjectId {
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
impl ArgumentDeployerCap for ObjectReference {}
impl ArgumentDeployerCap for Shared<ObjectId> {}
impl ArgumentDeployerCap for SharedMut<ObjectId> {}
impl ArgumentDeployerCap for Receiving<ObjectId> {}
/// Read-only handle bound to a runtime package address. Use
/// via `super::Package::at(addr).<module>()` and chain
/// `<type>_tag()` to build a [`TypeTag`] without spinning up
/// a `PtbBuilder`.
pub struct ModuleAt {
    pub(crate) package: Address,
}
impl ModuleAt {
    pub fn deployercap_tag(&self) -> TypeTag {
        <DeployerCap as MoveType>::type_tag_at(self.package)
    }
}
