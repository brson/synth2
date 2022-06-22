use anyhow::{Result, anyhow};
use crate::math::*;

pub struct LowPassFilter {
    last: f64,
}

impl LowPassFilter {
    pub fn new() -> LowPassFilter {
        LowPassFilter {
            last: 0.0,
        }
    }

    // https://www.musicdsp.org/en/latest/Filters/237-one-pole-filter-lp-and-hp.html
    pub fn process(
        &mut self,
        sample_rate: SampleRateKhz,
        freq: Hz64,
        input: f64
    ) -> f64 {
        let sample_rate = f64::from(sample_rate.0);
        let freq = f64::from(freq);
        let pi = std::f64::consts::PI;
        let x = (-2.0 * pi * freq / sample_rate).exp();

        let a0 = 1.0 - x;
        let b1 = -x;

        let out = a0 * input - b1 * self.last;
        self.last = out;
        out
    }
}
