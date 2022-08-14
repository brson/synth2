use s2_lib::try3::units::{Unipolar, Hz, Ms, Bipolar, SampleRateKhz};
use s2_lib::try3::static_config as sc;
use s2_lib::try3::state as st;

const NUM_VOICES: usize = 8;

pub struct Synth {
    config: sc::Layer,
    voices: [Voice; NUM_VOICES],
}

#[derive(Eq, PartialEq)]
#[derive(Copy, Clone)]
pub struct Note(pub u8);
#[derive(Copy, Clone)]
pub struct Velocity(pub Unipolar<1>);
#[derive(PartialEq, PartialOrd)]
#[derive(Copy, Clone)]
pub struct FrameOffset(pub u32);

#[derive(Copy, Clone)]
pub struct Voice {
    note: Note,
    velocity: Velocity,
    current_frame_offset: Option<FrameOffset>,
    release_frame_offset: Option<FrameOffset>,
    state: st::Layer,
}

impl Default for Voice {
    fn default() -> Voice {
        Voice {
            note: Note(0),
            velocity: Velocity(Unipolar(0.0)),
            current_frame_offset: None,
            release_frame_offset: None,
            state: st::Layer::default(),
        }
    }
}

impl Synth {
    pub fn new() -> Synth {
        Synth {
            config: Synth::default_config(),
            voices: [Voice::default(); NUM_VOICES],
        }
    }

    pub fn note_on(&mut self, note: Note, velocity: Velocity) {
        let voice = self.start_voice(note);
        *voice = Voice {
            note,
            velocity,
            current_frame_offset: Some(FrameOffset(0)),
            release_frame_offset: None,
            state: st::Layer::default(),
        };
    }

    pub fn note_off(&mut self, note: Note) {
        if let Some(voice) = self.find_voice(note) {
            if voice.release_frame_offset.is_none() {
                voice.release_frame_offset = voice.current_frame_offset;
            } else {
                log::warn!("note {} released twice", note.0);
            }
        }
    }

    fn start_voice(&mut self, note: Note) -> &mut Voice {
        if let Some(index) = self.find_voice_index(note) {
            log::debug!("using existing voice index {} for note {}", index, note.0);
            &mut self.voices[index]
        } else {
            self.add_voice(note)
        }
    }

    fn find_voice_index(&self, note: Note) -> Option<usize> {
        let mut found = None;
        for (index, voice) in self.voices.iter().enumerate() {
            if voice.note == note {
                found = Some(index);
            }
        }
        found
    }

    fn find_voice(&mut self, note: Note) -> Option<&mut Voice> {
        self.find_voice_index(note).map(|index| {
            &mut self.voices[index]
        })
    }

    fn add_voice(&mut self, note: Note) -> &mut Voice {
        let mut oldest: Option<&mut Voice> = None;
        let mut oldest_index = 0;
        for (index, voice) in self.voices.iter_mut().enumerate() {
            if let Some(current_oldest) = oldest {
                let this_frame_offset = voice.current_frame_offset.unwrap_or(FrameOffset(u32::max_value()));
                let oldest_frame_offset = current_oldest.current_frame_offset.unwrap_or(FrameOffset(u32::max_value()));
                if this_frame_offset > oldest_frame_offset {
                    oldest = Some(voice);
                    oldest_index = index;
                } else {
                    oldest = Some(current_oldest);
                }
            } else {
                oldest = Some(voice);
            }
        }
        log::debug!("using new voice index {} for note {}", oldest_index, note.0);
        oldest.unwrap()
    }
}

impl Synth {

    fn default_config() -> sc::Layer {
        sc::Layer {
            osc: sc::Oscillator::Saw,
            lpf: sc::LowPassFilter {
                freq: Hz(100.0),
            },
            amp_env: sc::Adsr {
                attack: Ms(200.0),
                decay: Ms(200.0),
                sustain: Unipolar(0.5),
                release: Ms(600.0)
            },
            mod_env: sc::Adsr {
                attack: Ms(800.0),
                decay: Ms(800.0),
                sustain: Unipolar(0.1),
                release: Ms(200.0)
            },
            modulations: sc::Modulations {
                mod_env_to_osc_freq: Bipolar(0.0),
                mod_env_to_lpf_freq: Bipolar(0.0),
            },
        }
    }

    pub fn sample(&mut self,
                  buffer: &mut [f32],
                  sample_rate: SampleRateKhz) {
        for index in 0..buffer.len() {
            let mut sample_accum = 0.0;
            for voice in &mut self.voices {
                if let Some(current_frame_offset) = voice.current_frame_offset {
                    let pitch = note_to_pitch(voice.note);
                    let offset = current_frame_offset.0;
                    let release_offset = voice.release_frame_offset.map(|v| v.0);
                    let sample = s2_lib::try3::process::process_layer(
                        &self.config,
                        &mut voice.state,
                        pitch,
                        sample_rate,
                        offset,
                        release_offset,
                    );
                    sample_accum += sample;
                    voice.current_frame_offset = Some(FrameOffset(current_frame_offset.0.saturating_add(1)));
                }
            }

            buffer[index] = sample_accum;
        }
    }
}

fn note_to_pitch(note: Note) -> Hz {
    let note = note.0 as f32;
    let freq = 440.0 * 2_f32.powf((note - 69.0) / 12.0);
    Hz(freq)
}
