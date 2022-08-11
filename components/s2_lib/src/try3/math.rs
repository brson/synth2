use std::simd::{f32x16, u32x16, StdFloat, SimdFloat};
use super::units::{Unipolar, Bipolar};

pub fn line_y_value(y_rise: f32, x_run: f32, x_value: f32) -> f32 {
    let slope = y_rise / x_run;
    let y_value = slope * x_value;
    y_value
}

pub fn line_y_value_with_y_offset(y_rise: f32, x_run: f32, x_value: f32, y_offset: f32) -> f32 {
    if !cfg!(feature = "fma") {
        let y_value = line_y_value(y_rise, x_run, x_value);
        y_value + y_offset
    } else {
        let slope = y_rise / x_run;
        slope.mul_add(x_value, y_offset)
    }
}

pub fn line_y_value_x16(y_rise: f32x16, x_run: f32x16, x_value: f32x16) -> f32x16 {
    let slope = y_rise / x_run;
    let y_value = slope * x_value;
    y_value
}

pub fn line_y_value_with_y_offset_x16(
    y_rise: f32x16,
    x_run: f32x16,
    x_value: f32x16,
    y_offset: f32x16,
) -> f32x16 {
    if !cfg!(feature = "fma") {
        let y_value = line_y_value_x16(y_rise, x_run, x_value);
        y_value + y_offset
    } else {
        let slope = y_rise / x_run;
        slope.mul_add(x_value, y_offset)
    }
}

pub const fn indexes_u32<const N: usize>() -> [u32; N] {
    let mut indexes = [0; N];
    let mut index = 0;
    while index < N {
        indexes[index] = index as u32;
        index += 1;
    }
    indexes
}

pub fn zip3<A, B, C, const N: usize>(a: [A; N], b: [B; N], c: [C; N]) -> [(A, B, C); N] {
    let a_b = a.zip(b);
    let a_b_c = a_b.zip(c);
    let a_b_c = a_b_c.map(|((a, b), c)| (a, b, c));
    a_b_c
}

pub fn table_lookup(table: &[f32], offset: f32, period: f32) -> f32 {
    let offset = offset % period;

    let table_length_u32 = table.len() as u32;
    let table_length = table.len() as f32;
    let table_offset = offset * table_length / period;
    let table_offset_low = table_offset.floor();
    let table_idx1 = table_offset_low as u32;
    let table_idx2 = (table_idx1 + 1) % table_length_u32;
    let table_idx1 = table_idx1 as usize;
    let table_idx2 = table_idx2 as usize;

    let sample1 = table[table_idx1];
    let sample2 = table[table_idx2];

    {
        let y_rise = sample2 - sample1;
        let x_run = 1.0;
        let x_value = table_offset - table_offset_low;
        let y_offset = sample1;

        let sample = line_y_value_with_y_offset(y_rise, x_run, x_value, y_offset);

        sample
    }
}

pub fn table_lookup_x16(table: &[f32], offset: [f32; 16], period: [f32; 16]) -> [f32; 16] {
    let period = f32x16::from_array(period);
    let offset = f32x16::from_array(offset);
    let offset = offset % period;

    let table_length_u32 = table.len() as u32;
    let table_length_u32 = u32x16::splat(table_length_u32);
    let table_length = table.len() as f32;
    let table_length = f32x16::splat(table_length);
    let table_offset = offset * table_length / period;
    let table_offset_low = table_offset.floor();
    let one = u32x16::splat(1);
    let table_idx1 = table_offset_low.cast::<u32>();;
    let table_idx2 = (table_idx1 + one) % table_length_u32;
    let table_idx1 = table_idx1.cast::<usize>();
    let table_idx2 = table_idx2.cast::<usize>();

    let sample1 = f32x16::gather_or_default(table, table_idx1);
    let sample2 = f32x16::gather_or_default(table, table_idx2);

    {
        let y_rise = sample2 - sample1;
        let x_run = f32x16::splat(1.0);
        let x_value = table_offset - table_offset_low;
        let y_offset = sample1;

        let sample = line_y_value_with_y_offset_x16(y_rise, x_run, x_value, y_offset);

        sample.to_array()
    }
}

pub fn unipolar_table_lookup<const N: u16>(table: &[f32], value: Unipolar<N>) -> f32 {
    table_lookup(table, value.0, N as f32)
}

pub fn unipolar_table_lookup_x16<const N: u16>(table: &[f32], value: [Unipolar<N>; 16]) -> [f32; 16] {
    table_lookup_x16(table, value.map(|v| v.0), [N as f32; 16])
}

pub fn bipolar_table_lookup<const N: u16>(
    pos_table: &[f32],
    neg_table: &[f32],
    value: Bipolar<N>,
) -> f32 {
    if value.0 > 0.0 {
        table_lookup(pos_table, value.0.abs(), N as f32)
    } else {
        table_lookup(neg_table, value.0.abs(), N as f32)
    }
}

pub fn bipolar_table_lookup_x16<const N: u16>(
    pos_table: &[f32],
    neg_table: &[f32],
    value: [Bipolar<N>; 16],
) -> [f32; 16] {
    let value = value.map(|v| v.0);
    let value = f32x16::from_array(value);
    let abs_value = value.abs();
    let pos_lookup = table_lookup_x16(pos_table, abs_value.to_array(), [N as f32; 16]);
    let neg_lookup = table_lookup_x16(neg_table, abs_value.to_array(), [N as f32; 16]);
    let pos_lookup = f32x16::from_array(pos_lookup);
    let neg_lookup = f32x16::from_array(neg_lookup);

    let lookup = value.is_sign_positive().select(pos_lookup, neg_lookup);
    lookup.to_array()
}
