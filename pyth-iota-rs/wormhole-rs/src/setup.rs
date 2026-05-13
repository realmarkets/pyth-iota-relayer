#![allow(unused_imports, non_snake_case, non_upper_case_globals)]
//! This module implements the mechanism to publish the Wormhole contract and
//! initialize `State` as a shared object.
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
/// Only the owner of the `DeployerCap` can call this method. This
/// method destroys the capability and shares the `State` object.
pub async fn complete(
    b: &mut PtbBuilder,
    arg0: impl ArgumentDeployerCap,
    arg1: impl ::iota_framework_rs::package::ArgumentUpgradeCap,
    arg2: impl PureU16,
    arg3: impl PureVec<u8>,
    arg4: impl PureU32,
    arg5: impl PureVec<Vec<u8>>,
    arg6: impl PureU32,
    arg7: impl PureU64,
) {
    let a0 = arg0.into_argument(b).await;
    let a1 = arg1.into_argument(b).await;
    let a2 = arg2.into_argument(b).await;
    let a3 = arg3.into_argument(b).await;
    let a4 = arg4.into_argument(b).await;
    let a5 = arg5.into_argument(b).await;
    let a6 = arg6.into_argument(b).await;
    let a7 = arg7.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "setup",
        "complete",
        Vec::new(),
        vec![a0, a1, a2, a3, a4, a5, a6, a7],
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
    pub fn deployercap_tag(&self) -> TypeTag {
        <DeployerCap as MoveType>::type_tag_at(self.package)
    }
}
