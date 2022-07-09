use s2_lib::try3::units::Unipolar;

pub struct Synth {
}

pub struct Note(pub u8);
pub struct Velocity(pub Unipolar<1>);

impl Synth {
    pub fn note_on(note: Note, vel: Velocity) {
    }

    pub fn note_off(note: Note) {
    }
}
