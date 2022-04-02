#![allow(unused)]

mod i16;
mod f64;
mod math;
mod seq;

use anyhow::Result;

fn main() -> Result<()> {
    i16::run()?;
    f64::run()?;

    Ok(())
}
