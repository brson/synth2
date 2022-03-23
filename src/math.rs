use anyhow::Result;
use std::ops::Div;
use std::cmp::{Ord, PartialOrd, Eq, PartialEq, Ordering};

/// Positive i32 (signed natural)
pub struct Snat32(i32);

impl Snat32 {
}

impl From<Snat32> for i32 {
    fn from(other: Snat32) -> i32 {
        other.0
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

impl PartialEq for Snat32 {
    fn eq(&self, other: &Snat32) -> bool {
        self.0.eq(&other.0)
    }
}

impl Eq for Snat32 { }

impl PartialOrd for Snat32 {
    fn partial_cmp(&self, other: &Snat32) -> Option<Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl Ord for Snat32 {
    fn cmp(&self, other: &Snat32) -> Ordering {
        self.0.cmp(&other.0)
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
