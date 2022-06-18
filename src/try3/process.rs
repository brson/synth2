use super::units::*;
use super::static_config as sc;
use super::dynamic_config as dc;

pub fn process_layer(
    static_config: &sc::Layer,
    pitch: Hz,
    sample_rate: SampleRateKhz,
    offset: u32,
    release_offset: Option<u32>
) -> f32 {
    let dynamic_config = prepare_frame(static_config, pitch, sample_rate, offset, release_offset);
    let sample = sample_voice(&dynamic_config, offset, release_offset);
    sample
}

fn prepare_frame(
    layer: &sc::Layer,
    pitch: Hz,
    sample_rate: SampleRateKhz,
    offset: u32,
    release_offset: Option<u32>
) -> dc::Layer {
    let mod_env_sample = sample_mod_envelope(
        layer.mod_env,
        sample_rate,
        offset,
        release_offset
    );
    let modulated_osc_freq = modulate_freq_unipolar(
        pitch,
        mod_env_sample,
        layer.modulations.mod_env_to_osc_freq
    );
    let modulated_lpf_freq = modulate_freq_unipolar(
        layer.lpf.freq,
        mod_env_sample,
        layer.modulations.mod_env_to_lpf_freq
    );
    dc::Layer {
        osc: dc::Oscillator {
            period: modulated_osc_freq.as_samples(sample_rate),
            kind: match layer.osc {
                sc::Oscillator::Square => dc::OscillatorKind::Square,
                sc::Oscillator::Saw => dc::OscillatorKind::Saw,
                sc::Oscillator::Triangle => dc::OscillatorKind::Triangle,
            },
        },
        lpf: dc::LowPassFilter {
            freq: modulated_lpf_freq,
            sample_rate,
        },
        amp_env: dc::Adsr {
            attack: layer.amp_env.attack.as_samples(sample_rate),
            decay: layer.amp_env.decay.as_samples(sample_rate),
            sustain: layer.amp_env.sustain,
            release: layer.amp_env.release.as_samples(sample_rate),
        },
    }
}

fn sample_mod_envelope(
    adsr_config: sc::Adsr,
    sample_rate: SampleRateKhz,
    offset: u32,
    release_offset: Option<u32>
) -> Unipolar<1> {
    let adsr = super::envelopes::Adsr {
        attack: adsr_config.attack.as_samples(sample_rate),
        decay: adsr_config.decay.as_samples(sample_rate),
        sustain: adsr_config.sustain,
        release: adsr_config.release.as_samples(sample_rate),
    };
    adsr.sample(offset, release_offset)
}

fn modulate_freq_unipolar(
    freq: Hz,
    modulation_sample: Unipolar<1>,
    modulation_amount: Bipolar<5>,
) -> Hz {
    let modulation_amount = modulation_sample.0 * modulation_amount.0;
    let freq = 2_f32.powf(modulation_amount) * freq.0;
    Hz(freq)
}

pub fn sample_voice(
    dynamic_config: &dc::Layer,
    offset: u32,
    release_offset: Option<u32>
) -> f32 {
    use super::oscillators::*;
    let osc = match dynamic_config.osc.kind {
        dc::OscillatorKind::Square => Oscillator::Square(SquareOscillator {
            period: dynamic_config.osc.period,
        }),
        dc::OscillatorKind::Saw => Oscillator::Saw(SawOscillator {
            period: dynamic_config.osc.period,
        }),
        dc::OscillatorKind::Triangle => Oscillator::Triangle(TriangleOscillator {
            period: dynamic_config.osc.period,
        }),
    };
    todo!()
}
