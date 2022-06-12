use crate::math::AssertFrom;
use super::units::*;
use super::math::*;

pub struct Adsr {
    pub attack: SampleOffset,
    pub decay: SampleOffset,
    pub sustain: Unipolar<1>,
    pub release: SampleOffset,
}

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
        offset: u32,
        release_offset: Option<u32>
    ) -> Unipolar<1> {
        let attack = self.attack.0;
        let decay = self.decay.0;
        let sustain = self.sustain.0;
        let release = self.release.0;

        let offset = offset as f32;
        let decay_offset = attack;
        let sustain_offset = attack + decay;
        let release_offset =
            (release_offset
            .unwrap_or(u32::MAX) as f32)
            .max(sustain_offset);
        let end_offset = release_offset + release;

        let stage = {
            let in_attack = offset < decay_offset;
            let in_decay = !in_attack && offset < sustain_offset;
            let in_sustain = !in_attack && !in_decay && offset < release_offset;
            let in_release = !in_attack && !in_decay && !in_sustain && offset < end_offset;

            if in_attack {
                AdsrStage::Attack
            } else if in_decay {
                AdsrStage::Decay
            } else if in_sustain {
                AdsrStage::Sustain
            } else if in_release {
                AdsrStage::Release
            } else {
                AdsrStage::End
            }
        };

        match stage {
            AdsrStage::Attack => {
                let rise = 1.0;
                let run = attack;
                let x_offset = offset;
                let y_start = 0.0;
                let sample = line_y_value_with_y_offset(
                    rise, run, x_offset, y_start
                );
                Unipolar::<1>::assert_from(sample)
            }
            AdsrStage::Decay => {
                let rise = sustain - 1.0;
                let run = decay;
                let x_offset = offset - decay_offset;
                let y_start = 1.0;
                let sample = line_y_value_with_y_offset(
                    rise, run, x_offset, y_start
                );
                Unipolar::<1>::assert_from(sample)
            }
            AdsrStage::Sustain => {
                Unipolar::<1>::assert_from(sustain)
            }
            AdsrStage::Release => {
                let rise = -sustain;
                let run = release;
                let x_offset = offset - release_offset;
                let y_start = sustain;
                let sample = line_y_value_with_y_offset(
                    rise, run, x_offset, y_start
                );
                Unipolar::<1>::assert_from(sample)
            }
            AdsrStage::End => {
                Unipolar::<1>::assert_from(0.0)
            }
        }
    }
}
