use std::simd::{f32x16, u32x16, SimdPartialOrd, StdFloat};
use super::math::*;
use super::units::*;
use super::lookup;
use std::ops::BitXor;

const SEED32: u32 = 0x9e_37_79_b9;

/// A noise generator based on fxhash.
pub struct HashNoise {
    pub seed: u32,
}

impl HashNoise {
    pub fn sample(&self, offset: SampleOffset) -> Bipolar<1> {
        let offset = offset.0 as u32;
        let hash = self.seed;
        let hash = hash_word(hash, offset);
        let value = hash as u16;
        let value = value as f32;
        let u16_max = u16::max_value() as f32;
        // todo: lift this * 2.0 out to an integer op
        let sample = value / u16_max * 2.0 - 1.0;
        Bipolar(sample)
    }
}

pub struct HashNoiseX16 {
    pub seed: u32,
}

impl HashNoiseX16 {
    pub fn sample(&self, offset: [SampleOffset; 16]) -> [Bipolar<1>; 16] {
        let offset = offset.map(|o| o.0);
        let offset = f32x16::from_array(offset);
        let offset = offset.cast::<u32>();
        let offset = offset.to_array();
        let hash = [self.seed; 16];
        let hash = hash_word_x16(hash, offset);
        let hash = u32x16::from_array(hash);
        let value = hash.cast::<u16>();
        let value = value.cast::<f32>();
        let u16_max = f32x16::splat(u16::max_value() as f32);
        let two = f32x16::splat(2.0);
        let one = f32x16::splat(1.0);
        let sample = value / u16_max * two - one;
        let sample = sample.to_array();
        sample.map(|s| Bipolar(s))
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
    ((rotated ^ word) * seed32).to_array()
}

#[test]
fn test_hash_word() {
    let start = u32::max_value();
    let word = u32::max_value();
    let start = 0xFF00FF00;
    let word = 0x11111111;
    let hash = hash_word(start, word);

    let start_x16 = [start; 16];
    let word_x16 = [word; 16];
    let hash_x16 = hash_word_x16(start_x16, word_x16);

    assert_eq!(hash, hash_x16[0]);
}

#[test]
fn test_hash_word_dist() {
    // Testing that hash_word produces ~50% 1s.
    // It happens to be exactly true for this number of times through the loop.
    let count = 200004;

    let mut ones = 0;
    for i in 0..count {
        let hash = hash_word(0, i);
        ones += hash.count_ones();
    }

    assert_eq!(ones, count * 16);
}
