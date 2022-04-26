use anyhow::{Result, anyhow};
use crate::math::*;

pub enum Oscillator {
    Square(SquareOscillator),
    Triangle(TriangleOscillator),
    Saw(SawOscillator),
    Sine(SineOscillator),
    Noise(NoiseOscillator),
}

pub struct SquareOscillator {
    pub sample_rate: SampleRateKhz,
    pub freq: Hz64,
    pub period: u32,
}

pub struct TriangleOscillator {
    pub sample_rate: SampleRateKhz,
    pub freq: Hz64,
    pub period: u32,
}

pub struct SawOscillator {
    pub sample_rate: SampleRateKhz,
    pub freq: Hz64,
    pub period: u32,
}

pub struct SineOscillator {
    pub sample_rate: SampleRateKhz,
    pub freq: Hz64,
    pub period: u32,
}

pub struct NoiseOscillator {
    pub seed: u32,
}

impl Oscillator {
    pub fn sample(&self, offset: u32) -> One64 {
        match self {
            Oscillator::Square(osc) => osc.sample(offset),
            Oscillator::Triangle(osc) => osc.sample(offset),
            Oscillator::Saw(osc) => osc.sample(offset),
            Oscillator::Sine(osc) => osc.sample(offset),
            Oscillator::Noise(osc) => osc.sample(offset),
        }
    }
}

impl SquareOscillator {
    pub fn sample(&self, offset: u32) -> One64 {
        todo!()
    }
}

impl TriangleOscillator {
    pub fn sample(&self, offset: u32) -> One64 {
        todo!()
    }
}

impl SawOscillator {
    pub fn sample(&self, offset: u32) -> One64 {
        todo!()
    }
}

impl SineOscillator {
    pub fn sample(&self, offset: u32) -> One64 {
        let period = self.freq.as_samples(self.sample_rate);
        let period = f64::from(period);
        let offset = f64::from(offset);
        let offset = offset % period;
        let pi = std::f64::consts::PI;
        let sin_offset = offset * pi * 2.0 / period;
        let sample = sin_offset.sin();
        One64::assert_from(sample)
    }
}

impl NoiseOscillator {
    pub fn sample(&self, offset: u32) -> One64 {
        // Use a hash function to generate a pseudo-random rng seed
        // based on the noise seed and the offset,
        // then an rng to generate a sample with the correct distribution.

        use std::hash::Hasher;
        use rand::Rng;

        let mut hasher = fxhash::FxHasher::default();
        hasher.write_u32(self.seed);
        hasher.write_u32(offset);
        let rand_u64 = hasher.finish();
        let mut rng = rand_pcg::Pcg64Mcg::new(rand_u64.into());
        let rand_f64: f64 = rng.gen_range(-1.0..=1.0);
        One64::assert_from(rand_f64)
    }
}

