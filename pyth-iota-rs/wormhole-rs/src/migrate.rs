#![allow(unused_imports, non_snake_case, non_upper_case_globals)]
//! This module implements a public method intended to be called after an
//! upgrade has been committed. The purpose is to add one-off migration logic
//! that would alter Wormhole `State`.
//!
//! Included in migration is the ability to ensure that breaking changes for
//! any of Wormhole's methods by enforcing the current build version as their
//! required minimum version.
use move_bindgen_runtime::*;
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;
/// Event reflecting when `migrate` is successfully executed.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MigrateComplete {
    pub package: ID,
}
impl MoveType for MigrateComplete {
    type Package = super::Package;
    const MODULE: &'static str = "migrate";
    const NAME: &'static str = "MigrateComplete";
}
impl MoveArg for MigrateComplete {
    fn pure_bytes(self) -> PureBytes {
        PureBytes(::bcs::to_bytes(&self).expect("bcs serialization failed"))
    }
}
pub trait ArgumentMigrateComplete: PTBArgument {
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
impl ArgumentMigrateComplete for MigrateComplete {}
impl ArgumentMigrateComplete for Argument {}
/// Execute migration logic. See `wormhole::migrate` description for more
/// info.
pub async fn migrate(
    b: &mut PtbBuilder,
    arg0: impl super::state::ArgumentState,
    arg1: impl PureVec<u8>,
    arg2: impl ::iota_framework_rs::clock::ArgumentClock,
) {
    let a0 = arg0.into_argument_mut(b).await;
    let a1 = arg1.into_argument(b).await;
    let a2 = arg2.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "migrate",
        "migrate",
        Vec::new(),
        vec![a0, a1, a2],
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
    pub fn migratecomplete_tag(&self) -> TypeTag {
        <MigrateComplete as MoveType>::type_tag_at(self.package)
    }
}
