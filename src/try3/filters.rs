use super::units::*;
use super::state as st;

pub struct LowPassFilter<'this> {
    pub state: &'this mut st::LowPassFilter,
    pub sample_rate: SampleRateKhz,
    pub freq: Hz,
}

impl<'this> LowPassFilter<'this> {
    pub fn process(&mut self, input: f32) -> f32 {
        let sample_rate = self.sample_rate.0 as f32;
        let freq = self.freq.0;

        let pi = std::f32::consts::PI;
        let x = (-2.0 * pi * freq / sample_rate).exp();

        let a0 = 1.0 - x;
        let b1 = -x;

        let out = a0 * input - b1 * self.state.last;
        self.state.last = out;
        out
    }
}
