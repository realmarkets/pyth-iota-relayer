#![allow(unused_imports, non_snake_case, non_upper_case_globals)]
//! This module provides functionality for generating secure randomness.
use move_bindgen_runtime::*;
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;
pub const ENotSystemAddress: u64 = 0u64;
pub const EWrongInnerVersion: u64 = 1u64;
pub const EInvalidRandomnessUpdate: u64 = 2u64;
pub const EInvalidRange: u64 = 3u64;
pub const EInvalidLength: u64 = 4u64;
pub const RAND_OUTPUT_LEN: u16 = 32u16;
pub const U16_MAX: u64 = 65535u64;
/// Singleton shared object which stores the global randomness state.
/// The actual state is stored in a versioned inner field.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Random {
    pub id: UID,
    pub inner: super::versioned::Versioned,
}
impl MoveType for Random {
    type Package = super::Package;
    const MODULE: &'static str = "random";
    const NAME: &'static str = "Random";
}
pub trait ArgumentRandom: PTBArgument {
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
impl ArgumentRandom for Argument {}
impl ArgumentRandom for ObjectId {
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
impl ArgumentRandom for ObjectReference {}
impl ArgumentRandom for Shared<ObjectId> {}
impl ArgumentRandom for SharedMut<ObjectId> {}
impl ArgumentRandom for Receiving<ObjectId> {}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RandomInner {
    pub version: u64,
    pub epoch: u64,
    pub randomness_round: u64,
    pub random_bytes: Vec<u8>,
}
impl MoveType for RandomInner {
    type Package = super::Package;
    const MODULE: &'static str = "random";
    const NAME: &'static str = "RandomInner";
}
impl MoveArg for RandomInner {
    fn pure_bytes(self) -> PureBytes {
        PureBytes(::bcs::to_bytes(&self).expect("bcs serialization failed"))
    }
}
pub trait ArgumentRandomInner: PTBArgument {
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
impl ArgumentRandomInner for RandomInner {}
impl ArgumentRandomInner for Argument {}
/// Unique randomness generator, derived from the global randomness.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RandomGenerator {
    pub seed: Vec<u8>,
    pub counter: u16,
    pub buffer: Vec<u8>,
}
impl MoveType for RandomGenerator {
    type Package = super::Package;
    const MODULE: &'static str = "random";
    const NAME: &'static str = "RandomGenerator";
}
impl MoveArg for RandomGenerator {
    fn pure_bytes(self) -> PureBytes {
        PureBytes(::bcs::to_bytes(&self).expect("bcs serialization failed"))
    }
}
pub trait ArgumentRandomGenerator: PTBArgument {
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
impl ArgumentRandomGenerator for RandomGenerator {}
impl ArgumentRandomGenerator for Argument {}
/// Create a generator. Can be used to derive up to MAX_U16 * 32 random bytes.
///
/// Returns: `0x2::random::RandomGenerator`.
pub async fn new_generator(b: &mut PtbBuilder, arg0: impl ArgumentRandom) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "random",
        "new_generator",
        Vec::new(),
        vec![a0],
    )
}
/// Generate n random bytes.
///
/// Returns: `vector<u8>`.
pub async fn generate_bytes(
    b: &mut PtbBuilder,
    arg0: impl ArgumentRandomGenerator,
    arg1: impl PureU16,
) -> Argument {
    let a0 = arg0.into_argument_mut(b).await;
    let a1 = arg1.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "random",
        "generate_bytes",
        Vec::new(),
        vec![a0, a1],
    )
}
/// Generate a u256.
///
/// Returns: `u256`.
pub async fn generate_u256(
    b: &mut PtbBuilder,
    arg0: impl ArgumentRandomGenerator,
) -> Argument {
    let a0 = arg0.into_argument_mut(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "random",
        "generate_u256",
        Vec::new(),
        vec![a0],
    )
}
/// Generate a u128.
///
/// Returns: `u128`.
pub async fn generate_u128(
    b: &mut PtbBuilder,
    arg0: impl ArgumentRandomGenerator,
) -> Argument {
    let a0 = arg0.into_argument_mut(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "random",
        "generate_u128",
        Vec::new(),
        vec![a0],
    )
}
/// Generate a u64.
///
/// Returns: `u64`.
pub async fn generate_u64(
    b: &mut PtbBuilder,
    arg0: impl ArgumentRandomGenerator,
) -> Argument {
    let a0 = arg0.into_argument_mut(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "random",
        "generate_u64",
        Vec::new(),
        vec![a0],
    )
}
/// Generate a u32.
///
/// Returns: `u32`.
pub async fn generate_u32(
    b: &mut PtbBuilder,
    arg0: impl ArgumentRandomGenerator,
) -> Argument {
    let a0 = arg0.into_argument_mut(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "random",
        "generate_u32",
        Vec::new(),
        vec![a0],
    )
}
/// Generate a u16.
///
/// Returns: `u16`.
pub async fn generate_u16(
    b: &mut PtbBuilder,
    arg0: impl ArgumentRandomGenerator,
) -> Argument {
    let a0 = arg0.into_argument_mut(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "random",
        "generate_u16",
        Vec::new(),
        vec![a0],
    )
}
/// Generate a u8.
///
/// Returns: `u8`.
pub async fn generate_u8(
    b: &mut PtbBuilder,
    arg0: impl ArgumentRandomGenerator,
) -> Argument {
    let a0 = arg0.into_argument_mut(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "random",
        "generate_u8",
        Vec::new(),
        vec![a0],
    )
}
/// Generate a boolean.
///
/// Returns: `bool`.
pub async fn generate_bool(
    b: &mut PtbBuilder,
    arg0: impl ArgumentRandomGenerator,
) -> Argument {
    let a0 = arg0.into_argument_mut(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "random",
        "generate_bool",
        Vec::new(),
        vec![a0],
    )
}
/// Generate a random u128 in [min, max] (with a bias of 2^{-64}).
///
/// Returns: `u128`.
pub async fn generate_u128_in_range(
    b: &mut PtbBuilder,
    arg0: impl ArgumentRandomGenerator,
    arg1: impl PureU128,
    arg2: impl PureU128,
) -> Argument {
    let a0 = arg0.into_argument_mut(b).await;
    let a1 = arg1.into_argument(b).await;
    let a2 = arg2.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "random",
        "generate_u128_in_range",
        Vec::new(),
        vec![a0, a1, a2],
    )
}
/// Returns: `u64`.
pub async fn generate_u64_in_range(
    b: &mut PtbBuilder,
    arg0: impl ArgumentRandomGenerator,
    arg1: impl PureU64,
    arg2: impl PureU64,
) -> Argument {
    let a0 = arg0.into_argument_mut(b).await;
    let a1 = arg1.into_argument(b).await;
    let a2 = arg2.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "random",
        "generate_u64_in_range",
        Vec::new(),
        vec![a0, a1, a2],
    )
}
/// Generate a random u32 in [min, max] (with a bias of 2^{-64}).
///
/// Returns: `u32`.
pub async fn generate_u32_in_range(
    b: &mut PtbBuilder,
    arg0: impl ArgumentRandomGenerator,
    arg1: impl PureU32,
    arg2: impl PureU32,
) -> Argument {
    let a0 = arg0.into_argument_mut(b).await;
    let a1 = arg1.into_argument(b).await;
    let a2 = arg2.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "random",
        "generate_u32_in_range",
        Vec::new(),
        vec![a0, a1, a2],
    )
}
/// Generate a random u16 in [min, max] (with a bias of 2^{-64}).
///
/// Returns: `u16`.
pub async fn generate_u16_in_range(
    b: &mut PtbBuilder,
    arg0: impl ArgumentRandomGenerator,
    arg1: impl PureU16,
    arg2: impl PureU16,
) -> Argument {
    let a0 = arg0.into_argument_mut(b).await;
    let a1 = arg1.into_argument(b).await;
    let a2 = arg2.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "random",
        "generate_u16_in_range",
        Vec::new(),
        vec![a0, a1, a2],
    )
}
/// Generate a random u8 in [min, max] (with a bias of 2^{-64}).
///
/// Returns: `u8`.
pub async fn generate_u8_in_range(
    b: &mut PtbBuilder,
    arg0: impl ArgumentRandomGenerator,
    arg1: impl PureU8,
    arg2: impl PureU8,
) -> Argument {
    let a0 = arg0.into_argument_mut(b).await;
    let a1 = arg1.into_argument(b).await;
    let a2 = arg2.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "random",
        "generate_u8_in_range",
        Vec::new(),
        vec![a0, a1, a2],
    )
}
/// Shuffle a vector using the random generator (Fisher–Yates/Knuth shuffle).
pub async fn shuffle<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentRandomGenerator,
    arg1: impl PureVec<T0>,
) {
    let a0 = arg0.into_argument_mut(b).await;
    let a1 = arg1.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "random",
        "shuffle",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![a0, a1],
    );
}
/// Read-only handle bound to a runtime package address. Use
/// via `super::Package::at(addr).<module>()` and chain
/// `<type>_tag()` to build a [`TypeTag`] without spinning up
/// a `PtbBuilder`.
pub struct ModuleAt {
    pub(crate) package: Address,
}
impl ModuleAt {
    pub fn random_tag(&self) -> TypeTag {
        <Random as MoveType>::type_tag_at(self.package)
    }
    pub fn randominner_tag(&self) -> TypeTag {
        <RandomInner as MoveType>::type_tag_at(self.package)
    }
    pub fn randomgenerator_tag(&self) -> TypeTag {
        <RandomGenerator as MoveType>::type_tag_at(self.package)
    }
}
