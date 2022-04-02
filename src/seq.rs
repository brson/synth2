use crate::f64::*;

const MAX_CLIPS: usize = 32;

struct Channel {
    synth: Synth,
    clips: [Option<Clip>; MAX_CLIPS],
}

struct Clip {
}

struct Synth {
    osc: Oscillator,
    lpf: LowPassFilter,
    adsr: Adsr,
    gain: f64,
}
