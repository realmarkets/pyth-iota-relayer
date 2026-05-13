#![allow(unused_imports, non_snake_case, non_upper_case_globals)]
//! This module implements a mechanism to parse and verify VAAs, which are
//! verified Wormhole messages (messages with Guardian signatures attesting to
//! its observation). Signatures on VAA are checked against an existing Guardian
//! set that exists in the `State` (see `wormhole::state`).
//!
//! A Wormhole integrator is discouraged from integrating `parse_and_verify` in
//! his contract. If there is a breaking change to the `vaa` module, Wormhole
//! will be upgraded to prevent previous build versions of this module to work.
//! If an integrator happened to use `parse_and_verify` in his contract, he will
//! need to be prepared to upgrade his contract to take the change (by building
//! with the latest package implementation).
//!
//! Instead, an integrator is encouraged to execute a transaction block, which
//! executes `parse_and_verify` from the latest Wormhole package ID and to
//! implement his methods that require redeeming a VAA to take `VAA` as an
//! argument.
//!
//! A good example of how this methodology is implemented is how the Token
//! Bridge contract redeems its VAAs.
use move_bindgen_runtime::*;
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;
/// Incorrect VAA version.
pub const E_WRONG_VERSION: u64 = 0u64;
/// Not enough guardians attested to this Wormhole observation.
pub const E_NO_QUORUM: u64 = 1u64;
/// Signature does not match expected Guardian public key.
pub const E_INVALID_SIGNATURE: u64 = 2u64;
/// Prior guardian set is no longer valid.
pub const E_GUARDIAN_SET_EXPIRED: u64 = 3u64;
/// Guardian signature is encoded out of sequence.
pub const E_NON_INCREASING_SIGNERS: u64 = 4u64;
pub const VERSION_VAA: u8 = 1u8;
/// Container storing verified Wormhole message info. This struct also
/// caches the digest, which is a double Keccak256 hash of the message body.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct VAA {
    /// Guardian set index of Guardians that attested to observing the
    /// Wormhole message.
    pub guardian_set_index: u32,
    /// Time when Wormhole message was emitted or observed.
    pub timestamp: u32,
    /// A.K.A. Batch ID.
    pub nonce: u32,
    /// Wormhole chain ID from which network the message originated from.
    pub emitter_chain: u16,
    /// Address of contract (standardized to 32 bytes) that produced the
    /// message.
    pub emitter_address: super::external_address::ExternalAddress,
    /// Sequence number of emitter's Wormhole message.
    pub sequence: u64,
    /// A.K.A. Finality.
    pub consistency_level: u8,
    /// Arbitrary payload encoding data relevant to receiver.
    pub payload: Vec<u8>,
    /// Double Keccak256 hash of message body.
    pub digest: super::bytes32::Bytes32,
}
impl MoveType for VAA {
    type Package = super::Package;
    const MODULE: &'static str = "vaa";
    const NAME: &'static str = "VAA";
}
impl MoveArg for VAA {
    fn pure_bytes(self) -> PureBytes {
        PureBytes(::bcs::to_bytes(&self).expect("bcs serialization failed"))
    }
}
pub trait ArgumentVAA: PTBArgument {
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
impl ArgumentVAA for VAA {}
impl ArgumentVAA for Argument {}
/// Returns: `u32`.
pub async fn guardian_set_index(b: &mut PtbBuilder, arg0: impl ArgumentVAA) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "vaa",
        "guardian_set_index",
        Vec::new(),
        vec![a0],
    )
}
/// Returns: `u32`.
pub async fn timestamp(b: &mut PtbBuilder, arg0: impl ArgumentVAA) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "vaa",
        "timestamp",
        Vec::new(),
        vec![a0],
    )
}
/// Returns: `u32`.
pub async fn nonce(b: &mut PtbBuilder, arg0: impl ArgumentVAA) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    b.move_call(b.package_id::<super::Package>(), "vaa", "nonce", Vec::new(), vec![a0])
}
/// Returns: `u32`.
pub async fn batch_id(b: &mut PtbBuilder, arg0: impl ArgumentVAA) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "vaa",
        "batch_id",
        Vec::new(),
        vec![a0],
    )
}
/// Returns: `vector<u8>`.
pub async fn payload(b: &mut PtbBuilder, arg0: impl ArgumentVAA) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    b.move_call(b.package_id::<super::Package>(), "vaa", "payload", Vec::new(), vec![a0])
}
/// Returns: `0xff00000000000002::bytes32::Bytes32`.
pub async fn digest(b: &mut PtbBuilder, arg0: impl ArgumentVAA) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    b.move_call(b.package_id::<super::Package>(), "vaa", "digest", Vec::new(), vec![a0])
}
/// Returns: `u16`.
pub async fn emitter_chain(b: &mut PtbBuilder, arg0: impl ArgumentVAA) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "vaa",
        "emitter_chain",
        Vec::new(),
        vec![a0],
    )
}
/// Returns: `0xff00000000000002::external_address::ExternalAddress`.
pub async fn emitter_address(b: &mut PtbBuilder, arg0: impl ArgumentVAA) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "vaa",
        "emitter_address",
        Vec::new(),
        vec![a0],
    )
}
/// Returns: `(u16, 0xff00000000000002::external_address::ExternalAddress, u64)`.
pub async fn emitter_info(
    b: &mut PtbBuilder,
    arg0: impl ArgumentVAA,
) -> (Argument, Argument, Argument) {
    let a0 = arg0.into_argument_ref(b).await;
    let __r = b
        .move_call_n(
            b.package_id::<super::Package>(),
            "vaa",
            "emitter_info",
            Vec::new(),
            vec![a0],
            3u16,
        );
    (__r[0], __r[1], __r[2])
}
/// Returns: `u64`.
pub async fn sequence(b: &mut PtbBuilder, arg0: impl ArgumentVAA) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "vaa",
        "sequence",
        Vec::new(),
        vec![a0],
    )
}
/// Returns: `u8`.
pub async fn consistency_level(b: &mut PtbBuilder, arg0: impl ArgumentVAA) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "vaa",
        "consistency_level",
        Vec::new(),
        vec![a0],
    )
}
/// Returns: `u8`.
pub async fn finality(b: &mut PtbBuilder, arg0: impl ArgumentVAA) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "vaa",
        "finality",
        Vec::new(),
        vec![a0],
    )
}
/// Destroy the `VAA` and take the Wormhole message payload.
///
/// Returns: `vector<u8>`.
pub async fn take_payload(b: &mut PtbBuilder, arg0: impl ArgumentVAA) -> Argument {
    let a0 = arg0.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "vaa",
        "take_payload",
        Vec::new(),
        vec![a0],
    )
}
/// Destroy the `VAA` and take emitter info (chain and address) and Wormhole
/// message payload.
///
/// Returns: `(u16, 0xff00000000000002::external_address::ExternalAddress, vector<u8>)`.
pub async fn take_emitter_info_and_payload(
    b: &mut PtbBuilder,
    arg0: impl ArgumentVAA,
) -> (Argument, Argument, Argument) {
    let a0 = arg0.into_argument(b).await;
    let __r = b
        .move_call_n(
            b.package_id::<super::Package>(),
            "vaa",
            "take_emitter_info_and_payload",
            Vec::new(),
            vec![a0],
            3u16,
        );
    (__r[0], __r[1], __r[2])
}
/// Parses and verifies the signatures of a VAA.
///
/// NOTE: This is the only public function that returns a VAA, and it should
/// be kept that way. This ensures that if an external module receives a
/// `VAA`, it has been verified.
///
/// Returns: `0xff00000000000002::vaa::VAA`.
pub async fn parse_and_verify(
    b: &mut PtbBuilder,
    arg0: impl super::state::ArgumentState,
    arg1: impl PureVec<u8>,
    arg2: impl ::iota_framework_rs::clock::ArgumentClock,
) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    let a1 = arg1.into_argument(b).await;
    let a2 = arg2.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "vaa",
        "parse_and_verify",
        Vec::new(),
        vec![a0, a1, a2],
    )
}
pub async fn consume(
    b: &mut PtbBuilder,
    arg0: impl super::consumed_vaas::ArgumentConsumedVAAs,
    arg1: impl ArgumentVAA,
) {
    let a0 = arg0.into_argument_mut(b).await;
    let a1 = arg1.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "vaa",
        "consume",
        Vec::new(),
        vec![a0, a1],
    );
}
/// Returns: `0xff00000000000002::bytes32::Bytes32`.
pub async fn compute_message_hash(
    b: &mut PtbBuilder,
    arg0: impl ArgumentVAA,
) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "vaa",
        "compute_message_hash",
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
    pub fn vaa_tag(&self) -> TypeTag {
        <VAA as MoveType>::type_tag_at(self.package)
    }
}
