use s2_lib::try3::units::Unipolar;

const NUM_VOICES: usize = 1;

pub struct Synth {
    voices: [Voice; NUM_VOICES],
}

#[derive(Eq, PartialEq)]
#[derive(Copy, Clone)]
pub struct Note(pub u8);
pub struct Velocity(pub Unipolar<1>);
#[derive(PartialEq, PartialOrd)]
#[derive(Copy, Clone)]
pub struct FrameOffset(pub u16);

pub struct Voice {
    note: Note,
    velocity: Velocity,
    current_frame_offset: Option<FrameOffset>,
    release_frame_offset: Option<FrameOffset>,
}

impl Synth {
    pub fn new() -> Synth {
        todo!()
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
