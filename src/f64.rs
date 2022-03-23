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
        let squareness: f64 = self.squareness.into();

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

        match stage {
            OscillatorStage::InitialRise => {
                let rise = 1.0;
                let run = half_rise_time;
                let x_offset = osc_offset;
                let y_start = 0.0;
                let sample = line_y_value_with_y_offset(
                    rise, run, x_offset, y_start
                );
                One64::assert_from(sample)
            }
            OscillatorStage::InitialFall => {
                let rise = -(1.0 - squareness);
                let run = half_fall_time;
                let x_offset = osc_offset - half_rise_time;
                let y_start = 1.0;
                let sample = line_y_value_with_y_offset(
                    rise, run, x_offset, y_start
                );
                One64::assert_from(sample)
            }
            OscillatorStage::FinalFall => {
                let rise = -(1.0 - squareness);
                let run = half_fall_time;
                let x_offset = osc_offset - half_period;
                let y_start = -squareness;
                let sample = line_y_value_with_y_offset(
                    rise, run, x_offset, y_start
                );
                One64::assert_from(sample)
            }
            OscillatorStage::FinalRise => {
                let rise = 1.0;
                let run = half_rise_time;
                let x_offset = osc_offset - half_rise_time - fall_time;
                let y_start = -1.0;
                let sample = line_y_value_with_y_offset(
                    rise, run, x_offset, y_start
                );
                One64::assert_from(sample)
            }
        }
    }
}

struct Adsr {
    attack: Snat32,
    decay: Snat32,
    sustain: Zone64,
    release: Snat32,
}

enum AdsrStage {
    Attack,
    Decay,
    Sustain,
    Release,
    End,
}

impl Adsr {
    fn sample(&self, offset: Snat32, release_offset: Option<Snat32>) -> Zone64 {
        let attack: f64 = self.attack.into();
        let decay: f64 = self.decay.into();
        let sustain: f64 = self.sustain.into();
        let release: f64 = self.release.into();

        let offset: f64 = offset.into();
        let decay_offset = attack;
        let sustain_offset = attack + decay;
        let release_offset = f64::from(
            release_offset
                .map(i32::from)
                .unwrap_or(i32::MAX)
        ).max(sustain_offset);
        let end_offset = release_offset + release;

        let stage = {
            let in_attack = offset < decay_offset;
            let in_decay = !in_attack && offset < sustain_offset;
            let in_sustain = !in_attack && !in_decay && offset < release_offset;
            let in_release = !in_attack && !in_decay && !in_sustain && offset < end_offset;

            if in_attack {
                AdsrStage::Attack
            } else if in_decay {
                AdsrStage::Decay
            } else if in_sustain {
                AdsrStage::Sustain
            } else if in_release {
                AdsrStage::Release
            } else {
                AdsrStage::End
            }
        };

        match stage {
            AdsrStage::Attack => {
                let rise = 1.0;
                let run = attack;
                let x_offset = offset;
                let y_start = 0.0;
                let sample = line_y_value_with_y_offset(
                    rise, run, x_offset, y_start
                );
                Zone64::assert_from(sample)
            }
            AdsrStage::Decay => {
                let rise = sustain - 1.0;
                let run = decay;
                let x_offset = offset - decay_offset;
                let y_start = 1.0;
                let sample = line_y_value_with_y_offset(
                    rise, run, x_offset, y_start
                );
                Zone64::assert_from(sample)
            }
            AdsrStage::Sustain => {
                Zone64::assert_from(sustain)
            }
            AdsrStage::Release => {
                let rise = -sustain;
                let run = release;
                let x_offset = offset - release_offset;
                let y_start = sustain;
                let sample = line_y_value_with_y_offset(
                    rise, run, x_offset, y_start
                );
                Zone64::assert_from(sample)
            }
            AdsrStage::End => {
                Zone64::assert_from(0.0)
            }
        }
    }
}
