use anyhow::Result;

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
