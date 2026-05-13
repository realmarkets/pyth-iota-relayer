#![allow(unused_imports, non_snake_case, non_upper_case_globals)]
use move_bindgen_runtime::*;
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;
pub const E_INCORRECT_FEE: u64 = 0u64;
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FeeCollector {
    pub fee_amount: u64,
    pub balance: ::iota_framework_rs::balance::Balance<::iota_framework_rs::iota::IOTA>,
}
impl MoveType for FeeCollector {
    type Package = super::Package;
    const MODULE: &'static str = "fee_collector";
    const NAME: &'static str = "FeeCollector";
}
impl MoveArg for FeeCollector {
    fn pure_bytes(self) -> PureBytes {
        PureBytes(::bcs::to_bytes(&self).expect("bcs serialization failed"))
    }
}
pub trait ArgumentFeeCollector: PTBArgument {
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
impl ArgumentFeeCollector for FeeCollector {}
impl ArgumentFeeCollector for Argument {}
/// Returns: `0xff00000000000002::fee_collector::FeeCollector`.
pub async fn new(b: &mut PtbBuilder, arg0: impl PureU64) -> Argument {
    let a0 = arg0.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "fee_collector",
        "new",
        Vec::new(),
        vec![a0],
    )
}
/// Returns: `u64`.
pub async fn fee_amount(
    b: &mut PtbBuilder,
    arg0: impl ArgumentFeeCollector,
) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "fee_collector",
        "fee_amount",
        Vec::new(),
        vec![a0],
    )
}
/// Returns: `u64`.
pub async fn balance_value(
    b: &mut PtbBuilder,
    arg0: impl ArgumentFeeCollector,
) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "fee_collector",
        "balance_value",
        Vec::new(),
        vec![a0],
    )
}
pub async fn deposit_balance(
    b: &mut PtbBuilder,
    arg0: impl ArgumentFeeCollector,
    arg1: impl ::iota_framework_rs::balance::ArgumentBalance<
        ::iota_framework_rs::iota::IOTA,
    >,
) {
    let a0 = arg0.into_argument_mut(b).await;
    let a1 = arg1.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "fee_collector",
        "deposit_balance",
        Vec::new(),
        vec![a0, a1],
    );
}
pub async fn deposit(
    b: &mut PtbBuilder,
    arg0: impl ArgumentFeeCollector,
    arg1: impl ::iota_framework_rs::coin::ArgumentCoin<::iota_framework_rs::iota::IOTA>,
) {
    let a0 = arg0.into_argument_mut(b).await;
    let a1 = arg1.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "fee_collector",
        "deposit",
        Vec::new(),
        vec![a0, a1],
    );
}
/// Returns: `0x2::balance::Balance<0x2::iota::IOTA>`.
pub async fn withdraw_balance(
    b: &mut PtbBuilder,
    arg0: impl ArgumentFeeCollector,
    arg1: impl PureU64,
) -> Argument {
    let a0 = arg0.into_argument_mut(b).await;
    let a1 = arg1.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "fee_collector",
        "withdraw_balance",
        Vec::new(),
        vec![a0, a1],
    )
}
/// Returns: `0x2::coin::Coin<0x2::iota::IOTA>`.
pub async fn withdraw(
    b: &mut PtbBuilder,
    arg0: impl ArgumentFeeCollector,
    arg1: impl PureU64,
) -> Argument {
    let a0 = arg0.into_argument_mut(b).await;
    let a1 = arg1.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "fee_collector",
        "withdraw",
        Vec::new(),
        vec![a0, a1],
    )
}
pub async fn change_fee(
    b: &mut PtbBuilder,
    arg0: impl ArgumentFeeCollector,
    arg1: impl PureU64,
) {
    let a0 = arg0.into_argument_mut(b).await;
    let a1 = arg1.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "fee_collector",
        "change_fee",
        Vec::new(),
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
    pub fn feecollector_tag(&self) -> TypeTag {
        <FeeCollector as MoveType>::type_tag_at(self.package)
    }
}
