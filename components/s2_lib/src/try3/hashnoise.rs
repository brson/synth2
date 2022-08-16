//! A noise generator based on fxhash.

pub mod basic {
    use std::simd::{f32x16, u32x16, SimdPartialOrd, StdFloat};
    use super::super::math::*;
    use super::super::units::*;
    use super::super::lookup;
    use std::ops::BitXor;

    const ROTATE: u32 = 5;
    const SEED32: u32 = 0x9e_37_79_b9;

    pub struct HashNoiseOscillator {
        pub seed: u32,
    }

    impl HashNoiseOscillator {
        pub fn sample(&self, offset: SampleOffset) -> Bipolar<1> {
            let hash = self.seed;
            let hash = hash_word(hash, offset.0 as u32);
            let value = hash as u16;
            let value = value as f32;
            let u16_max = u16::max_value() as f32;
            let sample = value / u16_max * 2.0 - 1.0;
            Bipolar(sample)
        }
    }

    pub struct HashNoiseOscillatorX16 {
        pub seed: [u32; 16],
    }

    impl HashNoiseOscillatorX16 {
        pub fn sample(&self, offset: [SampleOffset; 16]) -> [Bipolar<1>; 16] {
            todo!()
        }
    }
    
    fn hash_word(start: u32, word: u32) -> u32 {
        start.rotate_left(5).bitxor(word).wrapping_mul(SEED32)
    }

    fn hash_word_x16(
        start: [u32; 16],
        word: [u32; 16],
    ) -> [u32; 16] {
        let start = u32x16::from_array(start);
        let word = u32x16::from_array(word);
        let seed32 = u32x16::splat(SEED32);
        let leftshift = start << u32x16::splat(5);
        let rightshift = start >> u32x16::splat(32 - 5);
        let rotated = leftshift | rightshift;
        ((rotated | word) * seed32).to_array()
    }
}
