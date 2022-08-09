use super::lowered_config as lc;
use super::render_plan as rp;
use super::state as st;
use super::static_config as sc;
use super::units::*;

pub fn lower_config_layer(
    static_config: &sc::Layer,
    sample_rat: SampleRateKhz,
) -> lc::Layer {
    todo!()
}

pub fn render_layer(
    lowered_config: &lc::Layer,
    state: &mut st::Layer,
    pitch: Hz,
    offset: u32,
    release_offset: Option<u32>,
) -> f32 {
    let render_plan = create_render_plan_for_layer(
        lowered_config,
        pitch,
        offset,
        release_offset);
    render_layer_from_render_plan(&render_plan, state)
}

fn create_render_plan_for_layer(
    lowered_config: &lc::Layer,
    pitch: Hz,
    offset: u32,
    release_offset: Option<u32>,
) -> rp::Layer {
    todo!()
}

fn render_layer_from_render_plan(
    render_plan: &rp::Layer,
    state: &mut st::Layer,
) -> f32 {
    sample_voice(render_plan, state)
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
            },
        },
        lpf: rp::LowPassFilter {
            freq: modulated_lpf_freq,
            sample_rate,
        },
        gain: amp_env_sample,
    }
}

fn sample_envelope(
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

fn modulate_freq_unipolar(
    freq: Hz,
    modulation_sample: Unipolar<1>,
    modulation_amount: Bipolar<10>,
) -> Hz {
    let modulation_amount_ = modulation_sample.0 * modulation_amount.0;
    let freq = 2_f32.powf(modulation_amount_) * freq.0;
    Hz(freq)
}

pub fn sample_voice(
    render_plan: &rp::Layer,
    state: &mut st::Layer,
) -> f32 {
    use super::filters::*;
    use super::oscillators::phase_accumulating::*;
    let mut osc = match render_plan.osc.kind {
        rp::OscillatorKind::Square => Oscillator::Square(SquareOscillator {
            state: &mut state.osc,
            period: render_plan.osc.period,
            phase: Unipolar(0.0),
        }),
        rp::OscillatorKind::Saw => Oscillator::Saw(SawOscillator {
            state: &mut state.osc,
            period: render_plan.osc.period,
            phase: Unipolar(0.0),
        }),
        rp::OscillatorKind::Triangle => Oscillator::Triangle(TriangleOscillator {
            state: &mut state.osc,
            period: render_plan.osc.period,
            phase: Unipolar(0.0),
        }),
    };
    let mut lpf = LowPassFilter {
        state: &mut state.lpf,
        sample_rate: render_plan.lpf.sample_rate,
        freq: render_plan.lpf.freq,
    };
    let sample = osc.sample();
    let sample = lpf.process(sample.0);
    let sample = sample * render_plan.gain.0;
    sample
}
