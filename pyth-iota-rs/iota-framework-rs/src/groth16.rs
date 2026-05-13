#![allow(unused_imports, non_snake_case, non_upper_case_globals)]
use move_bindgen_runtime::*;
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;
pub const EInvalidVerifyingKey: u64 = 0u64;
pub const EInvalidCurve: u64 = 1u64;
pub const ETooManyPublicInputs: u64 = 2u64;
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Curve {
    pub id: u8,
}
impl MoveType for Curve {
    type Package = super::Package;
    const MODULE: &'static str = "groth16";
    const NAME: &'static str = "Curve";
}
impl MoveArg for Curve {
    fn pure_bytes(self) -> PureBytes {
        PureBytes(::bcs::to_bytes(&self).expect("bcs serialization failed"))
    }
}
pub trait ArgumentCurve: PTBArgument {
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
impl ArgumentCurve for Curve {}
impl ArgumentCurve for Argument {}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PreparedVerifyingKey {
    pub vk_gamma_abc_g1_bytes: Vec<u8>,
    pub alpha_g1_beta_g2_bytes: Vec<u8>,
    pub gamma_g2_neg_pc_bytes: Vec<u8>,
    pub delta_g2_neg_pc_bytes: Vec<u8>,
}
impl MoveType for PreparedVerifyingKey {
    type Package = super::Package;
    const MODULE: &'static str = "groth16";
    const NAME: &'static str = "PreparedVerifyingKey";
}
impl MoveArg for PreparedVerifyingKey {
    fn pure_bytes(self) -> PureBytes {
        PureBytes(::bcs::to_bytes(&self).expect("bcs serialization failed"))
    }
}
pub trait ArgumentPreparedVerifyingKey: PTBArgument {
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
impl ArgumentPreparedVerifyingKey for PreparedVerifyingKey {}
impl ArgumentPreparedVerifyingKey for Argument {}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PublicProofInputs {
    pub bytes: Vec<u8>,
}
impl MoveType for PublicProofInputs {
    type Package = super::Package;
    const MODULE: &'static str = "groth16";
    const NAME: &'static str = "PublicProofInputs";
}
impl MoveArg for PublicProofInputs {
    fn pure_bytes(self) -> PureBytes {
        PureBytes(::bcs::to_bytes(&self).expect("bcs serialization failed"))
    }
}
pub trait ArgumentPublicProofInputs: PTBArgument {
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
impl ArgumentPublicProofInputs for PublicProofInputs {}
impl ArgumentPublicProofInputs for Argument {}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ProofPoints {
    pub bytes: Vec<u8>,
}
impl MoveType for ProofPoints {
    type Package = super::Package;
    const MODULE: &'static str = "groth16";
    const NAME: &'static str = "ProofPoints";
}
impl MoveArg for ProofPoints {
    fn pure_bytes(self) -> PureBytes {
        PureBytes(::bcs::to_bytes(&self).expect("bcs serialization failed"))
    }
}
pub trait ArgumentProofPoints: PTBArgument {
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
impl ArgumentProofPoints for ProofPoints {}
impl ArgumentProofPoints for Argument {}
/// Returns: `0x2::groth16::Curve`.
pub async fn bls12381(b: &mut PtbBuilder) -> Argument {
    b.move_call(
        b.package_id::<super::Package>(),
        "groth16",
        "bls12381",
        Vec::new(),
        vec![],
    )
}
/// Returns: `0x2::groth16::Curve`.
pub async fn bn254(b: &mut PtbBuilder) -> Argument {
    b.move_call(b.package_id::<super::Package>(), "groth16", "bn254", Vec::new(), vec![])
}
/// Returns: `0x2::groth16::PreparedVerifyingKey`.
pub async fn pvk_from_bytes(
    b: &mut PtbBuilder,
    arg0: impl PureVec<u8>,
    arg1: impl PureVec<u8>,
    arg2: impl PureVec<u8>,
    arg3: impl PureVec<u8>,
) -> Argument {
    let a0 = arg0.into_argument(b).await;
    let a1 = arg1.into_argument(b).await;
    let a2 = arg2.into_argument(b).await;
    let a3 = arg3.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "groth16",
        "pvk_from_bytes",
        Vec::new(),
        vec![a0, a1, a2, a3],
    )
}
/// Returns: `vector<vector<u8>>`.
pub async fn pvk_to_bytes(
    b: &mut PtbBuilder,
    arg0: impl ArgumentPreparedVerifyingKey,
) -> Argument {
    let a0 = arg0.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "groth16",
        "pvk_to_bytes",
        Vec::new(),
        vec![a0],
    )
}
/// Returns: `0x2::groth16::PublicProofInputs`.
pub async fn public_proof_inputs_from_bytes(
    b: &mut PtbBuilder,
    arg0: impl PureVec<u8>,
) -> Argument {
    let a0 = arg0.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "groth16",
        "public_proof_inputs_from_bytes",
        Vec::new(),
        vec![a0],
    )
}
/// Returns: `0x2::groth16::ProofPoints`.
pub async fn proof_points_from_bytes(
    b: &mut PtbBuilder,
    arg0: impl PureVec<u8>,
) -> Argument {
    let a0 = arg0.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "groth16",
        "proof_points_from_bytes",
        Vec::new(),
        vec![a0],
    )
}
/// Returns: `0x2::groth16::PreparedVerifyingKey`.
pub async fn prepare_verifying_key(
    b: &mut PtbBuilder,
    arg0: impl ArgumentCurve,
    arg1: impl PureVec<u8>,
) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    let a1 = arg1.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "groth16",
        "prepare_verifying_key",
        Vec::new(),
        vec![a0, a1],
    )
}
/// Returns: `bool`.
pub async fn verify_groth16_proof(
    b: &mut PtbBuilder,
    arg0: impl ArgumentCurve,
    arg1: impl ArgumentPreparedVerifyingKey,
    arg2: impl ArgumentPublicProofInputs,
    arg3: impl ArgumentProofPoints,
) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    let a1 = arg1.into_argument_ref(b).await;
    let a2 = arg2.into_argument_ref(b).await;
    let a3 = arg3.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "groth16",
        "verify_groth16_proof",
        Vec::new(),
        vec![a0, a1, a2, a3],
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
    pub fn curve_tag(&self) -> TypeTag {
        <Curve as MoveType>::type_tag_at(self.package)
    }
    pub fn preparedverifyingkey_tag(&self) -> TypeTag {
        <PreparedVerifyingKey as MoveType>::type_tag_at(self.package)
    }
    pub fn publicproofinputs_tag(&self) -> TypeTag {
        <PublicProofInputs as MoveType>::type_tag_at(self.package)
    }
    pub fn proofpoints_tag(&self) -> TypeTag {
        <ProofPoints as MoveType>::type_tag_at(self.package)
    }
}
