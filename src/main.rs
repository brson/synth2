#![allow(unused)]

use anyhow::Result;
use std::path::{Path, PathBuf};

const SAMPLE_RATE_KHZ: u32 = 32_000;
const A440_SAMPLES: u32 = SAMPLE_RATE_KHZ / 440;

fn saw_osc() -> Oscillator {
    Oscillator {
        amplitude: i16::MAX,
        period: A440_SAMPLES,
        phase: 0,
        pulse_width: A440_SAMPLES / 2,
        rise_time: 0,
        squareness: 0,
    }
}

// 16-bit, 32 khz
struct Oscillator {
    amplitude: i16,
    period: u32,
    phase: u32,
    pulse_width: u32,
    rise_time: u32, // 0 for sawtooth, period / 2 for triangle
    squareness: i16, // 0 for saw/tri, amplitude for square
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
        assert!(self.rise_time <= self.period / 2);

        let osc_offset = offset % self.period;
        let half_period = self.period / 2;
        let half_rise_time = self.rise_time / 2;
        let in_initial_rise = osc_offset < half_rise_time;
        let in_final_rise = osc_offset > self.period - half_rise_time;
        let in_fall = !in_initial_rise && !in_final_rise;
        let fall_time = self.period - self.rise_time;
        let half_fall_time = fall_time / 2;

        let amplitude_i32: i32 = self.amplitude.into();
        let rise_time_i32: i32 = self.rise_time.try_into().expect("overflow");
        let half_rise_time_i32: i32 = half_rise_time.try_into().expect("overflow");
        let osc_offset_i32: i32 = osc_offset.try_into().expect("overflow");
        let fall_time_i32: i32 = fall_time.try_into().expect("overflow");
        let half_fall_time_i32: i32 = half_fall_time.try_into().expect("overflow");

        if in_initial_rise {
            // delta = amplitude / half_rise_time
            // sample = delta * osc_offset
            let sample = amplitude_i32.saturating_mul(osc_offset_i32) / half_rise_time_i32;
            sample.try_into().expect("overflow")
        } else if in_fall {
            let working_offset = osc_offset_i32 - half_rise_time_i32;
            // delta = amplitude / half_fall_time
            // sample = amplitude - delta * working_offset
            let sample = amplitude_i32 - amplitude_i32.saturating_mul(working_offset) / half_fall_time_i32;
            sample.try_into().expect("overflow")
        } else if in_final_rise {
            let working_offset = osc_offset_i32 - half_rise_time_i32 - fall_time_i32;
            
            let sample = amplitude_i32.saturating_mul(working_offset) / half_rise_time_i32 - amplitude_i32;
            sample.try_into().expect("overflow")
        } else {
            unreachable!()
        }       
    }
}


fn write_image(buf: &[i16], outdir: &Path, file_stem: &str) -> Result<()> {
    use plotters::prelude::*;

    let filepath = outdir.join(file_stem).with_extension("png");

    let root = BitMapBackend::new(&filepath, (1280, 720)).into_drawing_area();

    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption("Waveform", ("sans-serif", 50).into_font())
        .margin(50)
        .build_cartesian_2d(0..buf.len(), (i16::min_value() as f64)..(i16::max_value() as f64))?;

    chart.configure_mesh()
        .draw()?;

    chart.draw_series(
        LineSeries::new(
            (0..).zip(buf.iter().map(|v| *v as f64)),
            RED.mix(0.5).stroke_width(4),
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

    Ok(())
}

