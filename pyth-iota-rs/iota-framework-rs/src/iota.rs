#![allow(unused_imports, non_snake_case, non_upper_case_globals)]
//! Coin<IOTA> is the token used to pay for gas in IOTA.
//! It has 9 decimals, and the smallest unit (10^-9) is called "nano".
use move_bindgen_runtime::*;
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;
pub const EAlreadyMinted: u64 = 0u64;
/// Sender is not @0x0 the system address.
pub const ENotSystemAddress: u64 = 1u64;
/// Name of the coin
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct IOTA {
    pub dummy_field: bool,
}
impl MoveType for IOTA {
    type Package = super::Package;
    const MODULE: &'static str = "iota";
    const NAME: &'static str = "IOTA";
}
impl MoveArg for IOTA {
    fn pure_bytes(self) -> PureBytes {
        PureBytes(::bcs::to_bytes(&self).expect("bcs serialization failed"))
    }
}
pub trait ArgumentIOTA: PTBArgument {
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
impl ArgumentIOTA for IOTA {}
impl ArgumentIOTA for Argument {}
/// The IOTA token treasury capability.
/// Protects the token from unauthorized changes.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct IotaTreasuryCap {
    pub inner: super::coin::TreasuryCap<IOTA>,
}
impl MoveType for IotaTreasuryCap {
    type Package = super::Package;
    const MODULE: &'static str = "iota";
    const NAME: &'static str = "IotaTreasuryCap";
}
impl MoveArg for IotaTreasuryCap {
    fn pure_bytes(self) -> PureBytes {
        PureBytes(::bcs::to_bytes(&self).expect("bcs serialization failed"))
    }
}
pub trait ArgumentIotaTreasuryCap: PTBArgument {
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
impl ArgumentIotaTreasuryCap for IotaTreasuryCap {}
impl ArgumentIotaTreasuryCap for Argument {}
pub async fn transfer(
    b: &mut PtbBuilder,
    arg0: impl super::coin::ArgumentCoin<IOTA>,
    arg1: impl PureAddress,
) {
    let a0 = arg0.into_argument(b).await;
    let a1 = arg1.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "iota",
        "transfer",
        Vec::new(),
        vec![a0, a1],
    );
}
/// Create an IOTA coin worth `value` and increase the total supply in `cap` accordingly.
///
/// Returns: `0x2::coin::Coin<0x2::iota::IOTA>`.
pub async fn mint(
    b: &mut PtbBuilder,
    arg0: impl ArgumentIotaTreasuryCap,
    arg1: impl PureU64,
) -> Argument {
    let a0 = arg0.into_argument_mut(b).await;
    let a1 = arg1.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "iota",
        "mint",
        Vec::new(),
        vec![a0, a1],
    )
}
/// Mint some amount of IOTA as a `Balance` and increase the total supply in `cap` accordingly.
/// Aborts if `value` + `cap.inner.total_supply` >= U64_MAX
///
/// Returns: `0x2::balance::Balance<0x2::iota::IOTA>`.
pub async fn mint_balance(
    b: &mut PtbBuilder,
    arg0: impl ArgumentIotaTreasuryCap,
    arg1: impl PureU64,
) -> Argument {
    let a0 = arg0.into_argument_mut(b).await;
    let a1 = arg1.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "iota",
        "mint_balance",
        Vec::new(),
        vec![a0, a1],
    )
}
/// Destroy the IOTA coin `c` and decrease the total supply in `cap` accordingly.
///
/// Returns: `u64`.
pub async fn burn(
    b: &mut PtbBuilder,
    arg0: impl ArgumentIotaTreasuryCap,
    arg1: impl super::coin::ArgumentCoin<IOTA>,
) -> Argument {
    let a0 = arg0.into_argument_mut(b).await;
    let a1 = arg1.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "iota",
        "burn",
        Vec::new(),
        vec![a0, a1],
    )
}
/// Destroy the IOTA balance `b` and decrease the total supply in `cap` accordingly.
///
/// Returns: `u64`.
pub async fn burn_balance(
    b: &mut PtbBuilder,
    arg0: impl ArgumentIotaTreasuryCap,
    arg1: impl super::balance::ArgumentBalance<IOTA>,
) -> Argument {
    let a0 = arg0.into_argument_mut(b).await;
    let a1 = arg1.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "iota",
        "burn_balance",
        Vec::new(),
        vec![a0, a1],
    )
}
/// Return the total number of IOTA's in circulation.
///
/// Returns: `u64`.
pub async fn total_supply(
    b: &mut PtbBuilder,
    arg0: impl ArgumentIotaTreasuryCap,
) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "iota",
        "total_supply",
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
    pub fn iota_tag(&self) -> TypeTag {
        <IOTA as MoveType>::type_tag_at(self.package)
    }
    pub fn iotatreasurycap_tag(&self) -> TypeTag {
        <IotaTreasuryCap as MoveType>::type_tag_at(self.package)
    }
}
