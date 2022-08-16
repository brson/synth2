use super::render_plan as rp;
use super::state as st;
use super::static_config as sc;
use super::units::*;
use super::math;

pub fn process_layer_buf_simd(
    static_config: &sc::Layer,
    state: &mut st::Layer,
    pitch: Hz,
    sample_rate: SampleRateKhz,
    offset: u32,
    release_offset: Option<u32>,
    buf: &mut [f32],
) {
    let mut offset = offset;

    let mut chunks = buf.array_chunks_mut::<16>();

    while let Some(chunk) = chunks.next() {
        *chunk = process_layer_x16(
            static_config,
            state,
            pitch,
            sample_rate,
            offset,
            release_offset
        );
        offset = offset.checked_add(16).expect("overflow");
    }

    let remainder = chunks.into_remainder();
    process_layer_buf_sisd(
        static_config,
        state,
        pitch,
        sample_rate,
        offset,
        release_offset,
        remainder,
    );
}

pub fn process_layer_buf_sisd(
    static_config: &sc::Layer,
    state: &mut st::Layer,
    pitch: Hz,
    sample_rate: SampleRateKhz,
    offset: u32,
    release_offset: Option<u32>,
    buf: &mut [f32],
) {
    let mut offset = offset;

    for byte in buf {
        *byte = process_layer(
            static_config,
            state,
            pitch,
            sample_rate,
            offset,
            release_offset,
        );
        offset = offset.checked_add(1).expect("overflow");
    }
}

pub fn process_layer(
    static_config: &sc::Layer,
    state: &mut st::Layer,
    pitch: Hz,
    sample_rate: SampleRateKhz,
    offset: u32,
    release_offset: Option<u32>,
) -> f32 {
    let render_plan = prepare_frame(static_config, pitch, sample_rate, offset, release_offset);
    let sample = sample_voice(&render_plan, state);
    sample
}

pub fn process_layer_x16(
    static_config: &sc::Layer,
    state: &mut st::Layer,
    pitch: Hz,
    sample_rate: SampleRateKhz,
    offset: u32,
    release_offset: Option<u32>,
) -> [f32; 16] {
    let render_plan = prepare_frame_x16(static_config, pitch, sample_rate, offset, release_offset);
    let sample = sample_voice_x16(render_plan, state);
    sample
}

fn prepare_frame(
    layer: &sc::Layer,
    pitch: Hz,
    sample_rate: SampleRateKhz,
    offset: u32,
    release_offset: Option<u32>,
) -> rp::Layer {
    let amp_env_sample = sample_envelope(layer.amp_env, sample_rate, offset, release_offset);
    let mod_env_sample = sample_envelope(layer.mod_env, sample_rate, offset, release_offset);
    let modulated_osc_freq =
        modulate_freq_unipolar(pitch, mod_env_sample, layer.modulations.mod_env_to_osc_freq);
    let modulated_lpf_freq = modulate_freq_unipolar(
        layer.lpf.freq,
        mod_env_sample,
        layer.modulations.mod_env_to_lpf_freq,
    );
    rp::Layer {
        osc: rp::Oscillator {
            period: modulated_osc_freq.as_samples(sample_rate),
            kind: match layer.osc {
                sc::Oscillator::Square => rp::OscillatorKind::Square,
                sc::Oscillator::Saw => rp::OscillatorKind::Saw,
                sc::Oscillator::Triangle => rp::OscillatorKind::Triangle,
                sc::Oscillator::Sine => rp::OscillatorKind::Sine,
            },
        },
        lpf: rp::LowPassFilter {
            freq: modulated_lpf_freq,
            sample_rate,
        },
        gain: amp_env_sample,
    }
}

fn prepare_frame_x16(
    layer: &sc::Layer,
    pitch: Hz,
    sample_rate: SampleRateKhz,
    offset: u32,
    release_offset: Option<u32>,
) -> rp::LayerX<16> {
    let amp_env_samples = sample_envelope_x16(layer.amp_env, sample_rate, offset, release_offset);
    let mod_env_samples = sample_envelope_x16(layer.mod_env, sample_rate, offset, release_offset);
    let modulated_osc_freqs =
        modulate_freq_unipolar_x16(pitch, mod_env_samples, layer.modulations.mod_env_to_osc_freq);
    let modulated_lpf_freqs = modulate_freq_unipolar_x16(
        layer.lpf.freq,
        mod_env_samples,
        layer.modulations.mod_env_to_lpf_freq,
    );

    use super::units::HzX16;
    let modulated_osc_periods = modulated_osc_freqs.as_samples(sample_rate);

    rp::LayerX {
        osc: rp::OscillatorX {
            kind: match layer.osc {
                sc::Oscillator::Square => rp::OscillatorKind::Square,
                sc::Oscillator::Saw => rp::OscillatorKind::Saw,
                sc::Oscillator::Triangle => rp::OscillatorKind::Triangle,
                sc::Oscillator::Sine => rp::OscillatorKind::Sine,
            },
            periods: modulated_osc_periods,
        },
        lpf: rp::LowPassFilterX {
            sample_rate,
            freqs: modulated_lpf_freqs,
        },
        gains: amp_env_samples,
    }
}

pub fn sample_envelope(
    adsr_config: sc::Adsr,
    sample_rate: SampleRateKhz,
    offset: u32,
    release_offset: Option<u32>,
) -> Unipolar<1> {
    let adsr = super::envelopes::Adsr {
        attack: adsr_config.attack.as_samples(sample_rate),
        decay: adsr_config.decay.as_samples(sample_rate),
        sustain: adsr_config.sustain,
        release: adsr_config.release.as_samples(sample_rate),
    };
    adsr.sample(offset, release_offset)
}

pub fn sample_envelope_x16(
    adsr_config: sc::Adsr,
    sample_rate: SampleRateKhz,
    offset: u32,
    release_offset: Option<u32>,
) -> [Unipolar<1>; 16] {
    use crate::old::simdtest;
    use std::simd::{Simd, u32x16, f32x16};

    let adsr = simdtest::AdsrX16 {
        attack: f32x16::splat(adsr_config.attack.as_samples(sample_rate).0),
        decay: f32x16::splat(adsr_config.decay.as_samples(sample_rate).0),
        sustain: f32x16::splat(adsr_config.sustain.0),
        release: f32x16::splat(adsr_config.release.as_samples(sample_rate).0),
    };

    let indexes = math::indexes_u32::<16>();
    let indexes = u32x16::from_array(indexes);
    let offsets = u32x16::splat(offset);
    let offsets = offsets + indexes;

    let samples = adsr.sample(offsets, release_offset);
    let samples = samples.to_array();
    let samples = samples.map(|s| Unipolar(s));

    samples
}

fn modulate_freq_unipolar(
    freq: Hz,
    modulation_sample: Unipolar<1>,
    modulation_amount: Bipolar<10>,
) -> Hz {
    let modulation_amount_ = modulation_sample.0 * modulation_amount.0;
    let freq = 2_f32.powf(modulation_amount_) * freq.0;
    Hz(freq)
}

fn modulate_freq_unipolar_x16(
    freq: Hz,
    modulation_sample: [Unipolar<1>; 16],
    modulation_amount: Bipolar<10>,
) -> [Hz; 16] {
    use crate::old::simdtest;
    use std::simd::{Simd, u32x16, f32x16};
    use sleef::Sleef; // pow

    let freq = f32x16::splat(freq.0);
    let modulation_sample = modulation_sample.map(|s| s.0);
    let modulation_sample = f32x16::from_array(modulation_sample);
    let modulation_amount = f32x16::splat(modulation_amount.0);

    let two = f32x16::splat(2.0);

    let modulation_amount_ = modulation_sample * modulation_amount;
    let freq = two.pow(modulation_amount_) * freq;

    let freq = freq.to_array();
    let freq = freq.map(|f| Hz(f));

    freq
}

pub fn sample_voice(
    render_plan: &rp::Layer,
    state: &mut st::Layer,
) -> f32 {
    use super::filters::*;
    use super::oscillators::phase_accumulating::*;
    let sample = match render_plan.osc.kind {
        rp::OscillatorKind::Square => {
            SquareOscillator {
                state: &mut state.osc,
                period: render_plan.osc.period,
                phase: Unipolar(0.0),
            }.sample()
        },
        rp::OscillatorKind::Saw => {
            SawOscillator {
                state: &mut state.osc,
                period: render_plan.osc.period,
                phase: Unipolar(0.0),
            }.sample()
        },
        rp::OscillatorKind::Triangle => {
            TriangleOscillator {
                state: &mut state.osc,
                period: render_plan.osc.period,
                phase: Unipolar(0.0),
            }.sample()
        },
        rp::OscillatorKind::Sine => {
            SineOscillator {
                state: &mut state.osc,
                period: render_plan.osc.period,
                phase: Unipolar(0.0),
            }.sample()
        },
    };
    let mut lpf = LowPassFilter {
        state: &mut state.lpf,
        sample_rate: render_plan.lpf.sample_rate,
        freq: render_plan.lpf.freq,
    };
    let sample = lpf.process(sample.0);
    let sample = sample * render_plan.gain.0;
    sample
}

pub fn sample_voice_x16(
    render_plan: rp::LayerX<16>,
    state: &mut st::Layer,
) -> [f32; 16] {
    use super::filters::*;
    use super::oscillators::phase_accumulating::*;
    use std::simd::{Simd, f32x16};

    let samples = match render_plan.osc.kind {
        rp::OscillatorKind::Square => {
            SquareOscillatorX16 {
                state: &mut state.osc,
                period: render_plan.osc.periods,
                phase: Unipolar(0.0)
            }.sample()
        },
        rp::OscillatorKind::Saw => {
            SawOscillatorX16 {
                state: &mut state.osc,
                period: render_plan.osc.periods,
                phase: Unipolar(0.0)
            }.sample()
        },
        rp::OscillatorKind::Triangle => {
            TriangleOscillatorX16 {
                state: &mut state.osc,
                period: render_plan.osc.periods,
                phase: Unipolar(0.0)
            }.sample()
        },
        rp::OscillatorKind::Sine => {
            SineOscillatorX16 {
                state: &mut state.osc,
                period: render_plan.osc.periods,
                phase: Unipolar(0.0)
            }.sample()
        },
    };

    let sample_rate = render_plan.lpf.sample_rate;
    let samples = samples.map(|s| s.0);
    let lpf_freqs = render_plan.lpf.freqs;

    let samples = samples.zip(lpf_freqs).map(|(sample, lpf_freq)| {
        let mut lpf = LowPassFilter {
            state: &mut state.lpf,
            sample_rate: sample_rate,
            freq: lpf_freq,
        };
        lpf.process(sample)
    });

    let samples = f32x16::from_array(samples);
    let gains = render_plan.gains.map(|g| g.0);
    let gains = f32x16::from_array(gains);
    let samples = samples * gains;

    samples.to_array()
}
