#![allow(unused)]

mod f64;
mod envelopes;
mod filters;
mod math;
mod oscillators;
mod plotting;
mod seq;
mod synth;
mod synth2;
mod threads;

use anyhow::Result;

fn main() -> Result<()> {
    env_logger::init();

    //threads::run()?;

    Ok(())
}
