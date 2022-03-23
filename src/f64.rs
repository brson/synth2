use anyhow::Result;
use std::path::{Path, PathBuf};
use crate::math::{Snat32, Zone64, One64};

fn line_y_value(
    y_rise: f64,
    x_run: f64,
    x_value: f64,
) -> f64 {
    let slope = y_rise / x_run;
    let y_value = slope * x_value;
    y_value
}

fn line_y_value_with_y_offset(
    y_rise: f64,
    x_run: f64,
    x_value: f64,
    y_offset: f64,
) -> f64 {
    let y_value = line_y_value(y_rise, x_run, x_value);
    y_value + y_offset
}

const SAMPLE_RATE_KHZ: i32 = 32_000;
const A440_SAMPLES: i32 = SAMPLE_RATE_KHZ / 440;

struct Oscillator {
    period: Snat32,
    phase: i32,
    rise_time: Snat32, // 0 for sawtooth, period / 2 for triangle
    squareness: Zone64, // 0 for saw/tri, 1 for square
    pulse_width: Snat32,
}

impl Oscillator {
    fn sample(&self, offset: Snat32) -> One64 {
        //assert!(self.rise_time <= self.period / Snat32::from(2));
        todo!()
    }
}
