//! Per-network Pyth + Wormhole on-chain addresses.
//!
//! Hard-coded against the `iota-contract-testnet` branch of Pyth's
//! crosschain repo (see `move-bindgen.toml`). If Pyth redeploys these
//! values change — bump here, regen, recompile.

use iota_sdk_types::{Address, ObjectId};

use crate::cli::Network;

#[derive(Debug, Clone, Copy)]
pub struct Contracts {
    pub pyth_state: ObjectId,
    pub pyth_package: Address,
    pub wormhole_state: ObjectId,
    pub wormhole_package: Address,
}

impl Contracts {
    pub fn for_network(network: Network) -> Self {
        match network {
            Network::Testnet => TESTNET,
            Network::Mainnet => MAINNET,
        }
    }
}

/// Compile-time `ObjectId` literal from a hex string. Panics at
/// compile time on bad input — never at runtime.
const fn object_id_from_hex(hex: &str) -> ObjectId {
    ObjectId::new(addr_bytes_from_hex(hex))
}

/// Compile-time `Address` literal from a hex string.
const fn address_from_hex(hex: &str) -> Address {
    Address::new(addr_bytes_from_hex(hex))
}

/// Parse a 0x-prefixed 32-byte hex string into a `[u8; 32]` at
/// const-eval time. Accepts uppercase or lowercase nibbles.
const fn addr_bytes_from_hex(hex: &str) -> [u8; 32] {
    let bytes = hex.as_bytes();
    // Tolerate the leading "0x".
    let (offset, len) =
        if bytes.len() >= 2 && bytes[0] == b'0' && (bytes[1] == b'x' || bytes[1] == b'X') {
            (2, bytes.len() - 2)
        } else {
            (0, bytes.len())
        };
    assert!(len == 64, "address hex must be 32 bytes (64 hex chars)");
    let mut out = [0u8; 32];
    let mut i = 0;
    while i < 32 {
        let hi = hex_digit(bytes[offset + 2 * i]);
        let lo = hex_digit(bytes[offset + 2 * i + 1]);
        out[i] = (hi << 4) | lo;
        i += 1;
    }
    out
}

const fn hex_digit(b: u8) -> u8 {
    match b {
        b'0'..=b'9' => b - b'0',
        b'a'..=b'f' => 10 + (b - b'a'),
        b'A'..=b'F' => 10 + (b - b'A'),
        _ => panic!("invalid hex digit"),
    }
}

pub const TESTNET: Contracts = Contracts {
    pyth_state: object_id_from_hex(
        "0x68dda579251917b3db28e35c4df495c6e664ccc085ede867a9b773c8ebedc2c1",
    ),
    pyth_package: address_from_hex(
        "0x23994dd119480ea614f7623520337058dca913cb1bb6e5d8d51c7b067d3ca3bb",
    ),
    wormhole_state: object_id_from_hex(
        "0x8bc490f69520a97ca1b3de864c96aa2265a0cf5d90f5f3f016b2eddf0cf2af2b",
    ),
    wormhole_package: address_from_hex(
        "0xfca58c557f09cddb7930588c4e2a4edbe3cdded1ac1ed2270aa2dfa8d2b9ae0d",
    ),
};

pub const MAINNET: Contracts = Contracts {
    pyth_state: object_id_from_hex(
        "0x6bc33855c7675e006f55609f61eebb1c8a104d8973a698ee9efd3127c210b37f",
    ),
    pyth_package: address_from_hex(
        "0x7792c84e1f8683dac893126712f7cf3ba5fcc82450839f0a481215f60199769f",
    ),
    wormhole_state: object_id_from_hex(
        "0xd43b448afc9dd01deb18273ec39d8f27ddd4dd46b0922383874331771b70df73",
    ),
    wormhole_package: address_from_hex(
        "0x88b00a6f1d56966d48680ffad3b42d7a25b01c519b73732a0858e0314a960801",
    ),
};
