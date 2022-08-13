use std::simd::{f32x16, u32x16, StdFloat, SimdFloat, SimdPartialOrd};
use super::units::{Unipolar, Bipolar};
use super::math;

/// Linear-interpolated table lookup in range `[0, range)`.
///
/// The table should not include a final value for the top of the range.
///
/// Panics if `value` is out of range.
pub fn table_lookup_exclusive(
    table: &[f32],
    value: f32,
    range: f32,
) -> f32 {
    debug_assert!(table.len() > 1);
    debug_assert!(value >= 0.0);
    debug_assert!(value < range);
    debug_assert!(range > 0.0);

    let table_length_u32 = table.len() as u32;
    let table_length = table.len() as f32;
    let table_value = value * table_length / range;
    let table_value_low = table_value as u32;
    let table_idx1 = table_value_low;
    let table_idx2 = (table_idx1 + 1) % table_length_u32;
    let table_idx1 = table_idx1 as usize;
    let table_idx2 = table_idx2 as usize;
    let table_value_low = table_value_low as f32;

    let sample1 = table[table_idx1];
    let sample2 = table[table_idx2];

    {
        let y_rise = sample2 - sample1;
        let x_run = 1.0;
        let x_value = table_value - table_value_low;
        let y_offset = sample1;

        let sample = math::line_y_value_with_y_offset(y_rise, x_run, x_value, y_offset);

        sample
    }
}

pub fn table_lookup_exclusive_x16(
    table: &[f32],
    value: [f32; 16],
    range: [f32; 16],
) -> [f32; 16] {
    let range = f32x16::from_array(range);
    let value = f32x16::from_array(value);

    debug_assert!(table.len() > 1);
    debug_assert!(value.simd_ge(f32x16::splat(0.0)).all());
    debug_assert!(value.simd_lt(range).all());
    debug_assert!(range.simd_gt(f32x16::splat(0.0)).all());

    let table_length_u32 = table.len() as u32;
    let table_length_u32 = u32x16::splat(table_length_u32);
    let table_length = table.len() as f32;
    let table_length = f32x16::splat(table_length);
    let table_value = value * table_length / range;
    let table_value_low = table_value.cast::<u32>();
    let one = u32x16::splat(1);
    let table_idx1 = table_value_low;
    let table_idx2 = (table_idx1 + one) % table_length_u32;
    let table_idx1 = table_idx1.cast::<usize>();
    let table_idx2 = table_idx2.cast::<usize>();
    let table_value_low = table_value_low.cast::<f32>();
    todo!(); // rust-lang/rust#100474
    /*
    let sample1 = f32x16::gather_or_default(table, table_idx1);
    let sample2 = f32x16::gather_or_default(table, table_idx2);

    {
        let y_rise = sample2 - sample1;
        let x_run = f32x16::splat(1.0);
        let x_value = table_value - table_value_low;
        let y_offset = sample1;

        let sample = math::line_y_value_with_y_offset_x16(y_rise, x_run, x_value, y_offset);

        sample.to_array()
    }*/
}

/// Linear-interpolated table lookup in range `[0, range]`.
///
/// The table should include a final value for the top of the range.
///
/// Panics if `value` is out of range.
pub fn table_lookup_inclusive(
    table: &[f32],
    value: f32,
    range: f32,
) -> f32 {
    debug_assert!(table.len() > 1);
    debug_assert!(value >= 0.0);
    debug_assert!(value <= range);
    debug_assert!(range > 0.0);

    // The only differences between this and the "exclusive" calculation is
    //
    // - discard the final table element in calculations based on length
    // - don't modulus table_idx2 by the table length

    let table_length = table.len().saturating_sub(1);
    let table_length_u32 = table_length as u32;
    let table_length = table_length as f32;
    let table_value = value * table_length / range;
    let table_value_low = table_value as u32;
    let table_idx1 = table_value_low;
    let table_idx2 = (table_idx1 + 1);
    let table_idx1 = table_idx1 as usize;
    let table_idx2 = table_idx2 as usize;
    let table_value_low = table_value_low as f32;

    let sample1 = table[table_idx1];
    let sample2 = table[table_idx2];

    {
        let y_rise = sample2 - sample1;
        let x_run = 1.0;
        let x_value = table_value - table_value_low;
        let y_offset = sample1;

        let sample = math::line_y_value_with_y_offset(y_rise, x_run, x_value, y_offset);

        sample
    }
}

pub fn table_lookup_inclusive_x16(
    table: &[f32],
    value: [f32; 16],
    range: [f32; 16],
) -> [f32; 16] {
    let range = f32x16::from_array(range);
    let value = f32x16::from_array(value);

    debug_assert!(table.len() > 1);
    debug_assert!(value.simd_ge(f32x16::splat(0.0)).all());
    debug_assert!(value.simd_le(range).all());
    debug_assert!(range.simd_gt(f32x16::splat(0.0)).all());

    let table_length = table.len().saturating_sub(1);
    let table_length_u32 = table_length as u32;
    let table_length_u32 = u32x16::splat(table_length_u32);
    let table_length = table_length as f32;
    let table_length = f32x16::splat(table_length);
    let table_value = value * table_length / range;
    let table_value_low = table_value.cast::<u32>();
    let one = u32x16::splat(1);
    let table_idx1 = table_value_low;
    let table_idx2 = (table_idx1 + one);
    let table_idx1 = table_idx1.cast::<usize>();
    let table_idx2 = table_idx2.cast::<usize>();
    let table_value_low = table_value_low.cast::<f32>();
    todo!(); // rust-lang/rust#100474
    /*
    let sample1 = f32x16::gather_or_default(table, table_idx1);
    let sample2 = f32x16::gather_or_default(table, table_idx2);

    {
        let y_rise = sample2 - sample1;
        let x_run = f32x16::splat(1.0);
        let x_value = table_value - table_value_low;
        let y_offset = sample1;

        let sample = math::line_y_value_with_y_offset_x16(y_rise, x_run, x_value, y_offset);

        sample.to_array()
    }*/
}

/// Linear-interpolated table lookup in range `[0, range)` with modulus.
///
/// A modulus of `range` is applied to `value` before calling [`table_lookup_exclusive`].
///
/// Panics if `value` is out of range.
pub fn table_lookup_periodic(
    table: &[f32],
    value: f32,
    range: f32,
) -> f32 {
    table_lookup_exclusive(table, value % range, range)
}

pub fn table_lookup_periodic_x16(
    table: &[f32],
    value: [f32; 16],
    range: [f32; 16],
) -> [f32; 16] {
    let value = {
        let value = f32x16::from_array(value);
        let range = f32x16::from_array(range);
        let value = value % range;
        value.to_array()
    };
    table_lookup_exclusive_x16(table, value, range)
}

/// Linear-interpolated table lookup in the range `[0, N]`.
pub fn unipolar_table_lookup<const N: u16>(
    table: &[f32],
    value: Unipolar<N>,
) -> f32 {
    table_lookup_inclusive(table, value.0, N as f32)
}

pub fn unipolar_table_lookup_x16<const N: u16>(
    table: &[f32],
    value: [Unipolar<N>; 16],
) -> [f32; 16] {
    table_lookup_inclusive_x16(table, value.map(|v| v.0), [N as f32; 16])
}

/// Linear-interpolated table lookup in the range `[-N, +N]`.
pub fn bipolar_table_lookup<const N: u16>(
    pos_table: &[f32],
    neg_table: &[f32],
    value: Bipolar<N>,
) -> f32 {
    if value.0 > 0.0 {
        table_lookup_inclusive(pos_table, value.0.abs(), N as f32)
    } else {
        table_lookup_inclusive(neg_table, value.0.abs(), N as f32)
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
    let pos_lookup = table_lookup_inclusive_x16(pos_table, abs_value.to_array(), [N as f32; 16]);
    let neg_lookup = table_lookup_inclusive_x16(neg_table, abs_value.to_array(), [N as f32; 16]);
    let pos_lookup = f32x16::from_array(pos_lookup);
    let neg_lookup = f32x16::from_array(neg_lookup);

    let lookup = value.is_sign_positive().select(pos_lookup, neg_lookup);
    lookup.to_array()
}

mod tests {
    use super::*;

    #[test]
    fn test_table_lookup() {
        let table = &[0.0, 1.0, 2.0, 3.0];
        let v = table_lookup_exclusive(table, 0.0, 4.0);
        assert_eq!(v, 0.0);
        todo!()
    }
}
