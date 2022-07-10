use s2_lib::try3::units::{Unipolar, Hz, Ms, Bipolar, SampleRateKhz};
use s2_lib::try3::static_config as sc;

const NUM_VOICES: usize = 1;

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
pub struct FrameOffset(pub u16);

#[derive(Copy, Clone)]
pub struct Voice {
    note: Note,
    velocity: Velocity,
    current_frame_offset: Option<FrameOffset>,
    release_frame_offset: Option<FrameOffset>,
}

impl Default for Voice {
    fn default() -> Voice {
        Voice {
            note: Note(0),
            velocity: Velocity(Unipolar(0.0)),
            current_frame_offset: None,
            release_frame_offset: None,
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
        for voice in &mut self.voices {
            if let Some(current_oldest) = oldest {
                if voice.current_frame_offset > current_oldest.current_frame_offset {
                    oldest = Some(voice)
                } else {
                    oldest = Some(current_oldest)
                }
            } else {
                oldest = Some(voice)
            }
        }
        oldest.unwrap()
    }
}

impl Synth {

    fn default_config() -> sc::Layer {
        sc::Layer {
            osc: sc::Oscillator::Saw,
            lpf: sc::LowPassFilter {
                freq: Hz(20_000.0),
            },
            amp_env: sc::Adsr {
                attack: Ms(1.0),
                decay: Ms(0.0),
                sustain: Unipolar(1.0),
                release: Ms(50.0)
            },
            mod_env: sc::Adsr {
                attack: Ms(0.0),
                decay: Ms(50.0),
                sustain: Unipolar(0.0),
                release: Ms(10.0)
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
        todo!()
    }
}
