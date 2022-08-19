use std::simd::{f32x16, u32x16, StdFloat, SimdFloat, Simd, SimdPartialOrd, SimdPartialEq};
use std::simd::{mask32x16, Mask};
use std::simd::{LaneCount, SupportedLaneCount};
use std::ops::{Div, Mul, Add, Sub, Rem};
use std::marker::PhantomData;

#[allow(non_camel_case_types)]
pub type f32x1 = Simd<f32, 1>;

#[allow(non_camel_case_types)]
pub type mask32x1 = Mask<i32, 1>;

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

pub trait DspFloat: Copy {
    fn splat(v: f32) -> Self;
}

impl DspFloat for f32x1 {
    fn splat(v: f32) -> Self {
        Simd::splat(v)
    }
}

impl DspFloat for f32x16 {
    fn splat(v: f32) -> Self {
        Simd::splat(v)
    }
}

pub trait DspMask<DspFloat>: Sized
{
    fn select(
        self,
        true_values: DspFloat,
        false_values: DspFloat,
    ) -> DspFloat;
}

impl DspMask<f32x1> for mask32x1 {
    fn select(
        self,
        true_values: f32x1,
        false_values: f32x1,
    ) -> f32x1 {
        Mask::select(self, true_values, false_values)
    }
}

impl DspMask<f32x16> for mask32x16 {
    fn select(
        self,
        true_values: f32x16,
        false_values: f32x16,
    ) -> f32x16 {
        Mask::select(self, true_values, false_values)
    }
}

pub trait DspType: Sized
    + Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
    + Rem<Output = Self>
    + SimdPartialOrd
    + SimdPartialEq<Mask = Self::DspMask>
    + StdFloat
    + DspFloat
{
    type DspMask: DspMask<Self>;
}

impl DspType for f32x1 {
    type DspMask = mask32x1;
}

impl DspType for f32x16 {
    type DspMask = mask32x16;
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

#[test]
fn assertions() {
    let y_rise = f32x1::splat(0.0);
    let x_run = f32x1::splat(0.0);
    let x_value = f32x1::splat(0.0);
    line_y_value(y_rise, x_run, x_value);

    let y_rise = f32x16::splat(0.0);
    let x_run = f32x16::splat(0.0);
    let x_value = f32x16::splat(0.0);
    line_y_value(y_rise, x_run, x_value);
}




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

impl DspLike<SampleOffset, f32x1> for SampleOffset {
    fn to_dsp(self) -> f32x1 {
        f32x1::splat(self.0)
    }

    fn from_dsp_unchecked(other: f32x1) -> Self {
        Self(other.to_array()[0])
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

impl<const N: u16> DspLike<Unipolar<N>, f32x1> for Unipolar<N> {
    fn to_dsp(self) -> f32x1 {
        f32x1::splat(self.0)
    }

    fn from_dsp_unchecked(other: f32x1) -> Self {
        Self(other.to_array()[0])
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

pub struct SquareOscillator<Dsp, LikeSampleOffset>
where Dsp: DspType,
      LikeSampleOffset: DspLike<SampleOffset, Dsp>,
{
    pub dsp: PhantomData<Dsp>,
    pub period: LikeSampleOffset,
}

impl<Dsp, LikeSampleOffset> SquareOscillator<Dsp, LikeSampleOffset>
where Dsp: DspType,
      LikeSampleOffset: DspLike<SampleOffset, Dsp>,
{
    pub fn sample<Return>(&self, offset: LikeSampleOffset) -> Return
        where Return: DspLike<Bipolar<1>, Dsp>
    {
        let period = self.period.to_dsp();
        let offset = offset.to_dsp();
        let offset = offset % period;

        let two = Dsp::splat(2.0);
        let half_period = period / two;
        let offset_lt_half_period = offset.simd_lt(half_period);
        let one = Dsp::splat(1.0);
        let n_one = Dsp::splat(-1.0);
        let sample = offset_lt_half_period.select(one, n_one);

        Return::from_dsp(sample)
    }
}

