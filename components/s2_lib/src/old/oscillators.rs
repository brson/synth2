use super::math::*;
use anyhow::{anyhow, Result};

pub enum Oscillator {
    Square(SquareOscillator),
    Saw(SawOscillator),
    Triangle(TriangleOscillator),
    Sine(SineOscillator),
    Noise(NoiseOscillator),
}

pub struct SquareOscillator {}

pub struct SawOscillator {}

pub struct TriangleOscillator {}

pub struct SineOscillator {}

pub struct NoiseOscillator {
    pub seed: u32,
}

impl Oscillator {
    pub fn sample(&self, sample_rate: SampleRateKhz, freq: Hz64, offset: u32) -> One64 {
        match self {
            Oscillator::Square(osc) => osc.sample(sample_rate, freq, offset),
            Oscillator::Triangle(osc) => osc.sample(sample_rate, freq, offset),
            Oscillator::Saw(osc) => osc.sample(sample_rate, freq, offset),
            Oscillator::Sine(osc) => osc.sample(sample_rate, freq, offset),
            Oscillator::Noise(osc) => osc.sample(offset),
        }
    }
}

impl SquareOscillator {
    pub fn sample(&self, sample_rate: SampleRateKhz, freq: Hz64, offset: u32) -> One64 {
        let period = freq.as_samples(sample_rate);
        let period = f64::from(period);
        let offset = f64::from(offset);
        let offset = offset % period;

        let half_period = period / 2.0;
        let sample = if offset < half_period { 1.0 } else { -1.0 };

        One64::assert_from(sample)
    }
}

impl SawOscillator {
    pub fn sample(&self, sample_rate: SampleRateKhz, freq: Hz64, offset: u32) -> One64 {
        let period = freq.as_samples(sample_rate);
        let period = f64::from(period);
        let offset = f64::from(offset);
        let offset = offset % period;

        let x_rise = -2.0;
        let x_run = period;
        let x_value = offset;
        let y_offset = 1.0;

        let sample = line_y_value_with_y_offset(x_rise, x_run, x_value, y_offset);

        One64::assert_from(sample)
    }
}

impl TriangleOscillator {
    pub fn sample(&self, sample_rate: SampleRateKhz, freq: Hz64, offset: u32) -> One64 {
        let period = freq.as_samples(sample_rate);
        let period = f64::from(period);
        let offset = f64::from(offset);
        let offset = offset % period;

        let half_period = period / 2.0;
        let sample = if offset < half_period {
            let x_rise = -2.0;
            let x_run = half_period;
            let x_value = offset;
            let y_offset = 1.0;

            line_y_value_with_y_offset(x_rise, x_run, x_value, y_offset)
        } else {
            let x_rise = 2.0;
            let x_run = half_period;
            let x_value = offset - half_period;
            let y_offset = -1.0;

            line_y_value_with_y_offset(x_rise, x_run, x_value, y_offset)
        };

        One64::assert_from(sample)
    }
}

impl SineOscillator {
    pub fn sample(&self, sample_rate: SampleRateKhz, freq: Hz64, offset: u32) -> One64 {
        let period = freq.as_samples(sample_rate);
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

        use rand::Rng;
        use std::hash::Hasher;

        let mut hasher = fxhash::FxHasher::default();
        hasher.write_u32(self.seed);
        hasher.write_u32(offset);
        let rand_u64 = hasher.finish();
        let mut rng = rand_pcg::Pcg64Mcg::new(rand_u64.into());
        let rand_f64: f64 = rng.gen_range(-1.0..=1.0);
        One64::assert_from(rand_f64)
    }
}
