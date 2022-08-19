use std::simd::{f32x16, u32x16, StdFloat, SimdFloat, Simd, SimdPartialOrd};
use std::simd::{LaneCount, SupportedLaneCount};
use std::ops::{Div, Mul, Add, Sub};

#[derive(Copy, Clone)]
#[derive(Debug)]
#[derive(PartialEq, PartialOrd)]
pub struct SampleOffset(pub f32);

#[derive(Copy, Clone)]
#[derive(Debug)]
#[derive(PartialEq, PartialOrd)]
pub struct Unipolar<const N: u16>(pub f32);

#[derive(Copy, Clone)]
#[derive(Debug)]
#[derive(PartialEq, PartialOrd)]
pub struct Bipolar<const N: u16>(pub f32);

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
    + Mul<Output = Self>
    + Div<Output = Self>
    + DspFloat
{ }

impl DspType for f32 { }
impl DspType for f32x16 { }

pub trait DspLike<T, D>
where Self: Sized + Copy + Clone,
      D: DspType,
{
    fn to_dsp(self) -> D;

    fn from_dsp(other: D) -> Self {
        let s = Self::from_dsp_unchecked(other);
        s.debug_validate();
        s
    }

    fn from_dsp_unchecked(other: D) -> Self;

    fn validate(&self);

    fn debug_validate(&self) {
        if cfg!(debug) {
            self.validate();
        }
    }
}

impl DspLike<SampleOffset, f32> for SampleOffset {
    fn to_dsp(self) -> f32 {
        self.0
    }

    fn from_dsp_unchecked(other: f32) -> Self {
        Self(other)
    }

    fn validate(&self) {
        assert!(self.0 >= -1.0 && self.0 <= 1.0);
    }
}

impl DspLike<SampleOffset, f32x16> for [SampleOffset; 16] {
    fn to_dsp(self) -> f32x16 {
        f32x16::from_array(self.map(|s| s.0))
    }

    fn from_dsp_unchecked(other: f32x16) -> Self {
        other.to_array().map(|s| SampleOffset(s))
    }

    fn validate(&self) {
        assert!(
            (self.to_dsp().simd_ge(f32x16::splat(-1.0))
             & self.to_dsp().simd_le(f32x16::splat(1.0))
            ).all()
        );
    }
}

impl<const N: u16> DspLike<Unipolar<N>, f32> for Unipolar<N> {
    fn to_dsp(self) -> f32 {
        self.0
    }

    fn from_dsp_unchecked(other: f32) -> Self {
        Self(other)
    }

    fn validate(&self) {
        assert!(self.0 >= -(N as f32) && self.0 <= (N as f32));
    }
}

impl<const N: u16> DspLike<Unipolar<N>, f32x16> for [Unipolar<N>; 16] {
    fn to_dsp(self) -> f32x16 {
        f32x16::from_array(self.map(|s| s.0))
    }

    fn from_dsp_unchecked(other: f32x16) -> Self {
        other.to_array().map(|s| Unipolar(s))
    }

    fn validate(&self) {
        assert!(
            (self.to_dsp().simd_ge(f32x16::splat(-(N as f32)))
             & self.to_dsp().simd_le(f32x16::splat(N as f32))
            ).all()
        );
    }
}

fn phased_offset<Dsp, Return>(
    period: impl DspLike<SampleOffset, Dsp>,
    phase: impl DspLike<Unipolar<1>, Dsp>,
    offset: impl DspLike<SampleOffset, Dsp>,
) -> Return
where Dsp: DspType,
      Return: DspLike<SampleOffset, Dsp>,
{
    let period = period.to_dsp();
    let phase = phase.to_dsp();
    let offset = offset.to_dsp();

    if !cfg!(feature = "fma") {
        let phase_offset = period * phase;
        Return::from_dsp(offset + phase_offset)
    } else {
        let offset = period.mul_add(phase, offset);
        Return::from_dsp(offset)
    }
}

#[test]
fn phased_offset_assertions() {
    let period = SampleOffset(0.0);
    let phase = Unipolar(0.0);
    let offset = SampleOffset(0.0);
    let r: SampleOffset = phased_offset(period, phase, offset);
    assert_eq!(r, SampleOffset(0.0));
    let period = [period; 16];
    let phase = [phase; 16];
    let offset = [offset; 16];
    let r: [SampleOffset; 16] = phased_offset(period, phase, offset);
    assert_eq!(r, [SampleOffset(0.0); 16]);
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
