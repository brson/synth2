use super::units::*;

#[derive(Copy, Clone)]
pub struct Layer {
    pub osc: Oscillator,
    pub noise: Unipolar<1>,
    pub lpf: LowPassFilter,
    pub amp_env: Adsr,
    pub mod_env: Adsr,
    pub modulations: Modulations,
}

#[derive(Copy, Clone)]
pub struct Modulations {
    pub mod_env_to_osc_freq: Bipolar<10>,
    pub mod_env_to_lpf_freq: Bipolar<10>,
}

#[derive(Copy, Clone)]
pub struct Oscillator {
    pub kind: OscillatorKind,
    pub gain: Unipolar<1>,
}

#[derive(Copy, Clone)]
pub enum OscillatorKind {
    Square,
    Saw,
    Triangle,
    Sine,
}

#[derive(Copy, Clone)]
pub struct LowPassFilter {
    pub freq: Hz,
}

#[derive(Copy, Clone)]
pub struct Adsr {
    pub attack: Ms,
    pub decay: Ms,
    pub sustain: Unipolar<1>,
    pub release: Ms,
}
