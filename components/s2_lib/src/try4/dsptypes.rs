use std::simd::{f32x16, u32x16, StdFloat, SimdFloat, Simd, SimdPartialOrd};
use std::simd::{LaneCount, SupportedLaneCount};
use std::ops::{Div, Mul, Add, Sub};

#[derive(Copy, Clone)]
pub struct SampleOffset(pub f32);

#[derive(Copy, Clone)]
pub struct Bipolar<const N: u16>(pub f32);

#[derive(Copy, Clone)]
pub struct Unipolar<const N: u16>(pub f32);

pub trait DspFloat {
    fn mul_add(self, a: Self, b: Self) -> Self;
}

impl DspFloat for f32 {
    fn mul_add(self, a: Self, b: Self) -> Self {
        f32::mul_add(self, a, b)
    }
}

impl DspFloat for f32x16 {
    fn mul_add(self, a: Self, b: Self) -> Self {
        StdFloat::mul_add(self, a, b)
    }
}

pub trait DspType: Sized
    + Add<Output = Self>
    + Sub<Output = Self>
    + Div<Output = Self>
    + Mul<Output = Self>
    + DspFloat
{ }

impl DspType for f32 { }
impl DspType for f32x16 { }

pub trait DspTypeWrapper: Sized {
    type Dsp: DspType;

    fn to_dsp(&self) -> Self::Dsp;

    fn from_dsp(value: Self::Dsp) -> Self;

    fn validate(&self);

    fn debug_validate(&self) {
        if cfg!(debug) {
            self.validate();
        }
    }
}

impl DspTypeWrapper for SampleOffset {
    type Dsp = f32;

    fn to_dsp(&self) -> Self::Dsp {
        self.0
    }

    fn from_dsp(value: Self::Dsp) -> Self {
        Self(value)
    }

    fn validate(&self) {
        assert!(self.0 >= -1.0 && self.0 <= 1.0);
    }
}

impl DspTypeWrapper for [SampleOffset; 16] {
    type Dsp = f32x16;

    fn to_dsp(&self) -> Self::Dsp {
        f32x16::from_array(self.map(|s| s.0))
    }

    fn from_dsp(value: Self::Dsp) -> Self {
        value.to_array().map(|s| SampleOffset(s))
    }

    fn validate(&self) {
        assert!(
            (self.to_dsp().simd_ge(f32x16::splat(-1.0))
             & self.to_dsp().simd_le(f32x16::splat(1.0))
            ).all()
        );
    }
}

impl<const N: u16> DspTypeWrapper for Unipolar<N> {
    type Dsp = f32;

    fn to_dsp(&self) -> Self::Dsp {
        self.0
    }

    fn from_dsp(value: Self::Dsp) -> Self {
        Self(value)
    }

    fn validate(&self) {
        assert!(self.0 >= -1.0 && self.0 <= 1.0);
    }
}

impl<const N: u16> DspTypeWrapper for [Unipolar<N>; 16] {
    type Dsp = f32x16;

    fn to_dsp(&self) -> Self::Dsp {
        f32x16::from_array(self.map(|s| s.0))
    }

    fn from_dsp(value: Self::Dsp) -> Self {
        value.to_array().map(|s| Unipolar(s))
    }

    fn validate(&self) {
        assert!(
            (self.to_dsp().simd_ge(f32x16::splat(-1.0))
             & self.to_dsp().simd_le(f32x16::splat(1.0))
            ).all()
        );
    }
}

pub fn line_y_value<D>(
    y_rise: D,
    x_run: D,
    x_value: D,
) -> D
where D: DspType
{
    let slope = y_rise / x_run;
    let y_value = slope * x_value;
    y_value
}

pub fn line_y_value_with_y_offset<D>(
    y_rise: D,
    x_run: D,
    x_value: D,
    y_offset: D,
) -> D
where D: DspType
{
    if !cfg!(feature = "fma") {
        let y_value = line_y_value(y_rise, x_run, x_value);
        y_value + y_offset
    } else {
        let slope = y_rise / x_run;
        slope.mul_add(x_value, y_offset)
    }
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
