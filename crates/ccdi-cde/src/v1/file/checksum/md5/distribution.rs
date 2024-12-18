use rand::distributions::Distribution;
use rand::Rng;

pub struct Hexadecimal;

const GEN_HEX_CHARSET: &[u8] = b"ABCDEF0123456789";

impl Distribution<u8> for Hexadecimal {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> u8 {
        GEN_HEX_CHARSET[rng.next_u32() as usize % GEN_HEX_CHARSET.len()]
    }
}
