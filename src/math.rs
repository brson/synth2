use anyhow::Result;
use std::ops::Div;
use std::cmp::{Ord, PartialOrd, Eq, PartialEq, Ordering};
use std::fmt::Debug;

pub fn line_y_value(
    y_rise: f64,
    x_run: f64,
    x_value: f64,
) -> f64 {
    let slope = y_rise / x_run;
    let y_value = slope * x_value;
    y_value
}

pub fn line_y_value_with_y_offset(
    y_rise: f64,
    x_run: f64,
    x_value: f64,
    y_offset: f64,
) -> f64 {
    let y_value = line_y_value(y_rise, x_run, x_value);
    y_value + y_offset
}

pub trait AssertFrom<From>: Sized {
    fn assert_from(value: From) -> Self;
}

impl<T, From> AssertFrom<From> for T
where T: TryFrom<From>,
      <T as TryFrom<From>>::Error: Debug
{
    fn assert_from(value: From) -> Self {
        Self::try_from(value).expect("try from")
    }
}

/// F64 between zero and one
#[derive(Copy, Clone)]
pub struct ZOne64(f64);

impl From<ZOne64> for f64 {
    fn from(other: ZOne64) -> f64 {
        other.0
    }
}

impl TryFrom<f64> for ZOne64 {
    type Error = anyhow::Error;

    fn try_from(other: f64) -> Result<ZOne64> {
        if other >= 0.0 && other <= 1.0 {
            Ok(ZOne64(other))
        } else {
            Err(anyhow::anyhow!("float out of [0, 1] range"))
        }
    }
}

/// F64 between negative one and one
#[derive(Copy, Clone)]
pub struct One64(f64);

impl From<One64> for f64 {
    fn from(other: One64) -> f64 {
        other.0
    }
}

impl TryFrom<f64> for One64 {
    type Error = anyhow::Error;

    fn try_from(other: f64) -> Result<One64> {
        if other >= -1.0 && other <= 1.0 {
            Ok(One64(other))
        } else {
            Err(anyhow::anyhow!("float out of [-1, 1] range"))
        }
    }
}

/// Positive or zero float
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct ZPos64(f64);

impl From<ZPos64> for f64 {
    fn from(other: ZPos64) -> f64 {
        other.0
    }
}

impl TryFrom<f64> for ZPos64 {
    type Error = anyhow::Error;

    fn try_from(other: f64) -> Result<ZPos64> {
        if other >= 0.0 {
            Ok(ZPos64(other))
        } else {
            Err(anyhow::anyhow!("float out of [0, inf] range"))
        }
    }
}

/// Positive float
#[derive(Copy, Clone)]
pub struct Pos64(f64);

impl From<Pos64> for f64 {
    fn from(other: Pos64) -> f64 {
        other.0
    }
}

impl TryFrom<f64> for Pos64 {
    type Error = anyhow::Error;

    fn try_from(other: f64) -> Result<Pos64> {
        if other > 0.0 {
            Ok(Pos64(other))
        } else {
            Err(anyhow::anyhow!("float out of (0, inf] range"))
        }
    }
}

/// Sample rate in khz
#[derive(Copy, Clone)]
pub struct SampleRateKhz(pub u32);

/// Time in ms
#[derive(Copy, Clone)]
pub struct Ms64(ZPos64);

impl Ms64 {
    /// Get the time as samples
    pub fn as_samples(&self, sample_rate: SampleRateKhz) -> u32 {
        let sample_rate = f64::from(sample_rate.0);
        let ms = f64::from(self.0);
        let seconds = ms / 1000.0;
        let samples: f64 = sample_rate * seconds;
        samples as u32
    }
}    

impl TryFrom<f64> for Ms64 {
    type Error = anyhow::Error;

    fn try_from(other: f64) -> Result<Ms64> {
        Ok(Ms64(ZPos64::try_from(other)?))
    }
}

/// Positive frequency
#[derive(Copy, Clone)]
pub struct Hz64(Pos64);

impl Hz64 {
    pub fn as_samples(&self, sample_rate: SampleRateKhz) -> u32 {
        let sample_rate = f64::from(sample_rate.0);
        let hz = f64::from(self.0);
        let period: f64 = sample_rate / hz;
        period as u32
    }
}

impl From<Hz64> for f64 {
    fn from(other: Hz64) -> f64 {
        other.0.0
    }
}

impl TryFrom<f64> for Hz64 {
    type Error = anyhow::Error;

    fn try_from(other: f64) -> Result<Hz64> {
        Ok(Hz64(Pos64::try_from(other)?))
    }
}

