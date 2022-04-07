use crate::f64::*;
use crate::math::*;

struct Set {
    track: Vec<Track>,
}

struct Track {
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
    gain: ZPos64,
}

pub struct Sequencer {
    global_offset: Snat32,
    bps: Pos64,
    synth: Synth,
}

impl Sequencer {
    pub fn new() -> Sequencer {

        let synth = Synth {
            osc: saw_osc(),
            lpf: LowPassFilter::new(
                ZPos64::assert_from(440.0 * 1.5),
                Snat32::assert_from(SAMPLE_RATE_KHZ),
            ),
            adsr: Adsr {
                attack: Snat32::assert_from(10),
                decay: Snat32::assert_from(100),
                sustain: ZOne64::assert_from(0.1),
                release: Snat32::assert_from(100),
            },
            gain: ZPos64::assert_from(0.0),
        };
        
        Sequencer {
            global_offset: Snat32::assert_from(0),
            bps: Pos64::assert_from(120.0),
            synth,
        }
    }

    pub fn next_sample(&mut self) -> f64 {
        todo!()
    }
}
