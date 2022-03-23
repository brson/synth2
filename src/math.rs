use anyhow::Result;

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
