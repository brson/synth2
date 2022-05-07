use anyhow::{Result, anyhow};
use crate::math::*;

pub const SAMPLE_RATE_KHZ: u32 = 32_000;

pub fn saw_osc_hz(freq: Hz64) -> OscillatorHz {
    OscillatorHz {
        sample_rate: SampleRateKhz(SAMPLE_RATE_KHZ),
        freq,
        triangleness: ZOne64::assert_from(0_f64),
        squareness: ZOne64::assert_from(0_f64),
        sineness: ZOne64::assert_from(0_f64),
    }
}

pub fn triangle_osc_hz(freq: Hz64) -> OscillatorHz {
    OscillatorHz {
        sample_rate: SampleRateKhz(SAMPLE_RATE_KHZ),
        freq,
        triangleness: ZOne64::assert_from(1_f64),
        squareness: ZOne64::assert_from(0_f64),
        sineness: ZOne64::assert_from(0_f64),
    }
}

pub fn square_osc_hz(freq: Hz64) -> OscillatorHz {
    OscillatorHz {
        sample_rate: SampleRateKhz(SAMPLE_RATE_KHZ),
        freq,
        triangleness: ZOne64::assert_from(0_f64),
        squareness: ZOne64::assert_from(1_f64),
        sineness: ZOne64::assert_from(0_f64),
    }
}

pub fn funky_osc_hz(freq: Hz64) -> OscillatorHz {
    OscillatorHz {
        sample_rate: SampleRateKhz(SAMPLE_RATE_KHZ),
        freq,
        triangleness: ZOne64::assert_from(0.25_f64),
        squareness: ZOne64::assert_from(0.25_f64),
        sineness: ZOne64::assert_from(0_f64),
    }
}

pub fn sin_osc_hz(freq: Hz64) -> OscillatorHz {
    OscillatorHz {
        sample_rate: SampleRateKhz(SAMPLE_RATE_KHZ),
        freq,
        triangleness: ZOne64::assert_from(0_f64),
        squareness: ZOne64::assert_from(0_f64),
        sineness: ZOne64::assert_from(1_f64),
    }
}

pub fn round_saw_osc_hz(freq: Hz64) -> OscillatorHz {
    OscillatorHz {
        sample_rate: SampleRateKhz(SAMPLE_RATE_KHZ),
        freq,
        triangleness: ZOne64::assert_from(0_f64),
        squareness: ZOne64::assert_from(0_f64),
        sineness: ZOne64::assert_from(0.5_f64),
    }
}

pub fn round_triangle_osc_hz(freq: Hz64) -> OscillatorHz {
    OscillatorHz {
        sample_rate: SampleRateKhz(SAMPLE_RATE_KHZ),
        freq,
        triangleness: ZOne64::assert_from(1_f64),
        squareness: ZOne64::assert_from(0_f64),
        sineness: ZOne64::assert_from(0.5_f64),
    }
}

pub fn round_square_osc_hz(freq: Hz64) -> OscillatorHz {
    OscillatorHz {
        sample_rate: SampleRateKhz(SAMPLE_RATE_KHZ),
        freq,
        triangleness: ZOne64::assert_from(0_f64),
        squareness: ZOne64::assert_from(1_f64),
        sineness: ZOne64::assert_from(0.5_f64),
    }
}

pub fn round_funky_osc_hz(freq: Hz64) -> OscillatorHz {
    OscillatorHz {
        sample_rate: SampleRateKhz(SAMPLE_RATE_KHZ),
        freq,
        triangleness: ZOne64::assert_from(0.25_f64),
        squareness: ZOne64::assert_from(0.25_f64),
        sineness: ZOne64::assert_from(0.5_f64),
    }
}

pub struct OscillatorHz {
    pub sample_rate: SampleRateKhz,
    pub freq: Hz64,
    pub triangleness: ZOne64, // 0 for sawtooth, 1 for triangle
    pub squareness: ZOne64, // 0 for saw/tri, 1 for square
    pub sineness: ZOne64, // 0 for saw/tri/square, 1 for sine
}

impl OscillatorHz {
    pub fn sample(&self, offset: u32) -> One64 {
        let period = self.freq.as_samples(self.sample_rate);
        let angle_osc = AngleOscillator {
            period,
            triangleness: self.triangleness,
            squareness: self.squareness,
        };
        let sin_osc = SinOscillator {
            period,
        };
        let angle_sample = f64::from(angle_osc.sample(offset));
        let sin_sample = f64::from(sin_osc.sample(offset));

        // Cross-fade between the angle_osc and sin_osc
        let sample = {
            let diff_range = sin_sample - angle_sample;
            let angle_offset = diff_range * f64::from(self.sineness);
            let sample = angle_sample + angle_offset;
            sample
        };

        // Give it some curvature
        let sample = {
            let pow = 2;
            let curved_sample = if sample > 0.0 {
                sample.powi(pow)
            } else {
                -sample.powi(pow)
            };

            // Cross-fade
            let curviness = 0.2;
            let diff_range = curved_sample - sample;
            let sample_offset = diff_range * curviness;
            let sample = sample + sample_offset;
            sample
        };

        One64::assert_from(sample)
    }
}

struct SinOscillator {
    pub period: u32,
}

impl SinOscillator {
    pub fn sample(&self, offset: u32) -> One64 {
        let period = f64::from(self.period);
        let offset = f64::from(offset);
        let offset = offset % period;
        let pi = std::f64::consts::PI;
        let sin_offset = offset * pi * 2.0 / period;
        let sample = sin_offset.sin();
        One64::assert_from(sample)
    }
}

struct AngleOscillator {
    pub period: u32,
    pub triangleness: ZOne64, // 0 for sawtooth, 1 for triangle
    pub squareness: ZOne64, // 0 for saw/tri, 1 for square
}

enum AngleOscillatorStage {
    InitialRise,
    InitialFall,
    FinalFall,
    FinalRise,
}

impl AngleOscillator {
    pub fn sample(&self, offset: u32) -> One64 {
        let offset: f64 = offset.into();
        let period: f64 = self.period.into();
        let triangleness: f64 = self.triangleness.into();
        let squareness: f64 = self.squareness.into();

        let osc_offset = offset % period;
        let half_period = period / 2.0;

        let rise_time = half_period * triangleness;
        let half_rise_time = rise_time / 2.0;
        let fall_time = period - rise_time;
        let half_fall_time = fall_time / 2.0;

        let stage = {
            let in_initial_rise = osc_offset < half_rise_time;
            let in_final_rise = osc_offset >= period - half_rise_time;
            let in_fall = !in_initial_rise && !in_final_rise;
            let in_first_half_fall = in_fall && osc_offset < half_period;
            let in_second_half_fall = in_fall && !in_first_half_fall;

            if in_initial_rise {
                AngleOscillatorStage::InitialRise
            } else if in_first_half_fall {
                AngleOscillatorStage::InitialFall
            } else if in_second_half_fall {
                AngleOscillatorStage::FinalFall
            } else if in_final_rise {
                AngleOscillatorStage::FinalRise
            } else {
                unreachable!()
            }
        };

        match stage {
            AngleOscillatorStage::InitialRise => {
                let rise = 1.0;
                let run = half_rise_time;
                let x_offset = osc_offset;
                let y_start = 0.0;
                let sample = line_y_value_with_y_offset(
                    rise, run, x_offset, y_start
                );
                One64::assert_from(sample)
            }
            AngleOscillatorStage::InitialFall => {
                let rise = -(1.0 - squareness);
                let run = half_fall_time;
                let x_offset = osc_offset - half_rise_time;
                let y_start = 1.0;
                let sample = line_y_value_with_y_offset(
                    rise, run, x_offset, y_start
                );
                One64::assert_from(sample)
            }
            AngleOscillatorStage::FinalFall => {
                let rise = -(1.0 - squareness);
                let run = half_fall_time;
                let x_offset = osc_offset - half_period;
                let y_start = -squareness;
                let sample = line_y_value_with_y_offset(
                    rise, run, x_offset, y_start
                );
                One64::assert_from(sample)
            }
            AngleOscillatorStage::FinalRise => {
                let rise = 1.0;
                let run = half_rise_time;
                let x_offset = osc_offset - half_rise_time - fall_time;
                let y_start = -1.0;
                let sample = line_y_value_with_y_offset(
                    rise, run, x_offset, y_start
                );
                One64::assert_from(sample)
            }
        }
    }
}

pub struct Noise {
    pub seed: u32,
}

impl Noise {
    pub fn sample(&self, offset: u32) -> One64 {
        // Use a hash function to generate a pseudo-random rng seed
        // based on the noise seed and the offset,
        // then an rng to generate a sample with the correct distribution.
        
        //let mut hasher = blake3::Hasher::new();
        //let seed_buf = &self.seed.to_le_bytes()[..];
        //let offset_buf = &i32::from(offset).to_le_bytes()[..];
        //hasher.update(seed_buf);
        //hasher.update(offset_buf);
        //let hash_bytes = hasher.finalize().as_bytes();
        //let mut rand_buf = [0; 16];
        //rand_buf.copy_from_slice(hash_bytes);
        //let rand_u128 = u128::from_le_bytes(rand_buf);
        //let mut rng = rand_pcg::Pcg64Mcg::new(rand_u64);

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

// https://www.musicdsp.org/en/latest/Filters/237-one-pole-filter-lp-and-hp.html
#[derive(Debug)]
pub struct LowPassFilter {
    pub freq: ZPos64,
    sample_rate: u32,
    a0: f64,
    b1: f64,
    last: f64,
}

impl LowPassFilter {
    pub fn new(freq: ZPos64, sample_rate: u32) -> LowPassFilter {
        let x = (-2.0 * std::f64::consts::PI * f64::from(freq) / f64::from(sample_rate)).exp();
        LowPassFilter {
            freq,
            sample_rate,
            a0: 1.0 - x,
            b1: -x,
            last: 0.0,
        }
    }

    pub fn process(&mut self, input: f64) -> f64 {
        let out = self.a0 * input - self.b1 * self.last;
        self.last = out;
        out
    }

    pub fn modulate(&mut self, freq: ZPos64) -> ModulatedLowPassFilter<'_> {
        ModulatedLowPassFilter {
            parent: self,
            freq,
        }
    }
}

pub struct ModulatedLowPassFilter<'this> {
    parent: &'this mut LowPassFilter,
    freq: ZPos64,
}

impl<'this> ModulatedLowPassFilter<'this> {
    pub fn process(&mut self, input: f64) -> f64 {
        let mut child_lpf = LowPassFilter::new(self.freq, self.parent.sample_rate);
        child_lpf.last = self.parent.last;
        let sample = child_lpf.process(input);
        self.parent.last = child_lpf.last;
        sample
    }
}


