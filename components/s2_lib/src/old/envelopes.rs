use super::math::*;
use anyhow::{anyhow, Result};

pub struct Adsr {
    pub attack: Ms64,
    pub decay: Ms64,
    pub sustain: ZOne64,
    pub release: Ms64,
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
        sample_rate: SampleRateKhz,
        offset: u32,
        release_offset: Option<u32>,
    ) -> ZOne64 {
        let attack = f64::from(self.attack.as_samples(sample_rate));
        let decay = f64::from(self.decay.as_samples(sample_rate));
        let sustain = f64::from(self.sustain);
        let release = f64::from(self.release.as_samples(sample_rate));

        let offset: f64 = offset.into();
        let decay_offset = attack;
        let sustain_offset = attack + decay;
        let release_offset =
            f64::from(release_offset.map(u32::from).unwrap_or(u32::MAX)).max(sustain_offset);
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
                let sample = line_y_value_with_y_offset(rise, run, x_offset, y_start);
                ZOne64::assert_from(sample)
            }
            AdsrStage::Decay => {
                let rise = sustain - 1.0;
                let run = decay;
                let x_offset = offset - decay_offset;
                let y_start = 1.0;
                let sample = line_y_value_with_y_offset(rise, run, x_offset, y_start);
                ZOne64::assert_from(sample)
            }
            AdsrStage::Sustain => ZOne64::assert_from(sustain),
            AdsrStage::Release => {
                let rise = -sustain;
                let run = release;
                let x_offset = offset - release_offset;
                let y_start = sustain;
                let sample = line_y_value_with_y_offset(rise, run, x_offset, y_start);
                ZOne64::assert_from(sample)
            }
            AdsrStage::End => ZOne64::assert_from(0.0),
        }
    }
}
