#![feature(portable_simd)]
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

mod try3 {
    mod static_config;
    //mod dynamic_config;
    //mod state;
}

mod simdtest;

use anyhow::Result;

fn main() -> Result<()> {
    env_logger::init();

    //threads::run()?;

    Ok(())
}
