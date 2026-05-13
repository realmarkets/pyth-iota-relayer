#![allow(unused_imports, non_snake_case, non_upper_case_globals)]
//! Note: this module is adapted from Wormhole's version_control.move module.
//!
//! This module implements dynamic field keys as empty structs. These keys are
//! used to determine the latest version for this build. If the current version
//! is not this build's, then paths through the `state` module will abort.
//!
//! See `pyth::state` and `wormhole::package_utils` for more info.
use move_bindgen_runtime::*;
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;
/// RELEASE NOTES
///
/// - Gas optimizations on merkle tree verifications
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct V__0_1_2 {
    pub dummy_field: bool,
}
impl MoveType for V__0_1_2 {
    type Package = super::Package;
    const MODULE: &'static str = "version_control";
    const NAME: &'static str = "V__0_1_2";
}
impl MoveArg for V__0_1_2 {
    fn pure_bytes(self) -> PureBytes {
        PureBytes(::bcs::to_bytes(&self).expect("bcs serialization failed"))
    }
}
pub trait ArgumentV__0_1_2: PTBArgument {
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
impl ArgumentV__0_1_2 for V__0_1_2 {}
impl ArgumentV__0_1_2 for Argument {}
/// RELEASE NOTES
///
/// - Refactor state to use package management via
///   `wormhole::package_utils`.
/// - Add `MigrateComplete` event in `migrate`.
///
/// Also added `migrate__v__0_1_1` in `wormhole::state`, which is
/// meant to perform a one-time `State` modification via `migrate`.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct V__0_1_1 {
    pub dummy_field: bool,
}
impl MoveType for V__0_1_1 {
    type Package = super::Package;
    const MODULE: &'static str = "version_control";
    const NAME: &'static str = "V__0_1_1";
}
impl MoveArg for V__0_1_1 {
    fn pure_bytes(self) -> PureBytes {
        PureBytes(::bcs::to_bytes(&self).expect("bcs serialization failed"))
    }
}
pub trait ArgumentV__0_1_1: PTBArgument {
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
impl ArgumentV__0_1_1 for V__0_1_1 {}
impl ArgumentV__0_1_1 for Argument {}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct V__DUMMY {
    pub dummy_field: bool,
}
impl MoveType for V__DUMMY {
    type Package = super::Package;
    const MODULE: &'static str = "version_control";
    const NAME: &'static str = "V__DUMMY";
}
impl MoveArg for V__DUMMY {
    fn pure_bytes(self) -> PureBytes {
        PureBytes(::bcs::to_bytes(&self).expect("bcs serialization failed"))
    }
}
pub trait ArgumentV__DUMMY: PTBArgument {
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
impl ArgumentV__DUMMY for V__DUMMY {}
impl ArgumentV__DUMMY for Argument {}
/// Read-only handle bound to a runtime package address. Use
/// via `super::Package::at(addr).<module>()` and chain
/// `<type>_tag()` to build a [`TypeTag`] without spinning up
/// a `PtbBuilder`.
pub struct ModuleAt {
    pub(crate) package: Address,
}
impl ModuleAt {
    pub fn v__0_1_2_tag(&self) -> TypeTag {
        <V__0_1_2 as MoveType>::type_tag_at(self.package)
    }
    pub fn v__0_1_1_tag(&self) -> TypeTag {
        <V__0_1_1 as MoveType>::type_tag_at(self.package)
    }
    pub fn v__dummy_tag(&self) -> TypeTag {
        <V__DUMMY as MoveType>::type_tag_at(self.package)
    }
}
