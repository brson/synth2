use super::units::*;

#[derive(Default)]
#[derive(Copy, Clone)]
pub struct LowPassFilterState {
    pub last: f32,
}

pub struct LowPassFilter<'this> {
    pub state: &'this mut LowPassFilterState,
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

        let out = if !cfg!(feature = "fma") {
            a0 * input - b1 * self.state.last
        } else {
            // todo this has an extra subtraction. is in faster?
            a0.mul_add(input, -b1 * self.state.last)
        };
        self.state.last = out;
        out
    }
}
