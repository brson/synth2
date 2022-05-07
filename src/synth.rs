use crate::f64::*;
use crate::math::*;
use crate::oscillators::Oscillator;

pub struct Synth2 {
    pub sample_rate: SampleRateKhz,
    pub voice1: Voice,
    pub voice2: Voice,
}

pub struct Voice {
    pub osc: Oscillator,
    pub lpf: LowPassFilter,
    pub amp_env: AdsrMs,
    pub mod_env: AdsrMs,
    pub osc_mod_freq_multiplier: f64, // 1 = no mod, 2 = 1 octave
    pub lpf_mod_range_multiplier: f64, // 1 = no mod, 2 = 1 octave
}

impl Voice {
}

pub struct Synth {
    pub sample_rate: SampleRateKhz,
    pub osc: OscillatorHz,
    pub lpf: LowPassFilter,
    pub adsr: AdsrMs,
    pub gain: ZPos64,
    pub lpf_mod_adsr: AdsrMs,
    pub lpf_mod_range_multiplier: f64, // 0 = no mod, 1 = 1 octave
}

impl Synth {
    pub fn sample(&mut self, offset: u32) -> f64 {
        let release = Some(Ms64::assert_from(50.0));

        let mut modulated_lpf = {
            let lpf_mod_sample = self.lpf_mod_adsr.sample(self.sample_rate, offset, release);
            let lpf_mod_sample = f64::from(lpf_mod_sample);
            let lpf_freq = f64::from(self.lpf.freq);
            let addtl_lpf_freq = lpf_freq * lpf_mod_sample * self.lpf_mod_range_multiplier;
            let lpf_freq = lpf_freq + addtl_lpf_freq;
            let lpf_freq = ZPos64::assert_from(lpf_freq);
            let lpf = self.lpf.modulate(lpf_freq);
            lpf
        };

        let sample = f64::from(self.osc.sample(offset));
        let sample = modulated_lpf.process(sample);
        let release_offset = Ms64::assert_from(1000.0).as_samples(self.sample_rate);
        let adsr_sample = self.adsr.sample(self.sample_rate, offset, release);

        let sample = sample * f64::from(adsr_sample);
        let sample = sample * f64::from(self.gain);
        sample
    }
}
