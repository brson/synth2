use std::simd::f32x16;
use super::units::{Unipolar, Hz, Ms, Bipolar, SampleRateKhz};
use super::static_config as sc;
use super::state as st;
use super::process;

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
    // todo: this shouldn't need to be an Option since as of now once it is some it is always some.
    // either:
    // - run all voices and default the current_frame_offset to a large number, the release_frame_offset to 0, and the fast_fade_frame_offset_to 0.
    // - come up with a way to turn voices off so it is more useful for this to be option
    current_frame_offset: Option<FrameOffset>,
    release_frame_offset: Option<FrameOffset>,
    /// Used to quickly fade out a voice when another voices is played with the
    /// same note in polyphonic mode.
    fast_fade_frame_offset: Option<FrameOffset>,
    state: st::Layer,
}

impl Voice {
    /// The 'active' voice for a note is the one that is currently being played (no note-off has been recieved).
    ///
    /// There may be multiple voices for a single note making sound, but only one active.
    fn is_active(&self) -> bool {
        self.current_frame_offset.is_some() && self.fast_fade_frame_offset.is_none()
    }
}

impl Default for Voice {
    fn default() -> Voice {
        Voice {
            note: Note(0),
            velocity: Velocity(Unipolar(0.0)),
            current_frame_offset: None,
            release_frame_offset: None,
            fast_fade_frame_offset: None,
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
        self.fade_existing_voices(note);
        let voice = self.next_voice(note);
        *voice = Voice {
            note,
            velocity,
            current_frame_offset: Some(FrameOffset(0)),
            release_frame_offset: None,
            fast_fade_frame_offset: None,
            state: st::Layer::default(),
        };
    }

    pub fn note_off(&mut self, note: Note) {
        if let Some(voice) = self.find_active_voice(note) {
            if voice.release_frame_offset.is_none() {
                voice.release_frame_offset = voice.current_frame_offset;
            } else {
                log::warn!("note {} released twice", note.0);
            }
        }
    }

    fn find_active_voice_index(&self, note: Note) -> Option<usize> {
        let mut found = None;
        for (index, voice) in self.voices.iter().enumerate() {
            if voice.note == note && voice.is_active() {
                found = Some(index);
            }
        }
        found
    }

    fn find_active_voice(&mut self, note: Note) -> Option<&mut Voice> {
        self.find_active_voice_index(note).map(|index| {
            &mut self.voices[index]
        })
    }

    /// Returns the preferred voice for the next note, without modifying it.
    ///
    /// Just picks the oldest voice.
    fn next_voice(&mut self, note: Note) -> &mut Voice {
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

    fn fade_existing_voices(&mut self, note: Note) {
        for voice in &mut self.voices {
            if voice.note == note {
                match (voice.current_frame_offset, voice.fast_fade_frame_offset) {
                    (Some(current_frame_offset), None) => {
                        voice.fast_fade_frame_offset = Some(current_frame_offset);
                    }
                    _ => { }
                }
            }
        }
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

        let mut chunks = buffer.array_chunks_mut::<16>();

        while let Some(chunk) = chunks.next() {
            self.accumulate_frames(chunk, sample_rate);
        }

        let mut remainder = chunks.into_remainder();

        if remainder.len() > 0 {
            self.accumulate_frames(remainder, sample_rate);
        }
    }

    fn accumulate_frames(&mut self,
                         buffer: &mut [f32],
                         sample_rate: SampleRateKhz) {
        debug_assert!(buffer.len() <= 16);
        let needed_frames = buffer.len();
        let mut accum = f32x16::splat(0.0);
        for voice in &mut self.voices {
            if let Some(current_frame_offset) = voice.current_frame_offset {
                let pitch = note_to_pitch(voice.note);
                let offset = current_frame_offset.0;
                let release_offset = voice.release_frame_offset.map(|v| v.0);

                let mut buf = [0.0; 16];
                process::process_layer_buf_sisd(
                    &self.config,
                    &mut voice.state,
                    pitch,
                    sample_rate,
                    offset,
                    release_offset,
                    &mut buf[..needed_frames],
                );

                let buf = f32x16::from_array(buf);
                accum += buf;

                voice.current_frame_offset = Some(FrameOffset(current_frame_offset.0.saturating_add(needed_frames as u32)));
            }
        }

        let accum = accum.to_array();
        buffer.copy_from_slice(&accum[..needed_frames]);
    }

}

fn note_to_pitch(note: Note) -> Hz {
    let note = note.0 as f32;
    let freq = 440.0 * 2_f32.powf((note - 69.0) / 12.0);
    Hz(freq)
}
