use std::simd::{f32x16, u32x16, StdFloat, SimdFloat};
use super::units::{Unipolar, Bipolar};

pub fn line_y_value(y_rise: f32, x_run: f32, x_value: f32) -> f32 {
    let slope = y_rise / x_run;
    let y_value = slope * x_value;
    y_value
}

pub fn line_y_value_with_y_offset(y_rise: f32, x_run: f32, x_value: f32, y_offset: f32) -> f32 {
    if !cfg!(feature = "fma") {
        let y_value = line_y_value(y_rise, x_run, x_value);
        y_value + y_offset
    } else {
        let slope = y_rise / x_run;
        slope.mul_add(x_value, y_offset)
    }
}

pub fn line_y_value_x16(y_rise: f32x16, x_run: f32x16, x_value: f32x16) -> f32x16 {
    let slope = y_rise / x_run;
    let y_value = slope * x_value;
    y_value
}

pub fn line_y_value_with_y_offset_x16(
    y_rise: f32x16,
    x_run: f32x16,
    x_value: f32x16,
    y_offset: f32x16,
) -> f32x16 {
    if !cfg!(feature = "fma") {
        let y_value = line_y_value_x16(y_rise, x_run, x_value);
        y_value + y_offset
    } else {
        let slope = y_rise / x_run;
        slope.mul_add(x_value, y_offset)
    }
}

pub const fn indexes_u32<const N: usize>() -> [u32; N] {
    let mut indexes = [0; N];
    let mut index = 0;
    while index < N {
        indexes[index] = index as u32;
        index += 1;
    }
    indexes
}

pub fn zip3<A, B, C, const N: usize>(a: [A; N], b: [B; N], c: [C; N]) -> [(A, B, C); N] {
    let a_b = a.zip(b);
    let a_b_c = a_b.zip(c);
    let a_b_c = a_b_c.map(|((a, b), c)| (a, b, c));
    a_b_c
}
