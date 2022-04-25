use anyhow::{Result, anyhow};
use std::path::{Path, PathBuf};
use crate::math::*;

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

pub const SAMPLE_RATE_KHZ: u32 = 32_000;
const A440_SAMPLES: u32 = SAMPLE_RATE_KHZ / 440;

pub fn saw_osc() -> Oscillator {
    Oscillator {
        period: A440_SAMPLES,
        phase: 0,
        triangleness: ZOne64::assert_from(0_f64),
        squareness: ZOne64::assert_from(0_f64),
    }
}

pub fn triangle_osc() -> Oscillator {
    Oscillator {
        period: A440_SAMPLES,
        phase: 0,
        triangleness: ZOne64::assert_from(1_f64),
        squareness: ZOne64::assert_from(0_f64),
    }
}

pub fn square_osc() -> Oscillator {
    Oscillator {
        period: A440_SAMPLES,
        phase: 0,
        triangleness: ZOne64::assert_from(0_f64),
        squareness: ZOne64::assert_from(1_f64),
    }
}

pub fn funky_square_osc() -> Oscillator {
    Oscillator {
        period: A440_SAMPLES,
        phase: 0,
        triangleness: ZOne64::assert_from(1_f64 / 4_f64),
        squareness: ZOne64::assert_from(1_f64 / 4_f64),
    }
}

pub fn square_osc_hz(freq: Hz64) -> OscillatorHz {
    OscillatorHz {
        sample_rate: SampleRateKhz(SAMPLE_RATE_KHZ),
        freq,
        phase: 0,
        triangleness: ZOne64::assert_from(0_f64),
        squareness: ZOne64::assert_from(1_f64),
    }
}

pub struct OscillatorHz {
    pub sample_rate: SampleRateKhz,
    pub freq: Hz64,
    pub phase: i32,
    pub triangleness: ZOne64, // 0 for sawtooth, 1 for triangle
    pub squareness: ZOne64, // 0 for saw/tri, 1 for square
    //pulse_width: Snat32,
}

impl OscillatorHz {
    pub fn sample(&self, offset: u32) -> One64 {
        let period = self.freq.as_samples(self.sample_rate);
        let osc = Oscillator {
            period,
            phase: self.phase,
            triangleness: self.triangleness,
            squareness: self.squareness,
        };
        osc.sample(offset)
    }
}

pub struct Oscillator {
    pub period: u32,
    pub phase: i32,
    pub triangleness: ZOne64, // 0 for sawtooth, 1 for triangle
    pub squareness: ZOne64, // 0 for saw/tri, 1 for square
    //pulse_width: Snat32,
}

enum OscillatorStage {
    InitialRise,
    InitialFall,
    FinalFall,
    FinalRise,
}

impl Oscillator {
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

pub struct AdsrMs {
    pub sample_rate: SampleRateKhz,
    pub attack: Ms64,
    pub decay: Ms64,
    pub sustain: ZOne64,
    pub release: Ms64,
}

pub struct Adsr {
    pub attack: u32,
    pub decay: u32,
    pub sustain: ZOne64,
    pub release: u32,
}

enum AdsrStage {
    Attack,
    Decay,
    Sustain,
    Release,
    End,
}

impl AdsrMs {
    pub fn sample(&self, offset: u32, release_ms: Option<Ms64>) -> ZOne64 {
        let release_offset = release_ms.map(|ms| ms.as_samples(self.sample_rate));
        let sample_adsr = Adsr {
            attack: self.attack.as_samples(self.sample_rate),
            decay: self.decay.as_samples(self.sample_rate),
            sustain: self.sustain,
            release: self.release.as_samples(self.sample_rate),
        };
        sample_adsr.sample(offset, release_offset)
    }
}


impl Adsr {
    pub fn sample(&self, offset: u32, release_offset: Option<u32>) -> ZOne64 {
        let attack: f64 = self.attack.into();
        let decay: f64 = self.decay.into();
        let sustain: f64 = self.sustain.into();
        let release: f64 = self.release.into();

        let offset: f64 = offset.into();
        let decay_offset = attack;
        let sustain_offset = attack + decay;
        let release_offset = f64::from(
            release_offset
                .map(u32::from)
                .unwrap_or(u32::MAX)
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
                ZOne64::assert_from(sample)
            }
            AdsrStage::Decay => {
                let rise = sustain - 1.0;
                let run = decay;
                let x_offset = offset - decay_offset;
                let y_start = 1.0;
                let sample = line_y_value_with_y_offset(
                    rise, run, x_offset, y_start
                );
                ZOne64::assert_from(sample)
            }
            AdsrStage::Sustain => {
                ZOne64::assert_from(sustain)
            }
            AdsrStage::Release => {
                let rise = -sustain;
                let run = release;
                let x_offset = offset - release_offset;
                let y_start = sustain;
                let sample = line_y_value_with_y_offset(
                    rise, run, x_offset, y_start
                );
                ZOne64::assert_from(sample)
            }
            AdsrStage::End => {
                ZOne64::assert_from(0.0)
            }
        }
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


fn write_image(buf: &[f64], outdir: &Path, file_stem: &str) -> Result<()> {
    use plotters::prelude::*;

    let filepath = outdir.join(file_stem).with_extension("png");

    let root = BitMapBackend::new(&filepath, (1280, 720)).into_drawing_area();

    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption("Waveform", ("sans-serif", 50).into_font())
        .margin(50_f64)
        .set_label_area_size(LabelAreaPosition::Left, 100_f64)
        .set_label_area_size(LabelAreaPosition::Bottom, 100_f64)
        .build_cartesian_2d(0..buf.len(), (-1_f64)..(1_f64))?;

    chart.configure_mesh()
        .draw()?;

    chart.draw_series(
        LineSeries::new(
            (0..).zip(buf.iter().copied()),
            RED.mix(1.0).stroke_width(4),
        )
    )?;

    Ok(())
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

fn fill_buf_osc(buf: &mut [f64], osc: Oscillator) {
    for i in 0..buf.len() {
        let sample = osc.sample(i as u32);
        buf[i] = sample.into();
    }
}

fn write_test_osc(name: &str, osc: Oscillator) -> Result<()> {
    let mut buf = vec![0_f64; A440_SAMPLES as usize];
    fill_buf_osc(&mut buf, osc);
    write_image_ch(&buf, &PathBuf::from("out"), name)
}

fn fill_buf_adsr(buf: &mut [f64], adsr: Adsr, release_offset: u32) {
    for i in 0..buf.len() {
        let sample = adsr.sample(
            i as u32,
            Some(release_offset)
        );
        buf[i] = sample.into();
    }
}

fn write_test_adsr() -> Result<()> {
    let mut buf = vec![0_f64; 125];
    let adsr = Adsr {
        attack: 25,
        decay: 25,
        sustain: ZOne64::assert_from(0.5),
        release: 25,
    };
    fill_buf_adsr(&mut buf, adsr, 75);
    write_image_ch(&buf, &PathBuf::from("out"), "f64-adsr")
}

fn write_test_lpf() -> Result<()> {
    let osc = triangle_osc();
    let mut buf = vec![0_f64; A440_SAMPLES as usize * 4];
    fill_buf_osc(&mut buf, osc);
    let mut lpf = LowPassFilter::new(
        ZPos64::assert_from(440.0 * 2.0),
        SAMPLE_RATE_KHZ,
    );
    for i in 0..buf.len() {
        buf[i] = lpf.process(buf[i]);
    }
    write_image_ch(&buf, &PathBuf::from("out"), "f64-lpf")
}

pub fn run() -> Result<()> {
    write_test_osc("f16-saw", saw_osc())?;
    write_test_osc("f16-triangle", triangle_osc())?;
    write_test_osc("f16-square", square_osc())?;
    write_test_osc("f16-funky-square", funky_square_osc())?;
    write_test_adsr()?;
    write_test_lpf()?;

    Ok(())
}

