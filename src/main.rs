#![allow(unused)]

mod f64;
mod math;
mod oscillators;
mod seq;
mod synth;
mod threads;

use anyhow::Result;

fn main() -> Result<()> {
    env_logger::init();

    f64::run()?;

    //threads::run()?;

    Ok(())
}
