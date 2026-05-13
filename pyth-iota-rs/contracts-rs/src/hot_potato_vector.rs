#![allow(unused_imports, non_snake_case, non_upper_case_globals)]
//! This class represents a vector of objects wrapped
//! inside of a hot potato struct.
use move_bindgen_runtime::*;
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct HotPotatoVector<T0: MoveType> {
    pub contents: Vec<T0>,
}
impl<T0: MoveType> MoveType for HotPotatoVector<T0> {
    type Package = super::Package;
    const MODULE: &'static str = "hot_potato_vector";
    const NAME: &'static str = "HotPotatoVector";
    fn type_params(addrs: &impl PackageAddrs) -> Vec<TypeTag> {
        vec![< T0 as MoveType > ::type_tag(addrs)]
    }
}
impl<T0: MoveType + ::serde::Serialize> MoveArg for HotPotatoVector<T0> {
    fn pure_bytes(self) -> PureBytes {
        PureBytes(::bcs::to_bytes(&self).expect("bcs serialization failed"))
    }
}
pub trait ArgumentHotPotatoVector<T0: MoveType>: PTBArgument {
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
impl<T0: MoveType + ::serde::Serialize> ArgumentHotPotatoVector<T0>
for HotPotatoVector<T0> {}
impl<T0: MoveType> ArgumentHotPotatoVector<T0> for Argument {}
pub async fn destroy<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentHotPotatoVector<T0>,
) {
    let a0 = arg0.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "hot_potato_vector",
        "destroy",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![a0],
    );
}
/// Returns: `u64`.
pub async fn length<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentHotPotatoVector<T0>,
) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "hot_potato_vector",
        "length",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![a0],
    )
}
/// Returns: `bool`.
pub async fn is_empty<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentHotPotatoVector<T0>,
) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "hot_potato_vector",
        "is_empty",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![a0],
    )
}
