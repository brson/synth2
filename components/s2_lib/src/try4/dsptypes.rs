use std::simd::{f32x16, u32x16, StdFloat, SimdFloat, Simd};

#[derive(Copy, Clone)]
pub struct Bipolar<const N: u16>(pub f32);

#[derive(Copy, Clone)]
pub struct Unipolar<const N: u16>(pub f32);

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

use std::simd::{LaneCount, SupportedLaneCount};
use std::ops::{Div, Mul};

pub trait DspType: Sized
    + Div<Output = Self>
    + Mul<Output = Self>
{ }

impl DspType for f32 { }
impl DspType for f32x16 { }

pub fn line_y_value<F>(
    y_rise: F,
    x_run: F,
    x_value: F,
) -> F
where F: DspType
{
    let slope = y_rise / x_run;
    let y_value = slope * x_value;
    y_value
}

fn assertions() {
    let y_rise = 0.0;
    let x_run = 0.0;
    let x_value = 0.0;
    line_y_value(y_rise, x_run, x_value);

    let y_rise = f32x16::splat(y_rise);
    let x_run = f32x16::splat(x_run);
    let x_value = f32x16::splat(x_value);
    line_y_value(y_rise, x_run, x_value);
}
