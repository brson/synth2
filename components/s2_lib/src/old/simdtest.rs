use core_simd::{f32x4, mask32x4, u32x4, u8x4};
use core_simd::{Mask, SimdFloat, SimdPartialEq, SimdPartialOrd};

pub struct OscillatorX4 {
    kind: u8x4,
    period: f32x4,
}

const OSC_KIND_SQUARE: u8 = 1;
const OSC_KIND_SAW: u8 = 2;
const OSC_KIND_TRI: u8 = 3;

impl OscillatorX4 {
    pub fn sample(&self, offset: u32x4) -> f32x4 {
        let kind = self.kind;
        let period = self.period;

        let offset = offset.to_array();
        let offset = offset.map(|v| v as f32);
        let offset = f32x4::from_array(offset);
        let offset = offset % period;

        let square_sample = {
            let two = f32x4::splat(2.0);
            let half_period = period / two;
            if offset < half_period {
                f32x4::splat(1.0)
            } else {
                f32x4::splat(-1.0)
            }
        };

        let saw_sample = {
            let x_rise = f32x4::splat(-2.0);
            let x_run = period;
            let x_value = offset;
            let y_offset = f32x4::splat(1.0);

            line_y_value_with_y_offset(x_rise, x_run, x_value, y_offset)
        };

        let tri_sample = {
            let two = f32x4::splat(2.0);
            let half_period = period / two;

            let tri_first_half_sample = {
                let x_rise = f32x4::splat(-2.0);
                let x_run = half_period;
                let x_value = offset;
                let y_offset = f32x4::splat(1.0);

                line_y_value_with_y_offset(x_rise, x_run, x_value, y_offset)
            };
            let tri_second_half_sample = {
                let x_rise = f32x4::splat(2.0);
                let x_run = half_period;
                let x_value = offset - half_period;
                let y_offset = f32x4::splat(-1.0);

                line_y_value_with_y_offset(x_rise, x_run, x_value, y_offset)
            };

            let in_first_half = offset.simd_lt(half_period);

            in_first_half.select(tri_first_half_sample, tri_second_half_sample)
        };

        let is_kind_square = kind.simd_eq(u8x4::splat(OSC_KIND_SQUARE));
        let is_kind_saw = kind.simd_eq(u8x4::splat(OSC_KIND_SAW));
        let is_kind_tri = kind.simd_eq(u8x4::splat(OSC_KIND_TRI));

        let is_kind_square = mask32x4::from(is_kind_square);
        let is_kind_saw = mask32x4::from(is_kind_saw);
        let is_kind_tri = mask32x4::from(is_kind_tri);

        let sample = f32x4::splat(0.0);
        let sample = is_kind_square.select(square_sample, sample);
        let sample = is_kind_saw.select(saw_sample, sample);
        let sample = is_kind_tri.select(tri_sample, sample);

        sample
    }
}

pub fn line_y_value(y_rise: f32x4, x_run: f32x4, x_value: f32x4) -> f32x4 {
    let slope = y_rise / x_run;
    let y_value = slope * x_value;
    y_value
}

pub fn line_y_value_with_y_offset(
    y_rise: f32x4,
    x_run: f32x4,
    x_value: f32x4,
    y_offset: f32x4,
) -> f32x4 {
    let y_value = line_y_value(y_rise, x_run, x_value);
    y_value + y_offset
}

pub struct AdsrX4 {
    pub attack: f32x4,  // +samples
    pub decay: f32x4,   // +samples
    pub sustain: f32x4, // [0, 1]
    pub release: f32x4, // +samples
}

impl AdsrX4 {
    pub fn sample(&self, offset: u32x4, release_offset: Option<u32>) -> f32x4 {
        let attack = self.attack;
        let decay = self.decay;
        let sustain = self.sustain;
        let release = self.release;

        let offset = offset.to_array();
        let offset = offset.map(|v| v as f32);
        let offset = f32x4::from_array(offset);

        let decay_offset = attack;
        let sustain_offset = attack + decay;
        let release_offset = release_offset.unwrap_or(u32::MAX) as f32;
        let release_offset = f32x4::splat(release_offset);
        let release_offset = release_offset.simd_max(sustain_offset);
        let end_offset = release_offset + release;

        let in_attack = offset.simd_lt(decay_offset);
        let in_decay = !in_attack & offset.simd_lt(sustain_offset);
        let in_sustain = !in_attack & !in_decay & offset.simd_lt(release_offset);
        let in_release = !in_attack & !in_decay & !in_sustain & offset.simd_lt(end_offset);
        let in_end = !in_attack & !in_decay & !in_sustain & !in_release;

        let attack_sample = {
            let rise = f32x4::splat(1.0);
            let run = attack;
            let x_offset = offset;
            let y_start = f32x4::splat(0.0);
            line_y_value_with_y_offset(rise, run, x_offset, y_start)
        };

        let decay_sample = {
            let rise = sustain - f32x4::splat(1.0);
            let run = decay;
            let x_offset = offset - decay_offset;
            let y_start = f32x4::splat(1.0);
            line_y_value_with_y_offset(rise, run, x_offset, y_start)
        };

        let sustain_sample = { sustain };

        let release_sample = {
            let rise = -sustain;
            let run = release;
            let x_offset = offset - release_offset;
            let y_start = sustain;
            line_y_value_with_y_offset(rise, run, x_offset, y_start)
        };

        let end_sample = { f32x4::splat(0.0) };

        let sample = f32x4::splat(0.0);
        let sample = in_attack.select(attack_sample, sample);
        let sample = in_decay.select(decay_sample, sample);
        let sample = in_sustain.select(sustain_sample, sample);
        let sample = in_release.select(release_sample, sample);
        let sample = in_end.select(end_sample, sample);

        sample
    }
}
