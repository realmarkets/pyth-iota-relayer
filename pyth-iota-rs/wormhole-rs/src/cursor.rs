#![allow(unused_imports, non_snake_case, non_upper_case_globals)]
use move_bindgen_runtime::*;
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Cursor<T0: MoveType> {
    pub data: Vec<T0>,
}
impl<T0: MoveType> MoveType for Cursor<T0> {
    type Package = super::Package;
    const MODULE: &'static str = "cursor";
    const NAME: &'static str = "Cursor";
    fn type_params(addrs: &impl PackageAddrs) -> Vec<TypeTag> {
        vec![< T0 as MoveType > ::type_tag(addrs)]
    }
}
impl<T0: MoveType + ::serde::Serialize> MoveArg for Cursor<T0> {
    fn pure_bytes(self) -> PureBytes {
        PureBytes(::bcs::to_bytes(&self).expect("bcs serialization failed"))
    }
}
pub trait ArgumentCursor<T0: MoveType>: PTBArgument {
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
impl<T0: MoveType + ::serde::Serialize> ArgumentCursor<T0> for Cursor<T0> {}
impl<T0: MoveType> ArgumentCursor<T0> for Argument {}
/// Returns: `0xff00000000000002::cursor::Cursor<T0>`.
pub async fn new<T0: MoveType>(b: &mut PtbBuilder, arg0: impl PureVec<T0>) -> Argument {
    let a0 = arg0.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "cursor",
        "new",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![a0],
    )
}
/// Returns: `&vector<T0>`.
pub async fn data<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentCursor<T0>,
) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "cursor",
        "data",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![a0],
    )
}
/// Returns: `bool`.
pub async fn is_empty<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentCursor<T0>,
) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "cursor",
        "is_empty",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![a0],
    )
}
pub async fn destroy_empty<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentCursor<T0>,
) {
    let a0 = arg0.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "cursor",
        "destroy_empty",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![a0],
    );
}
/// Returns: `vector<T0>`.
pub async fn take_rest<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentCursor<T0>,
) -> Argument {
    let a0 = arg0.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "cursor",
        "take_rest",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![a0],
    )
}
/// Returns: `T0`.
pub async fn poke<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentCursor<T0>,
) -> Argument {
    let a0 = arg0.into_argument_mut(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "cursor",
        "poke",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![a0],
    )
}
