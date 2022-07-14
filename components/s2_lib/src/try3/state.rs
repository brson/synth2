use soa_derive::StructOfArray;

pub use super::oscillators::phase_accumulating::OscillatorState;

#[derive(StructOfArray, Default)]
#[derive(Copy, Clone)]
pub struct Layer {
    pub osc: OscillatorState,
    #[nested_soa]
    pub lpf: LowPassFilter,
}

#[derive(StructOfArray, Default)]
#[derive(Copy, Clone)]
pub struct LowPassFilter {
    pub last: f32,
}
