#![allow(unused_imports, non_snake_case, non_upper_case_globals)]
use move_bindgen_runtime::*;
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;
pub const PRICE_FEED_MESSAGE_TYPE: u8 = 0u8;
pub const E_INVALID_UPDATE_DATA: u64 = 245u64;
pub const E_INVALID_PROOF: u64 = 345u64;
pub const E_INVALID_WORMHOLE_MESSAGE: u64 = 454u64;
pub const E_INVALID_ACCUMULATOR_PAYLOAD: u64 = 554u64;
pub const E_INVALID_ACCUMULATOR_HEADER: u64 = 657u64;
pub const ACCUMULATOR_UPDATE_WORMHOLE_VERIFICATION_MAGIC: u32 = 1096111958u32;
pub const PYTHNET_ACCUMULATOR_UPDATE_MAGIC: u64 = 1347305813u64;
pub const MAJOR_VERSION: u8 = 1u8;
