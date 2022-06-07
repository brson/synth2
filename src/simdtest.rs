use core_simd::{f32x4, u8x4, mask32x4};
use core_simd::{SimdFloat, SimdPartialEq, Mask};

pub struct OscillatorX4 {
    kind: u8x4,
    period: f32x4,
}

const OSC_KIND_SQUARE: u8 = 1;
const OSC_KIND_SAW: u8 = 2;
const OSC_KIND_TRI: u8 = 3;

impl OscillatorX4 {
    pub fn sample(&self, offset: u32) -> f32x4 {
        let period = self.period;
        let offset = offset as f32;
        let offset = f32x4::splat(offset);
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

            line_y_value_with_y_offset(
                x_rise, x_run, x_value, y_offset
            )
        };

        let tri_sample = {
            let two = f32x4::splat(2.0);
            let half_period = period / two;

            let tri_first_half_sample = {
                let x_rise = f32x4::splat(-2.0);
                let x_run = half_period;
                let x_value = offset;
                let y_offset = f32x4::splat(1.0);

                line_y_value_with_y_offset(
                    x_rise, x_run, x_value, y_offset
                )
            };
            let tri_second_half_sample = {
                let x_rise = f32x4::splat(2.0);
                let x_run = half_period;
                let x_value = offset - half_period;
                let y_offset = f32x4::splat(-1.0);

                line_y_value_with_y_offset(
                    x_rise, x_run, x_value, y_offset
                )
            };

            let in_first_half: mask32x4 = (offset - half_period).is_sign_negative();

            in_first_half.select(tri_first_half_sample, tri_second_half_sample)
        };

        let kind = self.kind;

        let is_kind_square = kind.simd_eq(u8x4::splat(OSC_KIND_SQUARE));
        let is_kind_saw = kind.simd_eq(u8x4::splat(OSC_KIND_SAW));
        let is_kind_tri = kind.simd_eq(u8x4::splat(OSC_KIND_TRI));

        panic!()
    }
}

pub fn line_y_value(
    y_rise: f32x4,
    x_run: f32x4,
    x_value: f32x4,
) -> f32x4 {
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
