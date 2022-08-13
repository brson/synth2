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

/// Remainder, for positive numbers.
pub fn fast_fmodf(a: f32, b: f32) -> f32 {
    debug_assert!(a >= 0.0);
    debug_assert!(b > 0.0);
    if cfg!(feature = "fma") {
        a - (((a / b) as u32 as f32) * b)
    } else {
        -((a / b) as u32 as f32).mul_add(b, -a)
    }
}

pub fn fast_fmodf_x16(a: [f32; 16], b: [f32; 16]) -> [f32; 16] {
    todo!()
}

mod test {
    use super::*;

    fn fuzzy_eq(a: f32, b: f32) -> bool {
        let epsilon = 0.000001;
        a > b - epsilon && a < b + epsilon
    }
    
    #[test]
    fn test_fast_fmodf() {
        assert!(fuzzy_eq(fast_fmodf(0.0, 10.0), 0.0));
        assert!(fuzzy_eq(fast_fmodf(5.0, 10.0), 5.0));
        assert!(fuzzy_eq(fast_fmodf(15.0, 10.0), 5.0));
        assert!(fuzzy_eq(fast_fmodf(10.0, 10.0), 0.0));
        assert!(fuzzy_eq(fast_fmodf(10.1, 10.0), 0.1));
        assert!(fuzzy_eq(fast_fmodf(10.01, 10.0), 0.01));
        assert!(fuzzy_eq(fast_fmodf(10.001, 10.0), 0.001));
        assert!(fuzzy_eq(fast_fmodf(1001.0, 1000.0), 1.0));
        assert!(fuzzy_eq(fast_fmodf(1000001.0, 10000.0), 1.0));
        //assert!(fuzzy_eq(fast_fmodf(100000010.0, 10000.0), 10.0));
    }
}
