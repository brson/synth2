use anyhow::Result;
use std::ops::Div;
use std::cmp::{Ord, PartialOrd, Eq, PartialEq, Ordering};
use std::fmt::Debug;

/// Positive i32 (signed natural)
#[derive(Copy, Clone)]
#[derive(Eq, PartialEq, Ord, PartialOrd)]
pub struct Snat32(i32);

impl Snat32 {
}

impl From<Snat32> for i32 {
    fn from(other: Snat32) -> i32 {
        other.0
    }
}

impl From<Snat32> for f64 {
    fn from(other: Snat32) -> f64 {
        other.0.into()
    }
}

impl TryFrom<i32> for Snat32 {
    type Error = anyhow::Error;

    fn try_from(other: i32) -> Result<Snat32> {
        if other >= 0 {
            Ok(Snat32(other))
        } else {
            Err(anyhow::anyhow!("negative value"))
        }
    }
}

impl Div for Snat32 {
    type Output = Snat32;

    fn div(self, other: Snat32) -> Snat32 {
        Snat32(self.0 / other.0)
    }
}

/// F64 between zero and one
pub struct Zone64(f64);

impl Zone64 {
}

impl From<Zone64> for f64 {
    fn from(other: Zone64) -> f64 {
        other.0
    }
}

impl TryFrom<f64> for Zone64 {
    type Error = anyhow::Error;

    fn try_from(other: f64) -> Result<Zone64> {
        if other >= 0.0 && other <= 1.0 {
            Ok(Zone64(other))
        } else {
            Err(anyhow::anyhow!("float out of [0, 1] range"))
        }
    }
}

/// F64 between negative one and one
pub struct One64(f64);

impl One64 {
}

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
