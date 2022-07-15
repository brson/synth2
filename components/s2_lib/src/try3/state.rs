use soa_derive::StructOfArray;

pub use super::oscillators::phase_accumulating::{
    OscillatorState,
    OscillatorStateSlice,
    OscillatorStateSliceMut,
    OscillatorStatePtr,
    OscillatorStatePtrMut,
    OscillatorStateRef,
    OscillatorStateRefMut,
};

#[derive(StructOfArray)]
#[derive(Default)]
#[derive(Copy, Clone)]
pub struct Layer {
    #[nested_soa]
    pub osc: OscillatorState,
    #[nested_soa]
    pub lpf: LowPassFilter,
}

#[derive(StructOfArray)]
#[derive(Default)]
#[derive(Copy, Clone)]
pub struct LowPassFilter {
    pub last: f32,
}
