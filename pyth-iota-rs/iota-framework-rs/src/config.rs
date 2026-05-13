#![allow(unused_imports, non_snake_case, non_upper_case_globals)]
use move_bindgen_runtime::*;
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;
pub const EAlreadySetForEpoch: u64 = 0u64;
pub const ENotSetForEpoch: u64 = 1u64;
pub const EBCSSerializationFailure: u64 = 2u64;
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Config<T0: MoveType> {
    pub id: UID,
    #[serde(skip)]
    pub _phantom_t0: PhantomData<T0>,
}
impl<T0: MoveType> MoveType for Config<T0> {
    type Package = super::Package;
    const MODULE: &'static str = "config";
    const NAME: &'static str = "Config";
    fn type_params(addrs: &impl PackageAddrs) -> Vec<TypeTag> {
        vec![< T0 as MoveType > ::type_tag(addrs)]
    }
}
pub trait ArgumentConfig<T0: MoveType>: PTBArgument {
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
impl<T0: MoveType> ArgumentConfig<T0> for Argument {}
impl<T0: MoveType> ArgumentConfig<T0> for ObjectId {
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
impl<T0: MoveType> ArgumentConfig<T0> for ObjectReference {}
impl<T0: MoveType> ArgumentConfig<T0> for Shared<ObjectId> {}
impl<T0: MoveType> ArgumentConfig<T0> for SharedMut<ObjectId> {}
impl<T0: MoveType> ArgumentConfig<T0> for Receiving<ObjectId> {}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Setting<T0: MoveType> {
    pub data: Option<SettingData<T0>>,
}
impl<T0: MoveType> MoveType for Setting<T0> {
    type Package = super::Package;
    const MODULE: &'static str = "config";
    const NAME: &'static str = "Setting";
    fn type_params(addrs: &impl PackageAddrs) -> Vec<TypeTag> {
        vec![< T0 as MoveType > ::type_tag(addrs)]
    }
}
impl<T0: MoveType + ::serde::Serialize> MoveArg for Setting<T0> {
    fn pure_bytes(self) -> PureBytes {
        PureBytes(::bcs::to_bytes(&self).expect("bcs serialization failed"))
    }
}
pub trait ArgumentSetting<T0: MoveType>: PTBArgument {
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
impl<T0: MoveType + ::serde::Serialize> ArgumentSetting<T0> for Setting<T0> {}
impl<T0: MoveType> ArgumentSetting<T0> for Argument {}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SettingData<T0: MoveType> {
    pub newer_value_epoch: u64,
    pub newer_value: Option<T0>,
    pub older_value_opt: Option<T0>,
}
impl<T0: MoveType> MoveType for SettingData<T0> {
    type Package = super::Package;
    const MODULE: &'static str = "config";
    const NAME: &'static str = "SettingData";
    fn type_params(addrs: &impl PackageAddrs) -> Vec<TypeTag> {
        vec![< T0 as MoveType > ::type_tag(addrs)]
    }
}
impl<T0: MoveType + ::serde::Serialize> MoveArg for SettingData<T0> {
    fn pure_bytes(self) -> PureBytes {
        PureBytes(::bcs::to_bytes(&self).expect("bcs serialization failed"))
    }
}
pub trait ArgumentSettingData<T0: MoveType>: PTBArgument {
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
impl<T0: MoveType + ::serde::Serialize> ArgumentSettingData<T0> for SettingData<T0> {}
impl<T0: MoveType> ArgumentSettingData<T0> for Argument {}
