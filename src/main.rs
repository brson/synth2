#![allow(unused)]

mod i16;
mod f64;
mod math;
mod seq;
mod threads;

use anyhow::Result;

fn main() -> Result<()> {
    env_logger::init();

    //i16::run()?;
    //f64::run()?;

    threads::run()?;

    Ok(())
}
