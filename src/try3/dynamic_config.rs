use soa_derive::StructOfArray;

use super::units::*;

#[derive(StructOfArray)]
pub struct Layer {
    #[nested_soa]
    pub osc: Oscillator,
    #[nested_soa]
    pub lpf: LowPassFilter,
    #[nested_soa]
    pub amp_env: Adsr,
}

#[derive(StructOfArray)]
pub struct Oscillator {
    pub period: SampleOffset,
    pub kind: OscillatorKind,
}

pub enum OscillatorKind {
    Square,
    Saw,
    Triangle,
}

#[derive(StructOfArray)]
pub struct LowPassFilter {
    pub freq: Hz,
    pub sample_rate: SampleRateKhz,
}

#[derive(StructOfArray)]
pub struct Adsr {
    pub attack: SampleOffset,
    pub decay: SampleOffset,
    pub sustain: Unipolar<1>,
    pub release: SampleOffset,
}
