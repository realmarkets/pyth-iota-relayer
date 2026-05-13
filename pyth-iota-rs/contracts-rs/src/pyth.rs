#![allow(unused_imports, non_snake_case, non_upper_case_globals)]
use move_bindgen_runtime::*;
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;
pub const E_DATA_SOURCE_EMITTER_ADDRESS_AND_CHAIN_IDS_DIFFERENT_LENGTHS: u64 = 0u64;
pub const E_INVALID_DATA_SOURCE: u64 = 1u64;
pub const E_INSUFFICIENT_FEE: u64 = 2u64;
pub const E_STALE_PRICE_UPDATE: u64 = 3u64;
pub const E_UPDATE_AND_PRICE_INFO_OBJECT_MISMATCH: u64 = 4u64;
pub const E_PRICE_UPDATE_NOT_FOUND_FOR_PRICE_INFO_OBJECT: u64 = 5u64;
/// Init state and emit event corresponding to Pyth initialization.
pub async fn init_pyth(
    b: &mut PtbBuilder,
    arg0: impl super::setup::ArgumentDeployerCap,
    arg1: impl ::iota_framework_rs::package::ArgumentUpgradeCap,
    arg2: impl PureU64,
    arg3: impl PureU64,
    arg4: impl PureVec<u8>,
    arg5: impl PureVec<u64>,
    arg6: impl PureVec<Vec<u8>>,
    arg7: impl PureU64,
) {
    let a0 = arg0.into_argument(b).await;
    let a1 = arg1.into_argument(b).await;
    let a2 = arg2.into_argument(b).await;
    let a3 = arg3.into_argument(b).await;
    let a4 = arg4.into_argument(b).await;
    let a5 = arg5.into_argument(b).await;
    let a6 = arg6.into_argument(b).await;
    let a7 = arg7.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "pyth",
        "init_pyth",
        Vec::new(),
        vec![a0, a1, a2, a3, a4, a5, a6, a7],
    );
}
/// Create and share new price feed objects if they don't already exist using accumulator message.
pub async fn create_price_feeds_using_accumulator(
    b: &mut PtbBuilder,
    arg0: impl super::state::ArgumentState,
    arg1: impl PureVec<u8>,
    arg2: impl ::wormhole_rs::vaa::ArgumentVAA,
    arg3: impl ::iota_framework_rs::clock::ArgumentClock,
) {
    let a0 = arg0.into_argument_mut(b).await;
    let a1 = arg1.into_argument(b).await;
    let a2 = arg2.into_argument(b).await;
    let a3 = arg3.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "pyth",
        "create_price_feeds_using_accumulator",
        Vec::new(),
        vec![a0, a1, a2, a3],
    );
}
/// Create and share new price feed objects if they don't already exist using batch price attestation.
/// The name of the function is kept as is to remain backward compatible
pub async fn create_price_feeds(
    b: &mut PtbBuilder,
    arg0: impl super::state::ArgumentState,
    arg1: impl PureVec<::wormhole_rs::vaa::VAA>,
    arg2: impl ::iota_framework_rs::clock::ArgumentClock,
) {
    let a0 = arg0.into_argument_mut(b).await;
    let a1 = arg1.into_argument(b).await;
    let a2 = arg2.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "pyth",
        "create_price_feeds",
        Vec::new(),
        vec![a0, a1, a2],
    );
}
/// Returns: `0xff00000000000001::hot_potato_vector::HotPotatoVector<0xff00000000000001::price_info::PriceInfo>`.
pub async fn create_authenticated_price_infos_using_accumulator(
    b: &mut PtbBuilder,
    arg0: impl super::state::ArgumentState,
    arg1: impl PureVec<u8>,
    arg2: impl ::wormhole_rs::vaa::ArgumentVAA,
    arg3: impl ::iota_framework_rs::clock::ArgumentClock,
) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    let a1 = arg1.into_argument(b).await;
    let a2 = arg2.into_argument(b).await;
    let a3 = arg3.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "pyth",
        "create_authenticated_price_infos_using_accumulator",
        Vec::new(),
        vec![a0, a1, a2, a3],
    )
}
/// Creates authenticated price infos using batch price attestation
/// Name is kept as is to remain backward compatible
///
/// Returns: `0xff00000000000001::hot_potato_vector::HotPotatoVector<0xff00000000000001::price_info::PriceInfo>`.
pub async fn create_price_infos_hot_potato(
    b: &mut PtbBuilder,
    arg0: impl super::state::ArgumentState,
    arg1: impl PureVec<::wormhole_rs::vaa::VAA>,
    arg2: impl ::iota_framework_rs::clock::ArgumentClock,
) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    let a1 = arg1.into_argument(b).await;
    let a2 = arg2.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "pyth",
        "create_price_infos_hot_potato",
        Vec::new(),
        vec![a0, a1, a2],
    )
}
/// Update a singular Pyth PriceInfoObject (containing a price feed) with the
/// price data in the authenticated price infos vector (a vector of PriceInfo objects).
///
/// For more information on the end-to-end process for updating a price feed, please see the README.
///
/// The given fee must contain a sufficient number of coins to pay the update fee for the given vaas.
/// The update fee amount can be queried by calling get_update_fee(&vaas).
///
/// Please read more information about the update fee here: https://docs.pyth.network/documentation/pythnet-price-feeds/on-demand#fees
///
/// Returns: `0xff00000000000001::hot_potato_vector::HotPotatoVector<0xff00000000000001::price_info::PriceInfo>`.
pub async fn update_single_price_feed(
    b: &mut PtbBuilder,
    arg0: impl super::state::ArgumentState,
    arg1: impl super::hot_potato_vector::ArgumentHotPotatoVector<
        super::price_info::PriceInfo,
    >,
    arg2: impl super::price_info::ArgumentPriceInfoObject,
    arg3: impl ::iota_framework_rs::coin::ArgumentCoin<::iota_framework_rs::iota::IOTA>,
    arg4: impl ::iota_framework_rs::clock::ArgumentClock,
) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    let a1 = arg1.into_argument(b).await;
    let a2 = arg2.into_argument_mut(b).await;
    let a3 = arg3.into_argument(b).await;
    let a4 = arg4.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "pyth",
        "update_single_price_feed",
        Vec::new(),
        vec![a0, a1, a2, a3, a4],
    )
}
/// Determine if a price feed for the given price_identifier exists
///
/// Returns: `bool`.
pub async fn price_feed_exists(
    b: &mut PtbBuilder,
    arg0: impl super::state::ArgumentState,
    arg1: impl super::price_identifier::ArgumentPriceIdentifier,
) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    let a1 = arg1.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "pyth",
        "price_feed_exists",
        Vec::new(),
        vec![a0, a1],
    )
}
/// Get the latest available price cached for the given price identifier, if that price is
/// no older than the stale price threshold.
///
/// Please refer to the documentation at https://docs.pyth.network/documentation/pythnet-price-feeds/best-practices for
/// how to how this price safely.
///
/// Important: Pyth uses an on-demand update model, where consumers need to update the
/// cached prices before using them. Please read more about this at https://docs.pyth.network/documentation/pythnet-price-feeds/on-demand.
/// get_price() is likely to abort unless you call update_price_feeds() to update the cached price
/// beforehand, as the cached prices may be older than the stale price threshold.
///
/// The price_info_object is a Iota object with the key ability that uniquely
/// contains a price feed for a given price_identifier.
///
///
/// Returns: `0xff00000000000001::price::Price`.
pub async fn get_price(
    b: &mut PtbBuilder,
    arg0: impl super::state::ArgumentState,
    arg1: impl super::price_info::ArgumentPriceInfoObject,
    arg2: impl ::iota_framework_rs::clock::ArgumentClock,
) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    let a1 = arg1.into_argument_ref(b).await;
    let a2 = arg2.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "pyth",
        "get_price",
        Vec::new(),
        vec![a0, a1, a2],
    )
}
/// Get the latest available price cached for the given price identifier, if that price is
/// no older than the given age.
///
/// Returns: `0xff00000000000001::price::Price`.
pub async fn get_price_no_older_than(
    b: &mut PtbBuilder,
    arg0: impl super::price_info::ArgumentPriceInfoObject,
    arg1: impl ::iota_framework_rs::clock::ArgumentClock,
    arg2: impl PureU64,
) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    let a1 = arg1.into_argument_ref(b).await;
    let a2 = arg2.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "pyth",
        "get_price_no_older_than",
        Vec::new(),
        vec![a0, a1, a2],
    )
}
/// Get the latest available price cached for the given price identifier.
///
/// WARNING: the returned price can be from arbitrarily far in the past.
/// This function makes no guarantees that the returned price is recent or
/// useful for any particular application. Users of this function should check
/// the returned timestamp to ensure that the returned price is sufficiently
/// recent for their application. The checked get_price_no_older_than()
/// function should be used in preference to this.
///
/// Returns: `0xff00000000000001::price::Price`.
pub async fn get_price_unsafe(
    b: &mut PtbBuilder,
    arg0: impl super::price_info::ArgumentPriceInfoObject,
) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "pyth",
        "get_price_unsafe",
        Vec::new(),
        vec![a0],
    )
}
/// Get the stale price threshold: the amount of time after which a cached price
/// is considered stale and no longer returned by get_price()/get_ema_price().
///
/// Returns: `u64`.
pub async fn get_stale_price_threshold_secs(
    b: &mut PtbBuilder,
    arg0: impl super::state::ArgumentState,
) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "pyth",
        "get_stale_price_threshold_secs",
        Vec::new(),
        vec![a0],
    )
}
/// Please read more information about the update fee here: https://docs.pyth.network/documentation/pythnet-price-feeds/on-demand#fees
///
/// Returns: `u64`.
pub async fn get_total_update_fee(
    b: &mut PtbBuilder,
    arg0: impl super::state::ArgumentState,
    arg1: impl PureU64,
) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    let a1 = arg1.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "pyth",
        "get_total_update_fee",
        Vec::new(),
        vec![a0, a1],
    )
}
