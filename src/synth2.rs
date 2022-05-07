use crate::math::*;
use crate::oscillators::Oscillator;
use crate::envelopes::Adsr;
use crate::filters::LowPassFilter;

pub struct Synth2 {
    pub partial1: Partial,
    pub partial2: Partial,
}

pub struct Partial {
    pub osc: Oscillator,
    pub lpf: LowPassFilter,
    pub amp_env: Adsr,
    pub mod_env: Adsr,
    pub pitch_mod_freq_multiplier: f64, // 1 = no mod, 2 = 1 octave
    pub lpf_mod_freq_multiplier: f64, // 1 = no mod, 2 = 1 octave
}

impl Partial {
}
