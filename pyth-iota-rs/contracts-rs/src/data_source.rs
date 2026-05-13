#![allow(unused_imports, non_snake_case, non_upper_case_globals)]
use move_bindgen_runtime::*;
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;
pub const KEY: &[u8] = &[
    100u8, 97u8, 116u8, 97u8, 95u8, 115u8, 111u8, 117u8, 114u8, 99u8, 101u8, 115u8,
];
pub const E_DATA_SOURCE_REGISTRY_ALREADY_EXISTS: u64 = 0u64;
pub const E_DATA_SOURCE_ALREADY_REGISTERED: u64 = 1u64;
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DataSource {
    pub emitter_chain: u64,
    pub emitter_address: ::wormhole_rs::external_address::ExternalAddress,
}
impl MoveType for DataSource {
    type Package = super::Package;
    const MODULE: &'static str = "data_source";
    const NAME: &'static str = "DataSource";
}
impl MoveArg for DataSource {
    fn pure_bytes(self) -> PureBytes {
        PureBytes(::bcs::to_bytes(&self).expect("bcs serialization failed"))
    }
}
pub trait ArgumentDataSource: PTBArgument {
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
impl ArgumentDataSource for DataSource {}
impl ArgumentDataSource for Argument {}
/// Returns: `bool`.
pub async fn contains(
    b: &mut PtbBuilder,
    arg0: impl ArgumentObject<()>,
    arg1: impl ArgumentDataSource,
) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    let a1 = arg1.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "data_source",
        "contains",
        Vec::new(),
        vec![a0, a1],
    )
}
/// Returns: `u64`.
pub async fn emitter_chain(
    b: &mut PtbBuilder,
    arg0: impl ArgumentDataSource,
) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "data_source",
        "emitter_chain",
        Vec::new(),
        vec![a0],
    )
}
/// Returns: `0xff00000000000002::external_address::ExternalAddress`.
pub async fn emitter_address(
    b: &mut PtbBuilder,
    arg0: impl ArgumentDataSource,
) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "data_source",
        "emitter_address",
        Vec::new(),
        vec![a0],
    )
}
/// Read-only handle bound to a runtime package address. Use
/// via `super::Package::at(addr).<module>()` and chain
/// `<type>_tag()` to build a [`TypeTag`] without spinning up
/// a `PtbBuilder`.
pub struct ModuleAt {
    pub(crate) package: Address,
}
impl ModuleAt {
    pub fn datasource_tag(&self) -> TypeTag {
        <DataSource as MoveType>::type_tag_at(self.package)
    }
}
