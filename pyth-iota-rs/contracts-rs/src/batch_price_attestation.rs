#![allow(unused_imports, non_snake_case, non_upper_case_globals)]
use move_bindgen_runtime::*;
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;
pub const MAGIC: u64 = 1345476424u64;
pub const E_INVALID_ATTESTATION_MAGIC_VALUE: u64 = 0u64;
pub const E_INVALID_BATCH_ATTESTATION_HEADER_SIZE: u64 = 1u64;
/// @notice This struct is based on the legacy wormhole attester implementation in pythnet_sdk
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BatchPriceAttestation {
    pub header: Header,
    pub attestation_size: u64,
    pub attestation_count: u64,
    pub price_infos: Vec<super::price_info::PriceInfo>,
}
impl MoveType for BatchPriceAttestation {
    type Package = super::Package;
    const MODULE: &'static str = "batch_price_attestation";
    const NAME: &'static str = "BatchPriceAttestation";
}
impl MoveArg for BatchPriceAttestation {
    fn pure_bytes(self) -> PureBytes {
        PureBytes(::bcs::to_bytes(&self).expect("bcs serialization failed"))
    }
}
pub trait ArgumentBatchPriceAttestation: PTBArgument {
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
impl ArgumentBatchPriceAttestation for BatchPriceAttestation {}
impl ArgumentBatchPriceAttestation for Argument {}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Header {
    pub magic: u64,
    pub version_major: u64,
    pub version_minor: u64,
    pub header_size: u64,
    pub payload_id: u8,
}
impl MoveType for Header {
    type Package = super::Package;
    const MODULE: &'static str = "batch_price_attestation";
    const NAME: &'static str = "Header";
}
impl MoveArg for Header {
    fn pure_bytes(self) -> PureBytes {
        PureBytes(::bcs::to_bytes(&self).expect("bcs serialization failed"))
    }
}
pub trait ArgumentHeader: PTBArgument {
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
impl ArgumentHeader for Header {}
impl ArgumentHeader for Argument {}
/// Returns: `vector<0xff00000000000001::price_info::PriceInfo>`.
pub async fn destroy(
    b: &mut PtbBuilder,
    arg0: impl ArgumentBatchPriceAttestation,
) -> Argument {
    let a0 = arg0.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "batch_price_attestation",
        "destroy",
        Vec::new(),
        vec![a0],
    )
}
/// Returns: `u64`.
pub async fn get_attestation_count(
    b: &mut PtbBuilder,
    arg0: impl ArgumentBatchPriceAttestation,
) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "batch_price_attestation",
        "get_attestation_count",
        Vec::new(),
        vec![a0],
    )
}
/// Returns: `&0xff00000000000001::price_info::PriceInfo`.
pub async fn get_price_info(
    b: &mut PtbBuilder,
    arg0: impl ArgumentBatchPriceAttestation,
    arg1: impl PureU64,
) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    let a1 = arg1.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "batch_price_attestation",
        "get_price_info",
        Vec::new(),
        vec![a0, a1],
    )
}
/// Returns: `0xff00000000000001::batch_price_attestation::BatchPriceAttestation`.
pub async fn deserialize(
    b: &mut PtbBuilder,
    arg0: impl PureVec<u8>,
    arg1: impl ::iota_framework_rs::clock::ArgumentClock,
) -> Argument {
    let a0 = arg0.into_argument(b).await;
    let a1 = arg1.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "batch_price_attestation",
        "deserialize",
        Vec::new(),
        vec![a0, a1],
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
    pub fn batchpriceattestation_tag(&self) -> TypeTag {
        <BatchPriceAttestation as MoveType>::type_tag_at(self.package)
    }
    pub fn header_tag(&self) -> TypeTag {
        <Header as MoveType>::type_tag_at(self.package)
    }
}
