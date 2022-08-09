use super::units::*;

#[derive(Copy, Clone)]
pub struct Layer {
    pub osc: Oscillator,
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
    pub period: SampleOffset,
    pub kind: OscillatorKind,
}

#[derive(Copy, Clone)]
pub enum OscillatorKind {
    Square,
    Saw,
    Triangle,
}

#[derive(Copy, Clone)]
pub struct LowPassFilter {
    pub freq: Hz,
    pub sample_rate: SampleRateKhz,
}

#[derive(Copy, Clone)]
pub struct Adsr {
    pub attack: SampleOffset,
    pub decay: SampleOffset,
    pub sustain: Unipolar<1>,
    pub release: SampleOffset,
}
