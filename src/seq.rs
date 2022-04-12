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
    sample_rate: SampleRateKhz,
    osc: OscillatorHz,
    lpf: LowPassFilter,
    adsr: AdsrMs,
    gain: ZPos64,
    lpf_mod_adsr: AdsrMs,
    lpf_mod_range_multiplier: f64, // 0 = no mod, 1 = 1 octave
}

impl Synth {
    fn sample(&mut self, offset: Snat32) -> f64 {
        let release = Some(Ms64::assert_from(50.0));

        let mut modulated_lpf = {
            let lpf_mod_sample = self.lpf_mod_adsr.sample(offset, release);
            let lpf_mod_sample = f64::from(lpf_mod_sample);
            let lpf_freq = f64::from(self.lpf.freq);
            let addtl_lpf_freq = lpf_freq * lpf_mod_sample * self.lpf_mod_range_multiplier;
            let lpf_freq = lpf_freq + addtl_lpf_freq;
            let lpf_freq = ZPos64::assert_from(lpf_freq);
            let lpf = self.lpf.modulate(lpf_freq);
            lpf
        };

        let sample = f64::from(self.osc.sample(offset));
        let sample = modulated_lpf.process(sample);
        let release_offset = Ms64::assert_from(1000.0).as_samples(self.sample_rate);
        let adsr_sample = self.adsr.sample(offset, release);

        let sample = sample * f64::from(adsr_sample);
        let sample = sample * f64::from(self.gain);
        sample
    }
}

pub struct Sequencer {
    global_offset: Snat32,
    bpm: Pos64,
    synth: Synth,
}

impl Sequencer {
    pub fn new() -> Sequencer {

        let bpm = Pos64::assert_from(40.0);
        let sample_rate = SampleRateKhz(Snat32::assert_from(SAMPLE_RATE_KHZ));
        let freq = 80.0;
        let synth = Synth {
            sample_rate,
            osc: square_osc_hz(Hz64::assert_from(freq)),
            lpf: LowPassFilter::new(
                ZPos64::assert_from(freq),
                Snat32::assert_from(SAMPLE_RATE_KHZ),
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
            global_offset: Snat32::assert_from(0),
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
        let beat_offset = beat_offset as i32;
        let beat_offset = Snat32::assert_from(beat_offset);
        let sample = self.synth.sample(beat_offset);

        let global_offset = i32::assert_from(self.global_offset);
        let global_offset = global_offset.checked_add(1).expect("overflow");
        self.global_offset = Snat32::assert_from(global_offset);

        sample
    }
}
