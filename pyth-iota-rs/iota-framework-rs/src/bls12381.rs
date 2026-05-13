#![allow(unused_imports, non_snake_case, non_upper_case_globals)]
use move_bindgen_runtime::*;
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;
pub const SCALAR_ZERO_BYTES: &[u8] = &[
    0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
    0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
];
pub const SCALAR_ONE_BYTES: &[u8] = &[
    0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
    0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 1u8,
];
pub const G1_IDENTITY_BYTES: &[u8] = &[
    192u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
    0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
    0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
];
pub const G1_GENERATOR_BYTES: &[u8] = &[
    151u8, 241u8, 211u8, 167u8, 49u8, 151u8, 215u8, 148u8, 38u8, 149u8, 99u8, 140u8,
    79u8, 169u8, 172u8, 15u8, 195u8, 104u8, 140u8, 79u8, 151u8, 116u8, 185u8, 5u8, 161u8,
    78u8, 58u8, 63u8, 23u8, 27u8, 172u8, 88u8, 108u8, 85u8, 232u8, 63u8, 249u8, 122u8,
    26u8, 239u8, 251u8, 58u8, 240u8, 10u8, 219u8, 34u8, 198u8, 187u8,
];
pub const G2_IDENTITY_BYTES: &[u8] = &[
    192u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
    0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
    0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
    0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
    0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
    0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
];
pub const G2_GENERATOR_BYTES: &[u8] = &[
    147u8, 224u8, 43u8, 96u8, 82u8, 113u8, 159u8, 96u8, 125u8, 172u8, 211u8, 160u8,
    136u8, 39u8, 79u8, 101u8, 89u8, 107u8, 208u8, 208u8, 153u8, 32u8, 182u8, 26u8, 181u8,
    218u8, 97u8, 187u8, 220u8, 127u8, 80u8, 73u8, 51u8, 76u8, 241u8, 18u8, 19u8, 148u8,
    93u8, 87u8, 229u8, 172u8, 125u8, 5u8, 93u8, 4u8, 43u8, 126u8, 2u8, 74u8, 162u8,
    178u8, 240u8, 143u8, 10u8, 145u8, 38u8, 8u8, 5u8, 39u8, 45u8, 197u8, 16u8, 81u8,
    198u8, 228u8, 122u8, 212u8, 250u8, 64u8, 59u8, 2u8, 180u8, 81u8, 11u8, 100u8, 122u8,
    227u8, 209u8, 119u8, 11u8, 172u8, 3u8, 38u8, 168u8, 5u8, 187u8, 239u8, 212u8, 128u8,
    86u8, 200u8, 193u8, 33u8, 189u8, 184u8,
];
pub const GT_IDENTITY_BYTES: &[u8] = &[
    0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
    0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
    0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 1u8, 0u8, 0u8, 0u8,
    0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
    0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
    0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
    0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
    0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
    0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
    0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
    0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
    0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
    0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
    0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
    0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
    0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
    0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
    0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
    0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
    0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
    0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
    0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
    0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
    0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
    0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
    0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
    0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
    0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
    0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
    0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
    0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
    0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
    0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
    0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
];
pub const GT_GENERATOR_BYTES: &[u8] = &[
    18u8, 80u8, 235u8, 216u8, 113u8, 252u8, 10u8, 146u8, 167u8, 178u8, 216u8, 49u8,
    104u8, 208u8, 215u8, 39u8, 39u8, 45u8, 68u8, 27u8, 239u8, 161u8, 92u8, 80u8, 61u8,
    216u8, 233u8, 12u8, 233u8, 141u8, 179u8, 231u8, 182u8, 209u8, 148u8, 246u8, 8u8,
    57u8, 197u8, 8u8, 168u8, 67u8, 5u8, 170u8, 202u8, 23u8, 137u8, 182u8, 8u8, 154u8,
    28u8, 91u8, 70u8, 229u8, 17u8, 11u8, 134u8, 117u8, 14u8, 198u8, 165u8, 50u8, 52u8,
    136u8, 104u8, 168u8, 64u8, 69u8, 72u8, 60u8, 146u8, 183u8, 175u8, 90u8, 246u8, 137u8,
    69u8, 46u8, 175u8, 171u8, 241u8, 168u8, 148u8, 62u8, 80u8, 67u8, 159u8, 29u8, 89u8,
    136u8, 42u8, 152u8, 234u8, 160u8, 23u8, 15u8, 25u8, 242u8, 99u8, 55u8, 210u8, 5u8,
    251u8, 70u8, 156u8, 214u8, 189u8, 21u8, 195u8, 213u8, 160u8, 77u8, 200u8, 135u8,
    132u8, 251u8, 179u8, 208u8, 178u8, 219u8, 222u8, 165u8, 77u8, 67u8, 178u8, 183u8,
    63u8, 44u8, 187u8, 18u8, 213u8, 131u8, 134u8, 168u8, 112u8, 62u8, 15u8, 148u8, 130u8,
    38u8, 228u8, 126u8, 232u8, 157u8, 6u8, 251u8, 162u8, 62u8, 183u8, 197u8, 175u8, 13u8,
    159u8, 128u8, 148u8, 12u8, 167u8, 113u8, 182u8, 255u8, 213u8, 133u8, 123u8, 170u8,
    242u8, 34u8, 235u8, 149u8, 167u8, 210u8, 128u8, 157u8, 97u8, 191u8, 224u8, 46u8,
    27u8, 253u8, 27u8, 104u8, 255u8, 2u8, 240u8, 184u8, 16u8, 42u8, 225u8, 194u8, 213u8,
    213u8, 171u8, 26u8, 19u8, 104u8, 187u8, 68u8, 92u8, 124u8, 45u8, 32u8, 151u8, 3u8,
    242u8, 57u8, 104u8, 156u8, 227u8, 76u8, 3u8, 120u8, 166u8, 142u8, 114u8, 166u8,
    179u8, 178u8, 22u8, 218u8, 14u8, 34u8, 165u8, 3u8, 27u8, 84u8, 221u8, 255u8, 87u8,
    48u8, 147u8, 150u8, 179u8, 140u8, 136u8, 28u8, 76u8, 132u8, 158u8, 194u8, 62u8,
    135u8, 25u8, 53u8, 2u8, 184u8, 110u8, 219u8, 136u8, 87u8, 194u8, 115u8, 250u8, 7u8,
    90u8, 80u8, 81u8, 41u8, 55u8, 224u8, 121u8, 78u8, 30u8, 101u8, 167u8, 97u8, 124u8,
    144u8, 216u8, 189u8, 102u8, 6u8, 91u8, 31u8, 255u8, 229u8, 29u8, 122u8, 87u8, 153u8,
    115u8, 177u8, 49u8, 80u8, 33u8, 236u8, 60u8, 25u8, 147u8, 79u8, 17u8, 184u8, 180u8,
    36u8, 205u8, 72u8, 191u8, 56u8, 252u8, 239u8, 104u8, 8u8, 59u8, 11u8, 14u8, 197u8,
    200u8, 26u8, 147u8, 179u8, 48u8, 238u8, 26u8, 103u8, 125u8, 13u8, 21u8, 255u8, 123u8,
    152u8, 78u8, 137u8, 120u8, 239u8, 72u8, 136u8, 30u8, 50u8, 250u8, 201u8, 27u8, 147u8,
    180u8, 115u8, 51u8, 226u8, 186u8, 87u8, 3u8, 53u8, 15u8, 85u8, 167u8, 174u8, 252u8,
    211u8, 195u8, 27u8, 79u8, 203u8, 108u8, 229u8, 119u8, 28u8, 198u8, 160u8, 233u8,
    120u8, 106u8, 181u8, 151u8, 51u8, 32u8, 200u8, 6u8, 173u8, 54u8, 8u8, 41u8, 16u8,
    123u8, 168u8, 16u8, 197u8, 160u8, 159u8, 253u8, 217u8, 190u8, 34u8, 145u8, 160u8,
    194u8, 90u8, 153u8, 162u8, 1u8, 178u8, 245u8, 34u8, 71u8, 61u8, 23u8, 19u8, 145u8,
    18u8, 91u8, 168u8, 77u8, 196u8, 0u8, 124u8, 251u8, 242u8, 248u8, 218u8, 117u8, 47u8,
    124u8, 116u8, 24u8, 82u8, 3u8, 252u8, 202u8, 88u8, 154u8, 199u8, 25u8, 195u8, 77u8,
    255u8, 187u8, 170u8, 216u8, 67u8, 29u8, 173u8, 28u8, 31u8, 181u8, 151u8, 170u8,
    165u8, 1u8, 129u8, 7u8, 21u8, 79u8, 37u8, 167u8, 100u8, 189u8, 60u8, 121u8, 147u8,
    122u8, 69u8, 184u8, 69u8, 70u8, 218u8, 99u8, 75u8, 143u8, 107u8, 225u8, 74u8, 128u8,
    97u8, 229u8, 92u8, 206u8, 186u8, 71u8, 139u8, 35u8, 247u8, 218u8, 202u8, 163u8, 92u8,
    140u8, 167u8, 139u8, 234u8, 233u8, 98u8, 64u8, 69u8, 180u8, 182u8, 4u8, 197u8, 129u8,
    35u8, 77u8, 8u8, 106u8, 153u8, 2u8, 36u8, 155u8, 100u8, 114u8, 143u8, 253u8, 33u8,
    161u8, 137u8, 232u8, 121u8, 53u8, 169u8, 84u8, 5u8, 28u8, 124u8, 219u8, 167u8, 179u8,
    135u8, 38u8, 41u8, 164u8, 250u8, 252u8, 5u8, 6u8, 98u8, 69u8, 203u8, 145u8, 8u8,
    240u8, 36u8, 45u8, 15u8, 227u8, 239u8, 15u8, 65u8, 229u8, 134u8, 99u8, 191u8, 8u8,
    207u8, 6u8, 134u8, 114u8, 203u8, 208u8, 26u8, 126u8, 199u8, 59u8, 172u8, 164u8,
    215u8, 44u8, 169u8, 53u8, 68u8, 222u8, 255u8, 104u8, 107u8, 253u8, 109u8, 245u8,
    67u8, 212u8, 142u8, 170u8, 36u8, 175u8, 228u8, 126u8, 30u8, 253u8, 228u8, 73u8, 56u8,
    59u8, 103u8, 102u8, 49u8,
];
pub const SCALAR_TYPE: u8 = 0u8;
pub const G1_TYPE: u8 = 1u8;
pub const G2_TYPE: u8 = 2u8;
pub const GT_TYPE: u8 = 3u8;
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Scalar {
    pub dummy_field: bool,
}
impl MoveType for Scalar {
    type Package = super::Package;
    const MODULE: &'static str = "bls12381";
    const NAME: &'static str = "Scalar";
}
impl MoveArg for Scalar {
    fn pure_bytes(self) -> PureBytes {
        PureBytes(::bcs::to_bytes(&self).expect("bcs serialization failed"))
    }
}
pub trait ArgumentScalar: PTBArgument {
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
impl ArgumentScalar for Scalar {}
impl ArgumentScalar for Argument {}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct G1 {
    pub dummy_field: bool,
}
impl MoveType for G1 {
    type Package = super::Package;
    const MODULE: &'static str = "bls12381";
    const NAME: &'static str = "G1";
}
impl MoveArg for G1 {
    fn pure_bytes(self) -> PureBytes {
        PureBytes(::bcs::to_bytes(&self).expect("bcs serialization failed"))
    }
}
pub trait ArgumentG1: PTBArgument {
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
impl ArgumentG1 for G1 {}
impl ArgumentG1 for Argument {}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct G2 {
    pub dummy_field: bool,
}
impl MoveType for G2 {
    type Package = super::Package;
    const MODULE: &'static str = "bls12381";
    const NAME: &'static str = "G2";
}
impl MoveArg for G2 {
    fn pure_bytes(self) -> PureBytes {
        PureBytes(::bcs::to_bytes(&self).expect("bcs serialization failed"))
    }
}
pub trait ArgumentG2: PTBArgument {
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
impl ArgumentG2 for G2 {}
impl ArgumentG2 for Argument {}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GT {
    pub dummy_field: bool,
}
impl MoveType for GT {
    type Package = super::Package;
    const MODULE: &'static str = "bls12381";
    const NAME: &'static str = "GT";
}
impl MoveArg for GT {
    fn pure_bytes(self) -> PureBytes {
        PureBytes(::bcs::to_bytes(&self).expect("bcs serialization failed"))
    }
}
pub trait ArgumentGT: PTBArgument {
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
impl ArgumentGT for GT {}
impl ArgumentGT for Argument {}
/// Returns: `bool`.
pub async fn bls12381_min_sig_verify(
    b: &mut PtbBuilder,
    arg0: impl PureVec<u8>,
    arg1: impl PureVec<u8>,
    arg2: impl PureVec<u8>,
) -> Argument {
    let a0 = arg0.into_argument(b).await;
    let a1 = arg1.into_argument(b).await;
    let a2 = arg2.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "bls12381",
        "bls12381_min_sig_verify",
        Vec::new(),
        vec![a0, a1, a2],
    )
}
/// Returns: `bool`.
pub async fn bls12381_min_pk_verify(
    b: &mut PtbBuilder,
    arg0: impl PureVec<u8>,
    arg1: impl PureVec<u8>,
    arg2: impl PureVec<u8>,
) -> Argument {
    let a0 = arg0.into_argument(b).await;
    let a1 = arg1.into_argument(b).await;
    let a2 = arg2.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "bls12381",
        "bls12381_min_pk_verify",
        Vec::new(),
        vec![a0, a1, a2],
    )
}
/// Returns: `0x2::group_ops::Element<0x2::bls12381::Scalar>`.
pub async fn scalar_from_bytes(b: &mut PtbBuilder, arg0: impl PureVec<u8>) -> Argument {
    let a0 = arg0.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "bls12381",
        "scalar_from_bytes",
        Vec::new(),
        vec![a0],
    )
}
/// Returns: `0x2::group_ops::Element<0x2::bls12381::Scalar>`.
pub async fn scalar_from_u64(b: &mut PtbBuilder, arg0: impl PureU64) -> Argument {
    let a0 = arg0.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "bls12381",
        "scalar_from_u64",
        Vec::new(),
        vec![a0],
    )
}
/// Returns: `0x2::group_ops::Element<0x2::bls12381::Scalar>`.
pub async fn scalar_zero(b: &mut PtbBuilder) -> Argument {
    b.move_call(
        b.package_id::<super::Package>(),
        "bls12381",
        "scalar_zero",
        Vec::new(),
        vec![],
    )
}
/// Returns: `0x2::group_ops::Element<0x2::bls12381::Scalar>`.
pub async fn scalar_one(b: &mut PtbBuilder) -> Argument {
    b.move_call(
        b.package_id::<super::Package>(),
        "bls12381",
        "scalar_one",
        Vec::new(),
        vec![],
    )
}
/// Returns: `0x2::group_ops::Element<0x2::bls12381::Scalar>`.
pub async fn scalar_add(
    b: &mut PtbBuilder,
    arg0: impl super::group_ops::ArgumentElement<Scalar>,
    arg1: impl super::group_ops::ArgumentElement<Scalar>,
) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    let a1 = arg1.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "bls12381",
        "scalar_add",
        Vec::new(),
        vec![a0, a1],
    )
}
/// Returns: `0x2::group_ops::Element<0x2::bls12381::Scalar>`.
pub async fn scalar_sub(
    b: &mut PtbBuilder,
    arg0: impl super::group_ops::ArgumentElement<Scalar>,
    arg1: impl super::group_ops::ArgumentElement<Scalar>,
) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    let a1 = arg1.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "bls12381",
        "scalar_sub",
        Vec::new(),
        vec![a0, a1],
    )
}
/// Returns: `0x2::group_ops::Element<0x2::bls12381::Scalar>`.
pub async fn scalar_mul(
    b: &mut PtbBuilder,
    arg0: impl super::group_ops::ArgumentElement<Scalar>,
    arg1: impl super::group_ops::ArgumentElement<Scalar>,
) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    let a1 = arg1.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "bls12381",
        "scalar_mul",
        Vec::new(),
        vec![a0, a1],
    )
}
/// Returns: `0x2::group_ops::Element<0x2::bls12381::Scalar>`.
pub async fn scalar_div(
    b: &mut PtbBuilder,
    arg0: impl super::group_ops::ArgumentElement<Scalar>,
    arg1: impl super::group_ops::ArgumentElement<Scalar>,
) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    let a1 = arg1.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "bls12381",
        "scalar_div",
        Vec::new(),
        vec![a0, a1],
    )
}
/// Returns: `0x2::group_ops::Element<0x2::bls12381::Scalar>`.
pub async fn scalar_neg(
    b: &mut PtbBuilder,
    arg0: impl super::group_ops::ArgumentElement<Scalar>,
) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "bls12381",
        "scalar_neg",
        Vec::new(),
        vec![a0],
    )
}
/// Returns: `0x2::group_ops::Element<0x2::bls12381::Scalar>`.
pub async fn scalar_inv(
    b: &mut PtbBuilder,
    arg0: impl super::group_ops::ArgumentElement<Scalar>,
) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "bls12381",
        "scalar_inv",
        Vec::new(),
        vec![a0],
    )
}
/// Returns: `0x2::group_ops::Element<0x2::bls12381::G1>`.
pub async fn g1_from_bytes(b: &mut PtbBuilder, arg0: impl PureVec<u8>) -> Argument {
    let a0 = arg0.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "bls12381",
        "g1_from_bytes",
        Vec::new(),
        vec![a0],
    )
}
/// Returns: `0x2::group_ops::Element<0x2::bls12381::G1>`.
pub async fn g1_identity(b: &mut PtbBuilder) -> Argument {
    b.move_call(
        b.package_id::<super::Package>(),
        "bls12381",
        "g1_identity",
        Vec::new(),
        vec![],
    )
}
/// Returns: `0x2::group_ops::Element<0x2::bls12381::G1>`.
pub async fn g1_generator(b: &mut PtbBuilder) -> Argument {
    b.move_call(
        b.package_id::<super::Package>(),
        "bls12381",
        "g1_generator",
        Vec::new(),
        vec![],
    )
}
/// Returns: `0x2::group_ops::Element<0x2::bls12381::G1>`.
pub async fn g1_add(
    b: &mut PtbBuilder,
    arg0: impl super::group_ops::ArgumentElement<G1>,
    arg1: impl super::group_ops::ArgumentElement<G1>,
) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    let a1 = arg1.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "bls12381",
        "g1_add",
        Vec::new(),
        vec![a0, a1],
    )
}
/// Returns: `0x2::group_ops::Element<0x2::bls12381::G1>`.
pub async fn g1_sub(
    b: &mut PtbBuilder,
    arg0: impl super::group_ops::ArgumentElement<G1>,
    arg1: impl super::group_ops::ArgumentElement<G1>,
) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    let a1 = arg1.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "bls12381",
        "g1_sub",
        Vec::new(),
        vec![a0, a1],
    )
}
/// Returns: `0x2::group_ops::Element<0x2::bls12381::G1>`.
pub async fn g1_mul(
    b: &mut PtbBuilder,
    arg0: impl super::group_ops::ArgumentElement<Scalar>,
    arg1: impl super::group_ops::ArgumentElement<G1>,
) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    let a1 = arg1.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "bls12381",
        "g1_mul",
        Vec::new(),
        vec![a0, a1],
    )
}
/// Returns: `0x2::group_ops::Element<0x2::bls12381::G1>`.
pub async fn g1_div(
    b: &mut PtbBuilder,
    arg0: impl super::group_ops::ArgumentElement<Scalar>,
    arg1: impl super::group_ops::ArgumentElement<G1>,
) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    let a1 = arg1.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "bls12381",
        "g1_div",
        Vec::new(),
        vec![a0, a1],
    )
}
/// Returns: `0x2::group_ops::Element<0x2::bls12381::G1>`.
pub async fn g1_neg(
    b: &mut PtbBuilder,
    arg0: impl super::group_ops::ArgumentElement<G1>,
) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "bls12381",
        "g1_neg",
        Vec::new(),
        vec![a0],
    )
}
/// Returns: `0x2::group_ops::Element<0x2::bls12381::G1>`.
pub async fn hash_to_g1(b: &mut PtbBuilder, arg0: impl PureVec<u8>) -> Argument {
    let a0 = arg0.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "bls12381",
        "hash_to_g1",
        Vec::new(),
        vec![a0],
    )
}
/// Returns: `0x2::group_ops::Element<0x2::bls12381::G1>`.
pub async fn g1_multi_scalar_multiplication(
    b: &mut PtbBuilder,
    arg0: impl PureVec<super::group_ops::Element<Scalar>>,
    arg1: impl PureVec<super::group_ops::Element<G1>>,
) -> Argument {
    let a0 = arg0.into_argument(b).await;
    let a1 = arg1.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "bls12381",
        "g1_multi_scalar_multiplication",
        Vec::new(),
        vec![a0, a1],
    )
}
/// Returns: `0x2::group_ops::Element<0x2::bls12381::G2>`.
pub async fn g2_from_bytes(b: &mut PtbBuilder, arg0: impl PureVec<u8>) -> Argument {
    let a0 = arg0.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "bls12381",
        "g2_from_bytes",
        Vec::new(),
        vec![a0],
    )
}
/// Returns: `0x2::group_ops::Element<0x2::bls12381::G2>`.
pub async fn g2_identity(b: &mut PtbBuilder) -> Argument {
    b.move_call(
        b.package_id::<super::Package>(),
        "bls12381",
        "g2_identity",
        Vec::new(),
        vec![],
    )
}
/// Returns: `0x2::group_ops::Element<0x2::bls12381::G2>`.
pub async fn g2_generator(b: &mut PtbBuilder) -> Argument {
    b.move_call(
        b.package_id::<super::Package>(),
        "bls12381",
        "g2_generator",
        Vec::new(),
        vec![],
    )
}
/// Returns: `0x2::group_ops::Element<0x2::bls12381::G2>`.
pub async fn g2_add(
    b: &mut PtbBuilder,
    arg0: impl super::group_ops::ArgumentElement<G2>,
    arg1: impl super::group_ops::ArgumentElement<G2>,
) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    let a1 = arg1.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "bls12381",
        "g2_add",
        Vec::new(),
        vec![a0, a1],
    )
}
/// Returns: `0x2::group_ops::Element<0x2::bls12381::G2>`.
pub async fn g2_sub(
    b: &mut PtbBuilder,
    arg0: impl super::group_ops::ArgumentElement<G2>,
    arg1: impl super::group_ops::ArgumentElement<G2>,
) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    let a1 = arg1.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "bls12381",
        "g2_sub",
        Vec::new(),
        vec![a0, a1],
    )
}
/// Returns: `0x2::group_ops::Element<0x2::bls12381::G2>`.
pub async fn g2_mul(
    b: &mut PtbBuilder,
    arg0: impl super::group_ops::ArgumentElement<Scalar>,
    arg1: impl super::group_ops::ArgumentElement<G2>,
) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    let a1 = arg1.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "bls12381",
        "g2_mul",
        Vec::new(),
        vec![a0, a1],
    )
}
/// Returns: `0x2::group_ops::Element<0x2::bls12381::G2>`.
pub async fn g2_div(
    b: &mut PtbBuilder,
    arg0: impl super::group_ops::ArgumentElement<Scalar>,
    arg1: impl super::group_ops::ArgumentElement<G2>,
) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    let a1 = arg1.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "bls12381",
        "g2_div",
        Vec::new(),
        vec![a0, a1],
    )
}
/// Returns: `0x2::group_ops::Element<0x2::bls12381::G2>`.
pub async fn g2_neg(
    b: &mut PtbBuilder,
    arg0: impl super::group_ops::ArgumentElement<G2>,
) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "bls12381",
        "g2_neg",
        Vec::new(),
        vec![a0],
    )
}
/// Returns: `0x2::group_ops::Element<0x2::bls12381::G2>`.
pub async fn hash_to_g2(b: &mut PtbBuilder, arg0: impl PureVec<u8>) -> Argument {
    let a0 = arg0.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "bls12381",
        "hash_to_g2",
        Vec::new(),
        vec![a0],
    )
}
/// Returns: `0x2::group_ops::Element<0x2::bls12381::G2>`.
pub async fn g2_multi_scalar_multiplication(
    b: &mut PtbBuilder,
    arg0: impl PureVec<super::group_ops::Element<Scalar>>,
    arg1: impl PureVec<super::group_ops::Element<G2>>,
) -> Argument {
    let a0 = arg0.into_argument(b).await;
    let a1 = arg1.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "bls12381",
        "g2_multi_scalar_multiplication",
        Vec::new(),
        vec![a0, a1],
    )
}
/// Returns: `0x2::group_ops::Element<0x2::bls12381::GT>`.
pub async fn gt_identity(b: &mut PtbBuilder) -> Argument {
    b.move_call(
        b.package_id::<super::Package>(),
        "bls12381",
        "gt_identity",
        Vec::new(),
        vec![],
    )
}
/// Returns: `0x2::group_ops::Element<0x2::bls12381::GT>`.
pub async fn gt_generator(b: &mut PtbBuilder) -> Argument {
    b.move_call(
        b.package_id::<super::Package>(),
        "bls12381",
        "gt_generator",
        Vec::new(),
        vec![],
    )
}
/// Returns: `0x2::group_ops::Element<0x2::bls12381::GT>`.
pub async fn gt_add(
    b: &mut PtbBuilder,
    arg0: impl super::group_ops::ArgumentElement<GT>,
    arg1: impl super::group_ops::ArgumentElement<GT>,
) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    let a1 = arg1.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "bls12381",
        "gt_add",
        Vec::new(),
        vec![a0, a1],
    )
}
/// Returns: `0x2::group_ops::Element<0x2::bls12381::GT>`.
pub async fn gt_sub(
    b: &mut PtbBuilder,
    arg0: impl super::group_ops::ArgumentElement<GT>,
    arg1: impl super::group_ops::ArgumentElement<GT>,
) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    let a1 = arg1.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "bls12381",
        "gt_sub",
        Vec::new(),
        vec![a0, a1],
    )
}
/// Returns: `0x2::group_ops::Element<0x2::bls12381::GT>`.
pub async fn gt_mul(
    b: &mut PtbBuilder,
    arg0: impl super::group_ops::ArgumentElement<Scalar>,
    arg1: impl super::group_ops::ArgumentElement<GT>,
) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    let a1 = arg1.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "bls12381",
        "gt_mul",
        Vec::new(),
        vec![a0, a1],
    )
}
/// Returns: `0x2::group_ops::Element<0x2::bls12381::GT>`.
pub async fn gt_div(
    b: &mut PtbBuilder,
    arg0: impl super::group_ops::ArgumentElement<Scalar>,
    arg1: impl super::group_ops::ArgumentElement<GT>,
) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    let a1 = arg1.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "bls12381",
        "gt_div",
        Vec::new(),
        vec![a0, a1],
    )
}
/// Returns: `0x2::group_ops::Element<0x2::bls12381::GT>`.
pub async fn gt_neg(
    b: &mut PtbBuilder,
    arg0: impl super::group_ops::ArgumentElement<GT>,
) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "bls12381",
        "gt_neg",
        Vec::new(),
        vec![a0],
    )
}
/// Returns: `0x2::group_ops::Element<0x2::bls12381::GT>`.
pub async fn pairing(
    b: &mut PtbBuilder,
    arg0: impl super::group_ops::ArgumentElement<G1>,
    arg1: impl super::group_ops::ArgumentElement<G2>,
) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    let a1 = arg1.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "bls12381",
        "pairing",
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
    pub fn scalar_tag(&self) -> TypeTag {
        <Scalar as MoveType>::type_tag_at(self.package)
    }
    pub fn g1_tag(&self) -> TypeTag {
        <G1 as MoveType>::type_tag_at(self.package)
    }
    pub fn g2_tag(&self) -> TypeTag {
        <G2 as MoveType>::type_tag_at(self.package)
    }
    pub fn gt_tag(&self) -> TypeTag {
        <GT as MoveType>::type_tag_at(self.package)
    }
}
