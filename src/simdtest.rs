use core_simd::{f32x4, u8x4};

pub struct OscillatorX4 {
    kind: u8x4,
    period: f32x4,
}

const OSC_KIND_SQUARE: u8 = 1;
const OSC_KIND_SAW: u8 = 2;

impl OscillatorX4 {
    pub fn sample(&self, offset: u32) -> f32x4 {
        todo!()
    }
}
