#![allow(unused)]

use anyhow::Result;
use std::path::{Path, PathBuf};

const SAMPLE_RATE_KHZ: i32 = 32_000;
const A440_SAMPLES: i32 = SAMPLE_RATE_KHZ / 440;

fn saw_osc() -> Oscillator {
    Oscillator {
        amplitude: i16::MAX,
        period: A440_SAMPLES,
        phase: 0,
        rise_time: 0,
        squareness: 0,
        pulse_width: A440_SAMPLES / 2,
    }
}

fn triangle_osc() -> Oscillator {
    Oscillator {
        amplitude: i16::MAX,
        period: A440_SAMPLES,
        phase: 0,
        rise_time: A440_SAMPLES / 2,
        squareness: 0,
        pulse_width: A440_SAMPLES / 2,
    }
}

fn square_osc() -> Oscillator {
    Oscillator {
        amplitude: i16::MAX,
        period: A440_SAMPLES,
        phase: 0,
        rise_time: 0,
        squareness: i16::MAX,
        pulse_width: A440_SAMPLES / 2,
    }
}

fn funky_square_osc() -> Oscillator {
    Oscillator {
        amplitude: i16::MAX,
        period: A440_SAMPLES,
        phase: 0,
        rise_time: A440_SAMPLES / 8,
        squareness: i16::MAX - i16::MAX / 4,
        pulse_width: A440_SAMPLES / 2,
    }
}

// 16-bit, 32 khz
struct Oscillator {
    amplitude: i16,
    period: i32,
    phase: i32,
    rise_time: i32, // 0 for sawtooth, period / 2 for triangle
    squareness: i16, // 0 for saw/tri, amplitude for square
    pulse_width: i32,
}

// rise_time = 0 = sawtooth
//
//  |\
//  | \
// ----------------------------
//      \ |
//       \|

// rise_time = period / 2 = triangle
//
//   /\
//  /  \
// ----------------------------
//       \  /
//        \/


impl Oscillator {
    fn get_sample(&self, offset: u32) -> i16 {
        assert!(self.period >= 0);
        assert!(self.phase >= 0);
        assert!(self.rise_time >= 0);
        assert!(self.pulse_width >= 0);
        
        assert!(self.rise_time <= self.period / 2);
        if self.amplitude > 0 {
            assert!(self.squareness <= self.amplitude);
            assert!(self.squareness >= 0);
        } else {
            todo!();
        }

        let offset: i32 = offset.try_into().expect("overflow");
        let osc_offset = offset % self.period;
        let half_period = self.period / 2;
        let half_rise_time = self.rise_time / 2;
        let in_initial_rise = osc_offset < half_rise_time;
        let in_final_rise = osc_offset > self.period - half_rise_time;
        let in_fall = !in_initial_rise && !in_final_rise;
        let in_first_half_fall = in_fall && osc_offset < half_period;
        let in_second_half_fall = in_fall && !in_first_half_fall;
        let fall_time = self.period - self.rise_time;
        let half_fall_time = fall_time / 2;

        let amplitude_i32: i32 = self.amplitude.into();
        let squareness_i32: i32 = self.squareness.into();

        if in_initial_rise {
            // delta = amplitude / half_rise_time
            // sample = delta * osc_offset
            let sample = amplitude_i32.saturating_mul(osc_offset) / half_rise_time;
            clamp_i32_to_i16(sample)
        } else if in_first_half_fall {
            let working_offset = osc_offset - half_rise_time;
            // let delta = (amplitude_i32 - squareness_i32) / half_fall_time;
            let sample = amplitude_i32
                - (amplitude_i32 - squareness_i32).saturating_mul(working_offset) / half_fall_time;
            clamp_i32_to_i16(sample)
        } else if in_second_half_fall {
            let working_offset = osc_offset - half_period;
            let starting_amplitude = -squareness_i32;
            let sample = starting_amplitude
                - (amplitude_i32 - squareness_i32).saturating_mul(working_offset) / half_fall_time;
            clamp_i32_to_i16(sample)
        } else if in_final_rise {
            let working_offset = osc_offset - half_rise_time - fall_time;
            let sample = amplitude_i32.saturating_mul(working_offset) / half_rise_time - amplitude_i32;
            clamp_i32_to_i16(sample)
        } else {
            unreachable!()
        }       
    }
}

// FIXME: use more precise math to avoid the need for this
fn clamp_i32_to_i16(v: i32) -> i16 {
    v.try_into().unwrap_or_else(|_| {
        if v > 0 {
            i16::MAX
        } else {
            i16::MIN
        }
    })
}

struct Adsr {
    attack: i32,
    decay: i32,
    sustain: i32,
    release: i32,
}

impl Adsr {
    fn apply(&self, offset: u32, release_offset: Option<u32>, sample: i16) -> i16 {
        assert!(self.attack >= 0);
        assert!(self.decay >= 0);
        assert!(self.sustain >= 0);
        assert!(self.release >= 0);

        let offset: i32 = offset.try_into().expect("overflow");
        let release_offset: i32 = release_offset.unwrap_or(i32::MAX as u32).try_into().expect("overflow");
        let release_offset = if release_offset < self.attack + self.decay {
            self.attack + self.decay
        } else {
            release_offset
        };
        let decay_offset = self.attack;
        let sustain_offset = self.attack + self.decay;
        let release_end_offset = release_offset.saturating_add(self.release);

        let in_attack = offset < decay_offset;
        let in_decay = !in_attack && offset < sustain_offset;
        let in_sustain = !in_attack && !in_decay && offset < release_offset;
        let in_release = !in_attack && !in_decay && !in_sustain && offset < release_end_offset;

        if in_attack {
            todo!()
        } else if in_decay {
            todo!()
        } else if in_sustain {
            todo!()
        } else if in_release {
            todo!()
        } else {
            0
        }
    }
}

struct AdsrOscillator {
    osc: Oscillator,
    adsr: Adsr,
}

impl AdsrOscillator {
    fn get_sample(&self, offset: u32, release_offset: Option<u32>) -> i16 {
        let sample = self.osc.get_sample(offset);
        let sample = self.adsr.apply(offset, release_offset, sample);
        sample
    }
}

fn write_image(buf: &[i16], outdir: &Path, file_stem: &str) -> Result<()> {
    use plotters::prelude::*;

    let filepath = outdir.join(file_stem).with_extension("png");

    let root = BitMapBackend::new(&filepath, (1280, 720)).into_drawing_area();

    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption("Waveform", ("sans-serif", 50).into_font())
        .margin(50_f64)
        .set_label_area_size(LabelAreaPosition::Left, 100_f64)
        .set_label_area_size(LabelAreaPosition::Bottom, 100_f64)
        .build_cartesian_2d(0..buf.len(), (i16::min_value() as f64)..(i16::max_value() as f64))?;

    chart.configure_mesh()
        .draw()?;

    chart.draw_series(
        LineSeries::new(
            (0..).zip(buf.iter().map(|v| *v as f64)),
            RED.mix(1.0).stroke_width(4),
        )
    )?;

    Ok(())
}

fn fill_buf(buf: &mut [i16], osc: Oscillator) {
    for i in 0..buf.len() {
        let sample = osc.get_sample(i as u32);
        buf[i] = sample;
    }
}

fn write_test_osc(name: &str, osc: Oscillator) -> Result<()> {
    let mut buf = vec![0_i16; A440_SAMPLES as usize];
    fill_buf(&mut buf, osc);
    write_image(&buf, &PathBuf::from("out"), name)
}

fn main() -> Result<()> {
    write_test_osc("saw", saw_osc())?;
    write_test_osc("triangle", triangle_osc())?;
    write_test_osc("square", square_osc())?;
    write_test_osc("funky-square", funky_square_osc())?;

    Ok(())
}

