#![allow(unused_imports, non_snake_case, non_upper_case_globals)]
use move_bindgen_runtime::*;
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;
/// The provided index is out of bounds
pub const EINDEX: u64 = 131072u64;
/// An invalid length of bitvector was given
pub const ELENGTH: u64 = 131073u64;
pub const WORD_SIZE: u64 = 1u64;
/// The maximum allowed bitvector size
pub const MAX_SIZE: u64 = 1024u64;
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BitVector {
    pub length: u64,
    pub bit_field: Vec<bool>,
}
impl MoveType for BitVector {
    type Package = super::Package;
    const MODULE: &'static str = "bit_vector";
    const NAME: &'static str = "BitVector";
}
impl MoveArg for BitVector {
    fn pure_bytes(self) -> PureBytes {
        PureBytes(::bcs::to_bytes(&self).expect("bcs serialization failed"))
    }
}
pub trait ArgumentBitVector: PTBArgument {
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
impl ArgumentBitVector for BitVector {}
impl ArgumentBitVector for Argument {}
/// Returns: `0x1::bit_vector::BitVector`.
pub async fn new(b: &mut PtbBuilder, arg0: impl PureU64) -> Argument {
    let a0 = arg0.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "bit_vector",
        "new",
        Vec::new(),
        vec![a0],
    )
}
/// Set the bit at `bit_index` in the `bitvector` regardless of its previous state.
pub async fn set(b: &mut PtbBuilder, arg0: impl ArgumentBitVector, arg1: impl PureU64) {
    let a0 = arg0.into_argument_mut(b).await;
    let a1 = arg1.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "bit_vector",
        "set",
        Vec::new(),
        vec![a0, a1],
    );
}
/// Unset the bit at `bit_index` in the `bitvector` regardless of its previous state.
pub async fn unset(
    b: &mut PtbBuilder,
    arg0: impl ArgumentBitVector,
    arg1: impl PureU64,
) {
    let a0 = arg0.into_argument_mut(b).await;
    let a1 = arg1.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "bit_vector",
        "unset",
        Vec::new(),
        vec![a0, a1],
    );
}
/// Shift the `bitvector` left by `amount`. If `amount` is greater than the
/// bitvector's length the bitvector will be zeroed out.
pub async fn shift_left(
    b: &mut PtbBuilder,
    arg0: impl ArgumentBitVector,
    arg1: impl PureU64,
) {
    let a0 = arg0.into_argument_mut(b).await;
    let a1 = arg1.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "bit_vector",
        "shift_left",
        Vec::new(),
        vec![a0, a1],
    );
}
/// Return the value of the bit at `bit_index` in the `bitvector`. `true`
/// represents "1" and `false` represents a 0
///
/// Returns: `bool`.
pub async fn is_index_set(
    b: &mut PtbBuilder,
    arg0: impl ArgumentBitVector,
    arg1: impl PureU64,
) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    let a1 = arg1.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "bit_vector",
        "is_index_set",
        Vec::new(),
        vec![a0, a1],
    )
}
/// Return the length (number of usable bits) of this bitvector
///
/// Returns: `u64`.
pub async fn length(b: &mut PtbBuilder, arg0: impl ArgumentBitVector) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "bit_vector",
        "length",
        Vec::new(),
        vec![a0],
    )
}
/// Returns the length of the longest sequence of set bits starting at (and
/// including) `start_index` in the `bitvector`. If there is no such
/// sequence, then `0` is returned.
///
/// Returns: `u64`.
pub async fn longest_set_sequence_starting_at(
    b: &mut PtbBuilder,
    arg0: impl ArgumentBitVector,
    arg1: impl PureU64,
) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    let a1 = arg1.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "bit_vector",
        "longest_set_sequence_starting_at",
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
    pub fn bitvector_tag(&self) -> TypeTag {
        <BitVector as MoveType>::type_tag_at(self.package)
    }
}
