//! Pyth accumulator-update parser. The blob Hermes returns under
//! `binary.data[i]` (after base64 decode) is *not* a raw Wormhole VAA
//! — it's a Pyth accumulator message that *contains* a VAA. The
//! on-chain entry point
//! `pyth::create_authenticated_price_infos_using_accumulator` wants
//! the full accumulator bytes **and** the VAA pre-verified by
//! `wormhole::vaa::parse_and_verify`, so we have to fish the VAA out
//! client-side.
//!
//! Layout we care about (proof_type = 0 / wormhole_merkle):
//!
//! ```text
//!   magic            4 bytes  "PNAU"  (0x504e4155)
//!   major_version    1 byte
//!   minor_version    1 byte
//!   trailing_size    1 byte
//!   trailing         trailing_size bytes  (skipped — forward-compat)
//!   proof_type       1 byte  (0 = wormhole_merkle)
//!   vaa_size         2 bytes (big-endian u16)
//!   vaa              vaa_size bytes
//!   …                (price messages + merkle proofs — the Move side
//!                     re-parses these; we don't need them here.)
//! ```

use anyhow::{anyhow, bail, Result};

const PNAU_MAGIC: [u8; 4] = [b'P', b'N', b'A', b'U'];
const PROOF_TYPE_WORMHOLE_MERKLE: u8 = 0;

/// Read the embedded Wormhole VAA bytes out of an accumulator update.
pub fn extract_vaa(accum: &[u8]) -> Result<Vec<u8>> {
    let mut r = Reader::new(accum);
    if r.take(4)? != PNAU_MAGIC {
        bail!("accumulator: missing PNAU magic");
    }
    let _major = r.u8()?;
    let _minor = r.u8()?;
    let trailing = r.u8()? as usize;
    r.skip(trailing)?;
    let proof_type = r.u8()?;
    if proof_type != PROOF_TYPE_WORMHOLE_MERKLE {
        bail!("accumulator: unsupported proof_type {proof_type}");
    }
    let vaa_size = r.u16_be()? as usize;
    let vaa = r.take(vaa_size)?.to_vec();
    Ok(vaa)
}

struct Reader<'a> {
    buf: &'a [u8],
    pos: usize,
}

impl<'a> Reader<'a> {
    fn new(buf: &'a [u8]) -> Self {
        Self { buf, pos: 0 }
    }
    fn take(&mut self, n: usize) -> Result<&'a [u8]> {
        let end = self.pos.checked_add(n).ok_or_else(|| anyhow!("overflow"))?;
        let slice = self
            .buf
            .get(self.pos..end)
            .ok_or_else(|| anyhow!("accumulator: short read at {} ({} needed)", self.pos, n))?;
        self.pos = end;
        Ok(slice)
    }
    fn skip(&mut self, n: usize) -> Result<()> {
        self.take(n).map(|_| ())
    }
    fn u8(&mut self) -> Result<u8> {
        Ok(self.take(1)?[0])
    }
    fn u16_be(&mut self) -> Result<u16> {
        let b = self.take(2)?;
        Ok(u16::from_be_bytes([b[0], b[1]]))
    }
}
