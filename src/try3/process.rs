use super::units::*;
use super::static_config as sc;
use super::dynamic_config as dc;

pub fn process_voice(
    static_config: &sc::Voice,
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
    static_config: &sc::Voice,
    pitch: Hz,
    sample_rate: SampleRateKhz,
    offset: u32,
    release_offset: Option<u32>
) -> dc::Voice {
    let layers = static_config.layers.map(|layer| {
        dc::Layer {
            osc: todo!(),
            lpf: todo!(),
            amp_env: todo!(),
            mod_env: todo!(),
        }
    });

    dc::Voice {
        layers,
    }
}

pub fn sample_voice(
    dynamic_config: &dc::Voice,
    sample_rate: SampleRateKhz,
    offset: u32,
    release_offset: Option<u32>
) -> f32 {
    todo!()
}
