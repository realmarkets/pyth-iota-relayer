//! Helpers for loading the relayer's Ed25519 key from a `iotaprivkey1…`
//! Bech32 string supplied via `--key` / `IOTA_PRIVATE_KEY`.

use anyhow::{anyhow, Context, Result};
use iota_sdk_crypto::{ed25519::Ed25519PrivateKey, ToFromBech32};
use iota_sdk_types::Address;

/// Decode the Bech32 key string and return the (signer, derived
/// sender address) pair.
pub fn load(key_string: &str) -> Result<(Ed25519PrivateKey, Address)> {
    let signer = Ed25519PrivateKey::from_bech32(key_string)
        .map_err(|e| anyhow!("decoding iotaprivkey1… key: {e}"))?;
    let address = signer.public_key().derive_address();
    Ok((signer, address))
}

/// Require `--key` / `IOTA_PRIVATE_KEY` to be set. Wraps the option
/// into a clear error.
pub fn require(key: Option<&str>) -> Result<&str> {
    key.context("no private key configured — pass `--key iotaprivkey1…` or set `IOTA_PRIVATE_KEY`")
}
