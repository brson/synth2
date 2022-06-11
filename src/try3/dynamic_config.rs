use super::units::*;
use super::static_config::{
    NUM_LAYERS,
};

pub struct Voice {
    pub layers: [Layer; NUM_LAYERS],
}

pub struct Layer {
    pub osc: Oscillator,
    pub lpf: LowPassFilter,
    pub amp_env: Adsr,
    pub mod_env: Adsr,
}

pub enum Oscillator {
    Square,
    Saw,
    Triangle,
}

pub struct LowPassFilter {
    pub freq: Hz,
    pub sample_rate: SampleRateKhz,
}

pub struct Adsr {
    pub attack: SampleOffset,
    pub decay: SampleOffset,
    pub sustain: Unipolar<1>,
    pub release: SampleOffset,
}
