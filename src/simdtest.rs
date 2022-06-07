use core_simd::{f32x4, u8x4};
use core_simd::{SimdFloat};

pub struct OscillatorX4 {
    kind: u8x4,
    period: f32x4,
}

const OSC_KIND_SQUARE: u8 = 1;
const OSC_KIND_SAW: u8 = 2;

impl OscillatorX4 {
    pub fn sample(&self, offset: u32) -> f32x4 {
        let offset = offset as f32;
        let offset = f32x4::from([offset; 4]);
        panic!()
    }
}

pub fn line_y_value(
    y_rise: f32x4,
    x_run: f32x4,
    x_value: f32x4,
) -> f32x4 {
    let slope = y_rise / x_run;
    let y_value = slope * x_value;
    y_value
}

pub fn line_y_value_with_y_offset(
    y_rise: f32x4,
    x_run: f32x4,
    x_value: f32x4,
    y_offset: f32x4,
) -> f32x4 {
    let y_value = line_y_value(y_rise, x_run, x_value);
    y_value + y_offset
}
