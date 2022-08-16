pub use super::oscillators::phase_accumulating::{
    OscillatorState,
};
pub use super::filters::{
    LowPassFilterState,
};

#[derive(Default)]
#[derive(Copy, Clone)]
pub struct Layer {
    pub osc: OscillatorState,
    pub noise: NoiseState,
    pub lpf: LowPassFilterState,
}

#[derive(Default)]
#[derive(Copy, Clone)]
pub struct NoiseState {
    // todo don't default this
    pub seed: u32,
}
