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
    todo!()
}

fn prepare_frame(
    static_config: &sc::Voice,
    pitch: Hz,
) -> dc::Voice {
    todo!()
}
