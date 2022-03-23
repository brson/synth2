#![allow(unused)]

mod i16;

use anyhow::Result;

fn main() -> Result<()> {
    i16::run()?;

    Ok(())
}
