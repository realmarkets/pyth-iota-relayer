#![allow(unused_imports, non_snake_case, non_upper_case_globals)]
use move_bindgen_runtime::*;
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;
pub const ENotSupported: u64 = 0u64;
pub const EInvalidInput: u64 = 1u64;
pub const EInputTooLong: u64 = 2u64;
pub const EInvalidBufferLength: u64 = 3u64;
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Element<T0: MoveType> {
    pub bytes: Vec<u8>,
    #[serde(skip)]
    pub _phantom_t0: PhantomData<T0>,
}
impl<T0: MoveType> MoveType for Element<T0> {
    type Package = super::Package;
    const MODULE: &'static str = "group_ops";
    const NAME: &'static str = "Element";
    fn type_params(addrs: &impl PackageAddrs) -> Vec<TypeTag> {
        vec![< T0 as MoveType > ::type_tag(addrs)]
    }
}
impl<T0: MoveType + ::serde::Serialize> MoveArg for Element<T0> {
    fn pure_bytes(self) -> PureBytes {
        PureBytes(::bcs::to_bytes(&self).expect("bcs serialization failed"))
    }
}
pub trait ArgumentElement<T0: MoveType>: PTBArgument {
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
impl<T0: MoveType + ::serde::Serialize> ArgumentElement<T0> for Element<T0> {}
impl<T0: MoveType> ArgumentElement<T0> for Argument {}
/// Returns: `&vector<u8>`.
pub async fn bytes<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentElement<T0>,
) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "group_ops",
        "bytes",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![a0],
    )
}
/// Returns: `bool`.
pub async fn equal<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentElement<T0>,
    arg1: impl ArgumentElement<T0>,
) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    let a1 = arg1.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "group_ops",
        "equal",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![a0, a1],
    )
}
