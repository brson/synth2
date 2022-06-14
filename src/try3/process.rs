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
    let sample = sample_voice(&dynamic_config, sample_rate, offset, release_offset);
    sample
}

fn prepare_frame(
    static_config: &sc::Layer,
    pitch: Hz,
    sample_rate: SampleRateKhz,
    offset: u32,
    release_offset: Option<u32>
) -> dc::Layer {
    let mod_env_sample = sample_mod_envelope(
        static_config.mod_env,
        sample_rate,
        offset,
        release_offset
    );
    dc::Layer {
        osc: todo!(),
        lpf: todo!(),
        amp_env: todo!(),
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

pub fn sample_voice(
    dynamic_config: &dc::Layer,
    sample_rate: SampleRateKhz,
    offset: u32,
    release_offset: Option<u32>
) -> f32 {
    todo!()
}
