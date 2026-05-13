#![allow(unused_imports, non_snake_case, non_upper_case_globals)]
use move_bindgen_runtime::*;
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;
pub const E_ZERO_ADDRESS: u64 = 0u64;
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ExternalAddress {
    pub value: super::bytes32::Bytes32,
}
impl MoveType for ExternalAddress {
    type Package = super::Package;
    const MODULE: &'static str = "external_address";
    const NAME: &'static str = "ExternalAddress";
}
impl MoveArg for ExternalAddress {
    fn pure_bytes(self) -> PureBytes {
        PureBytes(::bcs::to_bytes(&self).expect("bcs serialization failed"))
    }
}
pub trait ArgumentExternalAddress: PTBArgument {
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
impl ArgumentExternalAddress for ExternalAddress {}
impl ArgumentExternalAddress for Argument {}
/// Returns: `0xff00000000000002::external_address::ExternalAddress`.
pub async fn new(
    b: &mut PtbBuilder,
    arg0: impl super::bytes32::ArgumentBytes32,
) -> Argument {
    let a0 = arg0.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "external_address",
        "new",
        Vec::new(),
        vec![a0],
    )
}
/// Returns: `0xff00000000000002::external_address::ExternalAddress`.
pub async fn default(b: &mut PtbBuilder) -> Argument {
    b.move_call(
        b.package_id::<super::Package>(),
        "external_address",
        "default",
        Vec::new(),
        vec![],
    )
}
/// Returns: `0xff00000000000002::external_address::ExternalAddress`.
pub async fn new_nonzero(
    b: &mut PtbBuilder,
    arg0: impl super::bytes32::ArgumentBytes32,
) -> Argument {
    let a0 = arg0.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "external_address",
        "new_nonzero",
        Vec::new(),
        vec![a0],
    )
}
/// Returns: `vector<u8>`.
pub async fn to_bytes(
    b: &mut PtbBuilder,
    arg0: impl ArgumentExternalAddress,
) -> Argument {
    let a0 = arg0.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "external_address",
        "to_bytes",
        Vec::new(),
        vec![a0],
    )
}
/// Returns: `0xff00000000000002::bytes32::Bytes32`.
pub async fn to_bytes32(
    b: &mut PtbBuilder,
    arg0: impl ArgumentExternalAddress,
) -> Argument {
    let a0 = arg0.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "external_address",
        "to_bytes32",
        Vec::new(),
        vec![a0],
    )
}
/// Returns: `0xff00000000000002::external_address::ExternalAddress`.
pub async fn take_bytes(
    b: &mut PtbBuilder,
    arg0: impl super::cursor::ArgumentCursor<u8>,
) -> Argument {
    let a0 = arg0.into_argument_mut(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "external_address",
        "take_bytes",
        Vec::new(),
        vec![a0],
    )
}
/// Returns: `0xff00000000000002::external_address::ExternalAddress`.
pub async fn take_nonzero(
    b: &mut PtbBuilder,
    arg0: impl super::cursor::ArgumentCursor<u8>,
) -> Argument {
    let a0 = arg0.into_argument_mut(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "external_address",
        "take_nonzero",
        Vec::new(),
        vec![a0],
    )
}
/// Returns: `address`.
pub async fn to_address(
    b: &mut PtbBuilder,
    arg0: impl ArgumentExternalAddress,
) -> Argument {
    let a0 = arg0.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "external_address",
        "to_address",
        Vec::new(),
        vec![a0],
    )
}
/// Returns: `0xff00000000000002::external_address::ExternalAddress`.
pub async fn from_address(b: &mut PtbBuilder, arg0: impl PureAddress) -> Argument {
    let a0 = arg0.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "external_address",
        "from_address",
        Vec::new(),
        vec![a0],
    )
}
/// Returns: `0xff00000000000002::external_address::ExternalAddress`.
pub async fn from_id(b: &mut PtbBuilder, arg0: impl PureID) -> Argument {
    let a0 = arg0.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "external_address",
        "from_id",
        Vec::new(),
        vec![a0],
    )
}
/// Returns: `bool`.
pub async fn is_nonzero(
    b: &mut PtbBuilder,
    arg0: impl ArgumentExternalAddress,
) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "external_address",
        "is_nonzero",
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
    pub fn externaladdress_tag(&self) -> TypeTag {
        <ExternalAddress as MoveType>::type_tag_at(self.package)
    }
}
