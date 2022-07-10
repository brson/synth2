use s2_lib::try3::units::Unipolar;

const NUM_VOICES: usize = 1;

pub struct Synth {
    voices: [Voice; NUM_VOICES],
    global_frame_offset: FrameOffset,
}

#[derive(Eq, PartialEq)]
#[derive(Copy, Clone)]
pub struct Note(pub u8);
pub struct Velocity(pub Unipolar<1>);
#[derive(PartialEq, PartialOrd)]
#[derive(Copy, Clone)]
pub struct FrameOffset(pub f32);

pub struct Voice {
    note: Note,
    velocity: Velocity,
    start_frame_offset: FrameOffset,
    release_frame_offset: Option<FrameOffset>,
}

impl Synth {
    pub fn note_on(&mut self, note: Note, vel: Velocity) {
    }

    pub fn note_off(&mut self, note: Note) {
        let global_frame_offset = self.global_frame_offset;
        if let Some(voice) = self.find_voice(note) {
            if voice.release_frame_offset.is_none() {
                voice.release_frame_offset = Some(global_frame_offset);
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
                if voice.start_frame_offset < current_oldest.start_frame_offset {
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
