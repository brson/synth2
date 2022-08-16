use super::units::*;

#[derive(Copy, Clone)]
pub struct Layer {
    pub osc: Oscillator,
    pub noise: Unipolar<1>,
    pub lpf: LowPassFilter,
    pub gain: Unipolar<1>,
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
    Sine,
}

#[derive(Copy, Clone)]
pub struct LowPassFilter {
    pub freq: Hz,
    pub sample_rate: SampleRateKhz,
}

#[derive(Copy, Clone)]
pub struct LayerX<const N: usize> {
    pub osc: OscillatorX<N>,
    pub noise: Unipolar<1>,
    pub lpf: LowPassFilterX<N>,
    pub gains: [Unipolar<1>; N],
}

#[derive(Copy, Clone)]
pub struct OscillatorX<const N: usize> {
    pub kind: OscillatorKind,
    pub periods: [SampleOffset; N],
}

#[derive(Copy, Clone)]
pub struct LowPassFilterX<const N: usize> {
    pub sample_rate: SampleRateKhz,
    pub freqs: [Hz; N],
}

