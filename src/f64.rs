use anyhow::Result;
use std::path::{Path, PathBuf};
use crate::math::{Snat32, Zone64, One64, AssertFrom};

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

enum OscillatorStage {
    InitialRise,
    InitialFall,
    FinalFall,
    FinalRise,
}
   

impl Oscillator {
    fn sample(&self, offset: Snat32) -> One64 {
        assert!(self.rise_time <= self.period / Snat32::assert_from(2));

        let offset: f64 = offset.into();
        let period: f64 = self.period.into();
        let rise_time: f64 = self.rise_time.into();

        let osc_offset = offset % period;
        let half_period = period / 2.0;
        let half_rise_time = rise_time / 2.0;
        let fall_time = period - rise_time;
        let half_fall_time = fall_time / 2.0;

        let stage = {
            let in_initial_rise = osc_offset < half_rise_time;
            let in_final_rise = osc_offset > period - half_rise_time;
            let in_fall = !in_initial_rise && !in_final_rise;
            let in_first_half_fall = in_fall && osc_offset < half_period;
            let in_second_half_fall = in_fall && !in_first_half_fall;
            if in_initial_rise {
                OscillatorStage::InitialRise
            } else if in_first_half_fall {
                OscillatorStage::InitialFall
            } else if in_second_half_fall {
                OscillatorStage::FinalFall
            } else if in_final_rise {
                OscillatorStage::FinalRise
            } else {
                unreachable!()
            }
        };
        

        todo!()
    }
}
