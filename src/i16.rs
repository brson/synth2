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
    sustain: i16, // i16::MAX = full scale
    release: i32,
}

impl Adsr {
    fn get_sample(&self, offset: u32, release_offset: Option<u32>) -> i16 {
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
            let working_offset = offset;
            let rise = i16::MAX.into();
            let run = self.attack;
            let sample = line_y_value(rise, run, working_offset);
            clamp_i32_to_i16(sample)
        } else if in_decay {
            let working_offset = offset - decay_offset;
            let y_start = i16::MAX.into();
            let rise = i32::from(self.sustain) - i32::from(i16::MAX);
            let run = self.decay;
            let sample = line_y_value_with_y_offset(rise, run, working_offset, y_start);
            clamp_i32_to_i16(sample)
        } else if in_sustain {
            self.sustain
        } else if in_release {
            let working_offset = offset - release_offset;
            let y_start = i32::from(self.sustain);
            let rise = i32::from(-self.sustain);
            let run = self.release;
            let sample = line_y_value_with_y_offset(rise, run, working_offset, y_start);
            clamp_i32_to_i16(sample)
        } else {
            0
        }
    }
}

fn line_y_value(y_rise: i32, x_run: i32, x_value: i32) -> i32 {
    // slope = rise / run
    // y_value = slope * x_offset
    x_value.saturating_mul(y_rise) / x_run
}

fn line_y_value_with_y_offset(
    y_rise: i32,
    x_run: i32,
    x_value: i32,
    y_offset: i32
) -> i32
{
    let value = line_y_value(y_rise, x_run, x_value);
    value.saturating_add(y_offset)
}

struct Voice {
    osc: Oscillator,
    adsr: Adsr,
}

impl Voice {
    fn get_sample(&self, offset: u32, release_offset: Option<u32>) -> i16 {
        let osc_sample = self.osc.get_sample(offset);
        let adsr_sample = self.adsr.get_sample(offset, release_offset);
        modulate_amplitude(osc_sample, adsr_sample)
    }
}

fn modulate_amplitude(sample: i16, adsr_sample: i16) -> i16 {
    assert!(adsr_sample >= 0);
    let sample = i32::from(sample);
    let adsr_sample = i32::from(adsr_sample);
    let i16_max = i32::from(i16::MAX);
    //let fraction = adsr_sample / i16_max;
    //let sample = sample * fraction;
    let sample = sample.saturating_mul(adsr_sample) / i16_max;
    clamp_i32_to_i16(sample)
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

fn fill_buf_osc(buf: &mut [i16], osc: Oscillator) {
    for i in 0..buf.len() {
        let sample = osc.get_sample(i as u32);
        buf[i] = sample;
    }
}

fn write_test_osc(name: &str, osc: Oscillator) -> Result<()> {
    let mut buf = vec![0_i16; A440_SAMPLES as usize];
    fill_buf_osc(&mut buf, osc);
    write_image(&buf, &PathBuf::from("out"), name)
}

fn fill_buf_adsr(buf: &mut [i16], adsr: Adsr, release_offset: u32) {
    for i in 0..buf.len() {
        let sample = adsr.get_sample(i as u32, Some(release_offset));
        buf[i] = sample;
    }
}

fn write_test_adsr() -> Result<()> {
    let mut buf = vec![0_i16; SAMPLE_RATE_KHZ as usize];
    let adsr = Adsr {
        attack: SAMPLE_RATE_KHZ / 4,
        decay: SAMPLE_RATE_KHZ / 4,
        sustain: i16::MAX / 2,
        release: SAMPLE_RATE_KHZ / 4,
    };
    fill_buf_adsr(&mut buf, adsr, (SAMPLE_RATE_KHZ / 4 * 3) as u32);
    write_image(&buf, &PathBuf::from("out"), "i16-adsr")
}

fn fill_buf_voice(buf: &mut [i16], voice: Voice, release_offset: u32) {
    for i in 0..buf.len() {
        let sample = voice.get_sample(i as u32, Some(release_offset));
        buf[i] = sample;
    }
}

fn write_test_voice() -> Result<()> {
    let samples = A440_SAMPLES * 16 as i32;
    let mut buf = vec![0_i16; samples as usize];
    let osc = square_osc();
    let adsr = Adsr {
        attack: samples / 4,
        decay: samples / 4,
        sustain: i16::MAX / 2,
        release: samples / 4,
    };
    let voice = Voice { osc, adsr };
    fill_buf_voice(&mut buf, voice, (samples / 4 * 3) as u32);
    write_image(&buf, &PathBuf::from("out"), "i16-voice")
}

pub fn run() -> Result<()> {
    //write_test_osc("i16-saw", saw_osc())?;
    //write_test_osc("i16-triangle", triangle_osc())?;
    //write_test_osc("i16-square", square_osc())?;
    //write_test_osc("i16-funky-square", funky_square_osc())?;
    //write_test_adsr()?;
    //write_test_voice()?;

    Ok(())
}
