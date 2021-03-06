use super::math::*;
use super::units::*;
use crate::old::math::AssertFrom;

pub struct Adsr {
    pub attack: SampleOffset,
    pub decay: SampleOffset,
    pub sustain: Unipolar<1>,
    pub release: SampleOffset,
}

#[derive(Debug)]
enum AdsrStage {
    Attack,
    Decay,
    Sustain,
    Release,
    End,
}

impl Adsr {
    pub fn sample(
        &self,
        offset: u32,                 // fixme SampleOffset
        release_offset: Option<u32>, // fixme ditto
    ) -> Unipolar<1> {
        let attack = self.attack.0;
        let decay = self.decay.0;
        let sustain = self.sustain.0;
        let release = self.release.0;

        let offset = offset as f32;
        let decay_offset = attack;
        let sustain_offset = attack + decay;
        let release_offset = release_offset.unwrap_or(u32::MAX) as f32;
        let end_offset = release_offset + release;

        let (stage, release_start_stage) = {
            let in_release = offset >= release_offset && offset < end_offset;
            let in_end = offset >= end_offset;
            let in_attack = !in_release && !in_end && offset < decay_offset;
            let in_decay = !in_release && !in_end && !in_attack && offset < sustain_offset;
            let in_sustain = !in_release && !in_end && !in_attack && !in_decay && offset < release_offset;

            let stage = if in_attack {
                AdsrStage::Attack
            } else if in_decay {
                AdsrStage::Decay
            } else if in_sustain {
                AdsrStage::Sustain
            } else if in_release {
                AdsrStage::Release
            } else {
                AdsrStage::End
            };

            let release_start_stage = if release_offset < decay_offset {
                AdsrStage::Attack
            } else if release_offset < sustain_offset {
                AdsrStage::Decay
            } else {
                AdsrStage::Sustain
            };

            (stage, release_start_stage)
        };

        let release_start_sample = match release_start_stage {
            AdsrStage::Attack => {
                let rise = 1.0;
                let run = attack;
                let x_offset = release_offset;
                let y_start = 0.0;
                let sample = line_y_value_with_y_offset(rise, run, x_offset, y_start);
                sample
            }
            AdsrStage::Decay => {
                let rise = sustain - 1.0;
                let run = decay;
                let x_offset = release_offset - decay_offset;
                let y_start = 1.0;
                let sample = line_y_value_with_y_offset(rise, run, x_offset, y_start);
                sample
            }
            AdsrStage::Sustain => sustain,
            _ => {
                if cfg!(debug) {
                    panic!()
                } else {
                    sustain
                }
            }
        };

        match stage {
            AdsrStage::Attack => {
                let rise = 1.0;
                let run = attack;
                let x_offset = offset;
                let y_start = 0.0;
                let sample = line_y_value_with_y_offset(rise, run, x_offset, y_start);
                if cfg!(debug) {
                    Unipolar::<1>::assert_from(sample)
                } else {
                    Unipolar::<1>(sample)
                }
            }
            AdsrStage::Decay => {
                let rise = sustain - 1.0;
                let run = decay;
                let x_offset = offset - decay_offset;
                let y_start = 1.0;
                let sample = line_y_value_with_y_offset(rise, run, x_offset, y_start);
                if cfg!(debug) {
                    Unipolar::<1>::assert_from(sample)
                } else {
                    Unipolar::<1>(sample)
                }
            }
            AdsrStage::Sustain => {
                let sample = sustain;
                if cfg!(debug) {
                    Unipolar::<1>::assert_from(sample)
                } else {
                    Unipolar::<1>(sample)
                }
            }
            AdsrStage::Release => {
                let rise = -release_start_sample;
                let run = release;
                let x_offset = offset - release_offset;
                let y_start = release_start_sample;
                let sample = line_y_value_with_y_offset(rise, run, x_offset, y_start);
                if cfg!(debug) {
                    Unipolar::<1>::assert_from(sample)
                } else {
                    Unipolar::<1>(sample)
                }
            }
            AdsrStage::End => {
                let sample = 0.0;
                if cfg!(debug) {
                    Unipolar::<1>::assert_from(sample)
                } else {
                    Unipolar::<1>(sample)
                }
            }
        }
    }
}
