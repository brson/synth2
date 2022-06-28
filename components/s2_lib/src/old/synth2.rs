use super::envelopes::Adsr;
use super::filters::LowPassFilter;
use super::math::*;
use super::oscillators::Oscillator;

pub struct Synth2 {
    pub partial1: Partial,
    pub partial2: Partial,
}

pub struct Partial {
    pub osc: Oscillator,
    pub lpf: LowPassFilter,
    pub lpf_freq: Hz64,
    pub amp_env: Adsr,
    pub mod_env: Adsr,
    pub pitch_mod: f64, // 0 = no mod, 1 = 1 octave
    pub lpf_mod: f64,   // 0 = no mod, 1 = 1 octave
}

impl Synth2 {
    pub fn sample(
        &mut self,
        sample_rate: SampleRateKhz,
        pitch: Hz64,
        offset: u32,
        release_offset: Option<u32>,
    ) -> f64 {
        let partial1_value = self
            .partial1
            .sample(sample_rate, pitch, offset, release_offset);
        let partial2_value = self
            .partial2
            .sample(sample_rate, pitch, offset, release_offset);
        partial1_value + partial2_value
    }
}

impl Partial {
    pub fn sample(
        &mut self,
        sample_rate: SampleRateKhz,
        pitch: Hz64,
        offset: u32,
        release_offset: Option<u32>,
    ) -> f64 {
        let mod_value = self.mod_env.sample(sample_rate, offset, release_offset);
        let mod_value = f64::from(mod_value);

        let pitch = f64::from(pitch);
        let mod_addtl_pitch = pitch * mod_value * self.pitch_mod;
        let pitch = pitch * mod_value;
        let pitch = Hz64::assert_from(pitch);

        let lpf_freq = f64::from(self.lpf_freq);
        let mod_addtl_lpf_freq = lpf_freq * mod_value * self.lpf_mod;
        let lpf_freq = lpf_freq + mod_addtl_lpf_freq;
        let lpf_freq = Hz64::assert_from(lpf_freq);

        let osc_sample = self.osc.sample(sample_rate, pitch, offset);
        let osc_sample = f64::from(osc_sample);

        let filtered_sample = self.lpf.process(sample_rate, lpf_freq, osc_sample);

        let amp_value = self.amp_env.sample(sample_rate, offset, release_offset);
        let amp_value = f64::from(amp_value);

        let amped_sample = filtered_sample * amp_value;

        amped_sample
    }
}
