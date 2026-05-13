#![allow(unused_imports, non_snake_case, non_upper_case_globals)]
use move_bindgen_runtime::*;
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;
pub const E_DUPLICATE_GUARDIAN: u64 = 0u64;
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GuardianSet {
    pub index: u32,
    pub guardians: Vec<super::guardian::Guardian>,
    pub expiration_timestamp_ms: u64,
}
impl MoveType for GuardianSet {
    type Package = super::Package;
    const MODULE: &'static str = "guardian_set";
    const NAME: &'static str = "GuardianSet";
}
impl MoveArg for GuardianSet {
    fn pure_bytes(self) -> PureBytes {
        PureBytes(::bcs::to_bytes(&self).expect("bcs serialization failed"))
    }
}
pub trait ArgumentGuardianSet: PTBArgument {
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
impl ArgumentGuardianSet for GuardianSet {}
impl ArgumentGuardianSet for Argument {}
/// Returns: `0xff00000000000002::guardian_set::GuardianSet`.
pub async fn new(
    b: &mut PtbBuilder,
    arg0: impl PureU32,
    arg1: impl PureVec<super::guardian::Guardian>,
) -> Argument {
    let a0 = arg0.into_argument(b).await;
    let a1 = arg1.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "guardian_set",
        "new",
        Vec::new(),
        vec![a0, a1],
    )
}
/// Returns: `u32`.
pub async fn index(b: &mut PtbBuilder, arg0: impl ArgumentGuardianSet) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "guardian_set",
        "index",
        Vec::new(),
        vec![a0],
    )
}
/// Returns: `u64`.
pub async fn index_as_u64(
    b: &mut PtbBuilder,
    arg0: impl ArgumentGuardianSet,
) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "guardian_set",
        "index_as_u64",
        Vec::new(),
        vec![a0],
    )
}
/// Returns: `&vector<0xff00000000000002::guardian::Guardian>`.
pub async fn guardians(b: &mut PtbBuilder, arg0: impl ArgumentGuardianSet) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "guardian_set",
        "guardians",
        Vec::new(),
        vec![a0],
    )
}
/// Returns: `&0xff00000000000002::guardian::Guardian`.
pub async fn guardian_at(
    b: &mut PtbBuilder,
    arg0: impl ArgumentGuardianSet,
    arg1: impl PureU64,
) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    let a1 = arg1.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "guardian_set",
        "guardian_at",
        Vec::new(),
        vec![a0, a1],
    )
}
/// Returns: `u64`.
pub async fn expiration_timestamp_ms(
    b: &mut PtbBuilder,
    arg0: impl ArgumentGuardianSet,
) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "guardian_set",
        "expiration_timestamp_ms",
        Vec::new(),
        vec![a0],
    )
}
/// Returns: `bool`.
pub async fn is_active(
    b: &mut PtbBuilder,
    arg0: impl ArgumentGuardianSet,
    arg1: impl ::iota_framework_rs::clock::ArgumentClock,
) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    let a1 = arg1.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "guardian_set",
        "is_active",
        Vec::new(),
        vec![a0, a1],
    )
}
/// Returns: `u64`.
pub async fn num_guardians(
    b: &mut PtbBuilder,
    arg0: impl ArgumentGuardianSet,
) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "guardian_set",
        "num_guardians",
        Vec::new(),
        vec![a0],
    )
}
/// Returns: `u64`.
pub async fn quorum(b: &mut PtbBuilder, arg0: impl ArgumentGuardianSet) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "guardian_set",
        "quorum",
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
    pub fn guardianset_tag(&self) -> TypeTag {
        <GuardianSet as MoveType>::type_tag_at(self.package)
    }
}
