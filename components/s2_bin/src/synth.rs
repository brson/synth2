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
pub struct FrameOffset(pub f32);

pub struct Voice {
    active: bool,
    note: Note,
    start_frame_offset: f32,
}

impl Synth {
    pub fn note_on(&mut self, note: Note, vel: Velocity) {
    }

    pub fn note_off(&mut self, note: Note) {
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
            if voice.note == note && voice.active {
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
        let mut available = None;
        let mut oldest: Option<&mut Voice> = None;
        for voice in &mut self.voices {
            assert!(!(voice.note == note && voice.active));
            if !voice.active {
                available = Some(voice);
            } else {
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
        }

        match (available, oldest) {
            (Some(voice), _) => voice,
            (_, Some(voice)) => voice,
            _ => panic!(),
        }
    }
}
