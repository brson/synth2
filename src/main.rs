#![allow(unused)]

mod i16;
mod f64;
mod math;

use anyhow::Result;

fn main() -> Result<()> {
    i16::run()?;

    Ok(())
}
