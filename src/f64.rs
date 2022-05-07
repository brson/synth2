use anyhow::{Result, anyhow};
use std::path::{Path, PathBuf};
use crate::math::*;

pub const SAMPLE_RATE_KHZ: u32 = 32_000;

pub fn saw_osc_hz(freq: Hz64) -> OscillatorHz {
    OscillatorHz {
        sample_rate: SampleRateKhz(SAMPLE_RATE_KHZ),
        freq,
        triangleness: ZOne64::assert_from(0_f64),
        squareness: ZOne64::assert_from(0_f64),
        sineness: ZOne64::assert_from(0_f64),
    }
}

pub fn triangle_osc_hz(freq: Hz64) -> OscillatorHz {
    OscillatorHz {
        sample_rate: SampleRateKhz(SAMPLE_RATE_KHZ),
        freq,
        triangleness: ZOne64::assert_from(1_f64),
        squareness: ZOne64::assert_from(0_f64),
        sineness: ZOne64::assert_from(0_f64),
    }
}

pub fn square_osc_hz(freq: Hz64) -> OscillatorHz {
    OscillatorHz {
        sample_rate: SampleRateKhz(SAMPLE_RATE_KHZ),
        freq,
        triangleness: ZOne64::assert_from(0_f64),
        squareness: ZOne64::assert_from(1_f64),
        sineness: ZOne64::assert_from(0_f64),
    }
}

pub fn funky_osc_hz(freq: Hz64) -> OscillatorHz {
    OscillatorHz {
        sample_rate: SampleRateKhz(SAMPLE_RATE_KHZ),
        freq,
        triangleness: ZOne64::assert_from(0.25_f64),
        squareness: ZOne64::assert_from(0.25_f64),
        sineness: ZOne64::assert_from(0_f64),
    }
}

pub fn sin_osc_hz(freq: Hz64) -> OscillatorHz {
    OscillatorHz {
        sample_rate: SampleRateKhz(SAMPLE_RATE_KHZ),
        freq,
        triangleness: ZOne64::assert_from(0_f64),
        squareness: ZOne64::assert_from(0_f64),
        sineness: ZOne64::assert_from(1_f64),
    }
}

pub fn round_saw_osc_hz(freq: Hz64) -> OscillatorHz {
    OscillatorHz {
        sample_rate: SampleRateKhz(SAMPLE_RATE_KHZ),
        freq,
        triangleness: ZOne64::assert_from(0_f64),
        squareness: ZOne64::assert_from(0_f64),
        sineness: ZOne64::assert_from(0.5_f64),
    }
}

pub fn round_triangle_osc_hz(freq: Hz64) -> OscillatorHz {
    OscillatorHz {
        sample_rate: SampleRateKhz(SAMPLE_RATE_KHZ),
        freq,
        triangleness: ZOne64::assert_from(1_f64),
        squareness: ZOne64::assert_from(0_f64),
        sineness: ZOne64::assert_from(0.5_f64),
    }
}

pub fn round_square_osc_hz(freq: Hz64) -> OscillatorHz {
    OscillatorHz {
        sample_rate: SampleRateKhz(SAMPLE_RATE_KHZ),
        freq,
        triangleness: ZOne64::assert_from(0_f64),
        squareness: ZOne64::assert_from(1_f64),
        sineness: ZOne64::assert_from(0.5_f64),
    }
}

pub fn round_funky_osc_hz(freq: Hz64) -> OscillatorHz {
    OscillatorHz {
        sample_rate: SampleRateKhz(SAMPLE_RATE_KHZ),
        freq,
        triangleness: ZOne64::assert_from(0.25_f64),
        squareness: ZOne64::assert_from(0.25_f64),
        sineness: ZOne64::assert_from(0.5_f64),
    }
}

pub struct OscillatorHz {
    pub sample_rate: SampleRateKhz,
    pub freq: Hz64,
    pub triangleness: ZOne64, // 0 for sawtooth, 1 for triangle
    pub squareness: ZOne64, // 0 for saw/tri, 1 for square
    pub sineness: ZOne64, // 0 for saw/tri/square, 1 for sine
}

impl OscillatorHz {
    pub fn sample(&self, offset: u32) -> One64 {
        let period = self.freq.as_samples(self.sample_rate);
        let angle_osc = AngleOscillator {
            period,
            triangleness: self.triangleness,
            squareness: self.squareness,
        };
        let sin_osc = SinOscillator {
            period,
        };
        let angle_sample = f64::from(angle_osc.sample(offset));
        let sin_sample = f64::from(sin_osc.sample(offset));

        // Cross-fade between the angle_osc and sin_osc
        let sample = {
            let diff_range = sin_sample - angle_sample;
            let angle_offset = diff_range * f64::from(self.sineness);
            let sample = angle_sample + angle_offset;
            sample
        };

        // Give it some curvature
        let sample = {
            let pow = 2;
            let curved_sample = if sample > 0.0 {
                sample.powi(pow)
            } else {
                -sample.powi(pow)
            };

            // Cross-fade
            let curviness = 0.2;
            let diff_range = curved_sample - sample;
            let sample_offset = diff_range * curviness;
            let sample = sample + sample_offset;
            sample
        };

        One64::assert_from(sample)
    }
}

struct SinOscillator {
    pub period: u32,
}

impl SinOscillator {
    pub fn sample(&self, offset: u32) -> One64 {
        let period = f64::from(self.period);
        let offset = f64::from(offset);
        let offset = offset % period;
        let pi = std::f64::consts::PI;
        let sin_offset = offset * pi * 2.0 / period;
        let sample = sin_offset.sin();
        One64::assert_from(sample)
    }
}

struct AngleOscillator {
    pub period: u32,
    pub triangleness: ZOne64, // 0 for sawtooth, 1 for triangle
    pub squareness: ZOne64, // 0 for saw/tri, 1 for square
}

enum AngleOscillatorStage {
    InitialRise,
    InitialFall,
    FinalFall,
    FinalRise,
}

impl AngleOscillator {
    pub fn sample(&self, offset: u32) -> One64 {
        let offset: f64 = offset.into();
        let period: f64 = self.period.into();
        let triangleness: f64 = self.triangleness.into();
        let squareness: f64 = self.squareness.into();

        let osc_offset = offset % period;
        let half_period = period / 2.0;

        let rise_time = half_period * triangleness;
        let half_rise_time = rise_time / 2.0;
        let fall_time = period - rise_time;
        let half_fall_time = fall_time / 2.0;

        let stage = {
            let in_initial_rise = osc_offset < half_rise_time;
            let in_final_rise = osc_offset >= period - half_rise_time;
            let in_fall = !in_initial_rise && !in_final_rise;
            let in_first_half_fall = in_fall && osc_offset < half_period;
            let in_second_half_fall = in_fall && !in_first_half_fall;

            if in_initial_rise {
                AngleOscillatorStage::InitialRise
            } else if in_first_half_fall {
                AngleOscillatorStage::InitialFall
            } else if in_second_half_fall {
                AngleOscillatorStage::FinalFall
            } else if in_final_rise {
                AngleOscillatorStage::FinalRise
            } else {
                unreachable!()
            }
        };

        match stage {
            AngleOscillatorStage::InitialRise => {
                let rise = 1.0;
                let run = half_rise_time;
                let x_offset = osc_offset;
                let y_start = 0.0;
                let sample = line_y_value_with_y_offset(
                    rise, run, x_offset, y_start
                );
                One64::assert_from(sample)
            }
            AngleOscillatorStage::InitialFall => {
                let rise = -(1.0 - squareness);
                let run = half_fall_time;
                let x_offset = osc_offset - half_rise_time;
                let y_start = 1.0;
                let sample = line_y_value_with_y_offset(
                    rise, run, x_offset, y_start
                );
                One64::assert_from(sample)
            }
            AngleOscillatorStage::FinalFall => {
                let rise = -(1.0 - squareness);
                let run = half_fall_time;
                let x_offset = osc_offset - half_period;
                let y_start = -squareness;
                let sample = line_y_value_with_y_offset(
                    rise, run, x_offset, y_start
                );
                One64::assert_from(sample)
            }
            AngleOscillatorStage::FinalRise => {
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

pub struct Noise {
    pub seed: u32,
}

impl Noise {
    pub fn sample(&self, offset: u32) -> One64 {
        // Use a hash function to generate a pseudo-random rng seed
        // based on the noise seed and the offset,
        // then an rng to generate a sample with the correct distribution.
        
        //let mut hasher = blake3::Hasher::new();
        //let seed_buf = &self.seed.to_le_bytes()[..];
        //let offset_buf = &i32::from(offset).to_le_bytes()[..];
        //hasher.update(seed_buf);
        //hasher.update(offset_buf);
        //let hash_bytes = hasher.finalize().as_bytes();
        //let mut rand_buf = [0; 16];
        //rand_buf.copy_from_slice(hash_bytes);
        //let rand_u128 = u128::from_le_bytes(rand_buf);
        //let mut rng = rand_pcg::Pcg64Mcg::new(rand_u64);

        use std::hash::Hasher;
        use rand::Rng;

        let mut hasher = fxhash::FxHasher::default();
        hasher.write_u32(self.seed);
        hasher.write_u32(offset);
        let rand_u64 = hasher.finish();
        let mut rng = rand_pcg::Pcg64Mcg::new(rand_u64.into());
        let rand_f64: f64 = rng.gen_range(-1.0..=1.0);
        One64::assert_from(rand_f64)
    }
}

// https://www.musicdsp.org/en/latest/Filters/237-one-pole-filter-lp-and-hp.html
#[derive(Debug)]
pub struct LowPassFilter {
    pub freq: ZPos64,
    sample_rate: u32,
    a0: f64,
    b1: f64,
    last: f64,
}

impl LowPassFilter {
    pub fn new(freq: ZPos64, sample_rate: u32) -> LowPassFilter {
        let x = (-2.0 * std::f64::consts::PI * f64::from(freq) / f64::from(sample_rate)).exp();
        LowPassFilter {
            freq,
            sample_rate,
            a0: 1.0 - x,
            b1: -x,
            last: 0.0,
        }
    }

    pub fn process(&mut self, input: f64) -> f64 {
        let out = self.a0 * input - self.b1 * self.last;
        self.last = out;
        out
    }

    pub fn modulate(&mut self, freq: ZPos64) -> ModulatedLowPassFilter<'_> {
        ModulatedLowPassFilter {
            parent: self,
            freq,
        }
    }
}

pub struct ModulatedLowPassFilter<'this> {
    parent: &'this mut LowPassFilter,
    freq: ZPos64,
}

impl<'this> ModulatedLowPassFilter<'this> {
    pub fn process(&mut self, input: f64) -> f64 {
        let mut child_lpf = LowPassFilter::new(self.freq, self.parent.sample_rate);
        child_lpf.last = self.parent.last;
        let sample = child_lpf.process(input);
        self.parent.last = child_lpf.last;
        sample
    }
}

fn write_image_ch(buf: &[f64], outdir: &Path, file_stem: &str) -> Result<()> {
    use charts::{Chart, ScaleLinear, MarkerType, PointLabelPosition, LineSeriesView};

    let filepath = outdir.join(file_stem).with_extension("svg");

    let width = 1280;
    let height = 720;
    let (top, right, bottom, left) = (100, 100, 100, 100);

    let available_width = width - left - right;
    let available_height = height - top - bottom;

    let x = ScaleLinear::new()
        .set_domain(vec![0_f32, buf.len() as f32])
        .set_range(vec![0, available_width]);

    let y = ScaleLinear::new()
        .set_domain(vec![-1_f32, 1_f32])
        .set_range(vec![available_height, 0]);

    let line_data = (0..).zip(buf.iter().copied());
    let line_data = line_data.map(|(x, y)| (x as f32, y as f32));
    let line_data: Vec<_> = line_data.collect();

    let line_view = LineSeriesView::new()
        .set_x_scale(&x)
        .set_y_scale(&y)
        .set_marker_type(MarkerType::Circle)
    //.set_label_position(PointLabelPosition::N)
        .set_label_visibility(false)
        .load_data(&line_data)
        .map_err(|e| anyhow!("{}", e))?;

    Chart::new()
        .set_width(width)
        .set_height(height)
        .set_margins(top, right, bottom, left)
        //.add_title(String::from("Line Chart"))
        .add_view(&line_view)
        .add_axis_bottom(&x)
        .add_axis_left(&y)
        //.add_left_axis_label("Custom Y Axis Label")
        //.add_bottom_axis_label("Custom X Axis Label")
        .save(filepath)
        .map_err(|e| anyhow!("{}", e))?;

    Ok(())
}

