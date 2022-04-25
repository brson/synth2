use crate::f64::*;
use crate::math::*;
use crate::synth::*;

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
    start_sample: u32,
    sample_length: u32,
}

pub struct Sequencer {
    global_offset: u32,
    bpm: Pos64,
    synth: Synth,
}

impl Sequencer {
    pub fn new() -> Sequencer {

        let bpm = Pos64::assert_from(40.0);
        let sample_rate = SampleRateKhz(SAMPLE_RATE_KHZ);
        let freq = 80.0;
        let synth = Synth {
            sample_rate,
            osc: square_osc_hz(Hz64::assert_from(freq)),
            lpf: LowPassFilter::new(
                ZPos64::assert_from(freq),
                SAMPLE_RATE_KHZ,
            ),
            adsr: AdsrMs {
                sample_rate,
                attack: Ms64::assert_from(5.0),
                decay: Ms64::assert_from(900.0),
                sustain: ZOne64::assert_from(0.1),
                release: Ms64::assert_from(100.0),
            },
            gain: ZPos64::assert_from(1.0),
            lpf_mod_adsr: AdsrMs {
                sample_rate,
                attack: Ms64::assert_from(0.0),
                decay: Ms64::assert_from(50.0),
                sustain: ZOne64::assert_from(0.0),
                release: Ms64::assert_from(0.0),
            },
            lpf_mod_range_multiplier: 16.0,
        };
        
        Sequencer {
            global_offset: 0,
            bpm,
            synth,
        }
    }

    pub fn next_sample(&mut self) -> f64 {
        let sample_rate = f64::from(SAMPLE_RATE_KHZ);
        let bpm = f64::from(self.bpm);
        let bps = bpm / 60.0;
        let samples_per_beat = sample_rate / bps;
        let global_offset = f64::from(self.global_offset);
        let beat_offset = global_offset % samples_per_beat;
        let beat_offset = beat_offset as u32;
        let sample = self.synth.sample(beat_offset);

        let global_offset = self.global_offset.checked_add(1).expect("overflow");
        self.global_offset = global_offset;

        sample
    }
}
