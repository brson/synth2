use crate::f64::*;
use crate::math::*;

struct Set {
    channels: Vec<Channel>,
}

struct Channel {
    name: String,
    synth: Synth,
    clips: Vec<Option<Clip>>,
}

struct Clip {
    notes: Vec<Note>,
}

struct Note {
    start_sample: Snat32,
    sample_length: Snat32,
}

struct Synth {
    osc: Oscillator,
    lpf: LowPassFilter,
    adsr: Adsr,
    gain: f64,
}