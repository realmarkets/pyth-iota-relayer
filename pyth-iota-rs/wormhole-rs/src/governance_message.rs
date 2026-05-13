#![allow(unused_imports, non_snake_case, non_upper_case_globals)]
//! This module implements a custom type representing a Guardian governance
//! action. Each governance action has an associated module name, relevant chain
//! and payload encoding instructions/data used to perform an administrative
//! change on a contract.
use move_bindgen_runtime::*;
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;
/// Guardian set used to sign VAA did not use current Guardian set.
pub const E_OLD_GUARDIAN_SET_GOVERNANCE: u64 = 0u64;
/// Governance chain does not match.
pub const E_INVALID_GOVERNANCE_CHAIN: u64 = 1u64;
/// Governance emitter address does not match.
pub const E_INVALID_GOVERNANCE_EMITTER: u64 = 2u64;
/// Governance module name does not match.
pub const E_INVALID_GOVERNANCE_MODULE: u64 = 4u64;
/// Governance action does not match.
pub const E_INVALID_GOVERNANCE_ACTION: u64 = 5u64;
/// Governance target chain not indicative of global action.
pub const E_GOVERNANCE_TARGET_CHAIN_NONZERO: u64 = 6u64;
/// Governance target chain not indicative of actino specifically for Iota
/// Wormhole contract.
pub const E_GOVERNANCE_TARGET_CHAIN_NOT_SUI: u64 = 7u64;
/// The public constructors for `DecreeTicket` (`authorize_verify_global`
/// and `authorize_verify_local`) require a witness of type `T`. This is to
/// ensure that `DecreeTicket`s cannot be mixed up between modules
/// maliciously.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DecreeTicket<T0: MoveType> {
    pub governance_chain: u16,
    pub governance_contract: super::external_address::ExternalAddress,
    pub module_name: super::bytes32::Bytes32,
    pub action: u8,
    pub global: bool,
    #[serde(skip)]
    pub _phantom_t0: PhantomData<T0>,
}
impl<T0: MoveType> MoveType for DecreeTicket<T0> {
    type Package = super::Package;
    const MODULE: &'static str = "governance_message";
    const NAME: &'static str = "DecreeTicket";
    fn type_params(addrs: &impl PackageAddrs) -> Vec<TypeTag> {
        vec![< T0 as MoveType > ::type_tag(addrs)]
    }
}
impl<T0: MoveType + ::serde::Serialize> MoveArg for DecreeTicket<T0> {
    fn pure_bytes(self) -> PureBytes {
        PureBytes(::bcs::to_bytes(&self).expect("bcs serialization failed"))
    }
}
pub trait ArgumentDecreeTicket<T0: MoveType>: PTBArgument {
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
impl<T0: MoveType + ::serde::Serialize> ArgumentDecreeTicket<T0> for DecreeTicket<T0> {}
impl<T0: MoveType> ArgumentDecreeTicket<T0> for Argument {}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DecreeReceipt<T0: MoveType> {
    pub payload: Vec<u8>,
    pub digest: super::bytes32::Bytes32,
    pub sequence: u64,
    #[serde(skip)]
    pub _phantom_t0: PhantomData<T0>,
}
impl<T0: MoveType> MoveType for DecreeReceipt<T0> {
    type Package = super::Package;
    const MODULE: &'static str = "governance_message";
    const NAME: &'static str = "DecreeReceipt";
    fn type_params(addrs: &impl PackageAddrs) -> Vec<TypeTag> {
        vec![< T0 as MoveType > ::type_tag(addrs)]
    }
}
impl<T0: MoveType + ::serde::Serialize> MoveArg for DecreeReceipt<T0> {
    fn pure_bytes(self) -> PureBytes {
        PureBytes(::bcs::to_bytes(&self).expect("bcs serialization failed"))
    }
}
pub trait ArgumentDecreeReceipt<T0: MoveType>: PTBArgument {
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
impl<T0: MoveType + ::serde::Serialize> ArgumentDecreeReceipt<T0> for DecreeReceipt<T0> {}
impl<T0: MoveType> ArgumentDecreeReceipt<T0> for Argument {}
/// This method prepares `DecreeTicket` for global governance action. This
/// means the VAA encodes target chain ID == 0.
///
/// Returns: `0xff00000000000002::governance_message::DecreeTicket<T0>`.
pub async fn authorize_verify_global<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentObject<()>,
    arg1: impl PureU16,
    arg2: impl super::external_address::ArgumentExternalAddress,
    arg3: impl super::bytes32::ArgumentBytes32,
    arg4: impl PureU8,
) -> Argument {
    let a0 = arg0.into_argument(b).await;
    let a1 = arg1.into_argument(b).await;
    let a2 = arg2.into_argument(b).await;
    let a3 = arg3.into_argument(b).await;
    let a4 = arg4.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "governance_message",
        "authorize_verify_global",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![a0, a1, a2, a3, a4],
    )
}
/// This method prepares `DecreeTicket` for local governance action. This
/// means the VAA encodes target chain ID == 21 (Iota's).
///
/// Returns: `0xff00000000000002::governance_message::DecreeTicket<T0>`.
pub async fn authorize_verify_local<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentObject<()>,
    arg1: impl PureU16,
    arg2: impl super::external_address::ArgumentExternalAddress,
    arg3: impl super::bytes32::ArgumentBytes32,
    arg4: impl PureU8,
) -> Argument {
    let a0 = arg0.into_argument(b).await;
    let a1 = arg1.into_argument(b).await;
    let a2 = arg2.into_argument(b).await;
    let a3 = arg3.into_argument(b).await;
    let a4 = arg4.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "governance_message",
        "authorize_verify_local",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![a0, a1, a2, a3, a4],
    )
}
/// Returns: `u64`.
pub async fn sequence<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentDecreeReceipt<T0>,
) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "governance_message",
        "sequence",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![a0],
    )
}
/// This method unpacks `DecreeReceipt` and puts the VAA digest into a
/// `ConsumedVAAs` container. Then it returns the governance payload.
///
/// Returns: `vector<u8>`.
pub async fn take_payload<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl super::consumed_vaas::ArgumentConsumedVAAs,
    arg1: impl ArgumentDecreeReceipt<T0>,
) -> Argument {
    let a0 = arg0.into_argument_mut(b).await;
    let a1 = arg1.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "governance_message",
        "take_payload",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![a0, a1],
    )
}
/// Method to peek into the payload in `DecreeReceipt`.
///
/// Returns: `vector<u8>`.
pub async fn payload<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentDecreeReceipt<T0>,
) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "governance_message",
        "payload",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![a0],
    )
}
/// Destroy the receipt.
pub async fn destroy<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentDecreeReceipt<T0>,
) {
    let a0 = arg0.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "governance_message",
        "destroy",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![a0],
    );
}
/// This method unpacks a `DecreeTicket` to validate its members to make
/// sure that the parameters match what was encoded in the VAA.
///
/// Returns: `0xff00000000000002::governance_message::DecreeReceipt<T0>`.
pub async fn verify_vaa<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl super::state::ArgumentState,
    arg1: impl super::vaa::ArgumentVAA,
    arg2: impl ArgumentDecreeTicket<T0>,
) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    let a1 = arg1.into_argument(b).await;
    let a2 = arg2.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "governance_message",
        "verify_vaa",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![a0, a1, a2],
    )
}
