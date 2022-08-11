pub use basic::*;

pub mod basic {
    use std::simd::{f32x16, u32x16, SimdPartialOrd, StdFloat};
    use super::super::math::*;
    use super::super::units::*;

    pub struct SquareOscillator {
        pub period: SampleOffset,
    }

    pub struct SquareOscillatorX16 {
        pub period: [SampleOffset; 16],
    }

    pub struct SawOscillator {
        pub period: SampleOffset,
    }

    pub struct SawOscillatorX16 {
        pub period: [SampleOffset; 16],
    }

    pub struct TriangleOscillator {
        pub period: SampleOffset,
    }

    pub struct TriangleOscillatorX16 {
        pub period: [SampleOffset; 16],
    }

    pub struct TableOscillator<'this> {
        pub table: &'this [f32],
        pub period: SampleOffset,
    }

    pub struct TableOscillatorX16<'this> {
        pub table: &'this [f32],
        pub period: [SampleOffset; 16],
    }

    impl SquareOscillator {
        pub fn sample(&self, offset: SampleOffset) -> Bipolar<1> {
            let period = self.period.0;
            let offset = offset.0;
            let offset = offset % period;

            let half_period = period / 2.0;
            let sample = if offset < half_period { 1.0 } else { -1.0 };

            Bipolar(sample)
        }
    }

    impl SquareOscillatorX16 {
        pub fn sample(&self, offset: [SampleOffset; 16]) -> [Bipolar<1>; 16] {
            let period = self.period.map(|p| p.0);
            let period = f32x16::from_array(period);
            let offset = offset.map(|o| o.0);
            let offset = f32x16::from_array(offset);
            let offset = offset % period;

            let two = f32x16::splat(2.0);
            let half_period = period / two;
            let one = f32x16::splat(1.0);
            let n_one = f32x16::splat(-1.0);
            let offset_lt_half_period = offset.simd_lt(half_period);
            let sample = offset_lt_half_period.select(one, n_one);

            let sample = sample.to_array();
            let sample = sample.map(|s| Bipolar(s));

            sample
        }
    }

    impl SawOscillator {
        pub fn sample(&self, offset: SampleOffset) -> Bipolar<1> {
            let period = self.period.0;
            let offset = offset.0;
            let offset = offset % period;

            let y_rise = -2.0;
            let x_run = period;
            let x_value = offset;
            let y_offset = 1.0;

            let sample = line_y_value_with_y_offset(y_rise, x_run, x_value, y_offset);

            Bipolar(sample)
        }
    }

    impl SawOscillatorX16 {
        pub fn sample(&self, offset: [SampleOffset; 16]) -> [Bipolar<1>; 16] {
            let period = self.period.map(|p| p.0);
            let period = f32x16::from_array(period);
            let offset = offset.map(|o| o.0);
            let offset = f32x16::from_array(offset);
            let offset = offset % period;

            let y_rise = f32x16::splat(-2.0);
            let x_run = period;
            let x_value = offset;
            let y_offset = f32x16::splat(1.0);

            let sample = line_y_value_with_y_offset_x16(y_rise, x_run, x_value, y_offset);

            let sample = sample.to_array();
            let sample = sample.map(|s| Bipolar(s));

            sample
        }
    }

    impl TriangleOscillator {
        pub fn sample(&self, offset: SampleOffset) -> Bipolar<1> {
            let period = self.period.0;
            let offset = offset.0;
            let offset = offset % period;

            let half_period = period / 2.0;
            let sample = if offset < half_period {
                let y_rise = -2.0;
                let x_run = half_period;
                let x_value = offset;
                let y_offset = 1.0;

                line_y_value_with_y_offset(y_rise, x_run, x_value, y_offset)
            } else {
                let y_rise = 2.0;
                let x_run = half_period;
                let x_value = offset - half_period;
                let y_offset = -1.0;

                line_y_value_with_y_offset(y_rise, x_run, x_value, y_offset)
            };

            Bipolar(sample)
        }
    }

    impl TriangleOscillatorX16 {
        pub fn sample(&self, offset: [SampleOffset; 16]) -> [Bipolar<1>; 16] {
            let period = self.period.map(|p| p.0);
            let period = f32x16::from_array(period);
            let offset = offset.map(|o| o.0);
            let offset = f32x16::from_array(offset);
            let offset = offset % period;

            let two = f32x16::splat(2.0);
            let half_period = period / two;
            let sample_first_half = {
                let y_rise = f32x16::splat(-2.0);
                let x_run = half_period;
                let x_value = offset;
                let y_offset = f32x16::splat(1.0);

                line_y_value_with_y_offset_x16(y_rise, x_run, x_value, y_offset)
            };
            let sample_second_half = {
                let y_rise = f32x16::splat(2.0);
                let x_run = half_period;
                let x_value = offset - half_period;
                let y_offset = f32x16::splat(-1.0);

                line_y_value_with_y_offset_x16(y_rise, x_run, x_value, y_offset)
            };
            let offset_lt_half_period = offset.simd_lt(half_period);

            let sample = offset_lt_half_period.select(sample_first_half, sample_second_half);

            let sample = sample.to_array();
            let sample = sample.map(|s| Bipolar(s));

            sample
        }
    }

    impl<'this> TableOscillator<'this> {
        pub fn sample(&self, offset: SampleOffset) -> Bipolar<1> {
            let period = self.period.0;
            let offset = offset.0;
            let offset = offset % period;

            let table_length = self.table.len() as f32;
            let table_offset = offset * table_length / period;
            let table_offset_low = table_offset.floor();
            let table_idx1 = table_offset_low as usize;
            let table_idx2 = (table_idx1 + 1) % self.table.len();

            let sample1 = self.table[table_idx1];
            let sample2 = self.table[table_idx2];

            {
                let y_rise = sample2 - sample1;
                let x_run = 1.0;
                let x_value = table_offset - table_offset_low;
                let y_offset = sample1;

                let sample = line_y_value_with_y_offset(y_rise, x_run, x_value, y_offset);

                Bipolar(sample)
            }
        }
    }

    impl<'this> TableOscillatorX16<'this> {
        pub fn sample(&self, offset: [SampleOffset; 16]) -> [Bipolar<1>; 16] {
            let period = self.period.map(|p| p.0);
            let period = f32x16::from_array(period);
            let offset = offset.map(|o| o.0);
            let offset = f32x16::from_array(offset);
            let offset = offset % period;

            let table_length_u32 = self.table.len() as u32;
            let table_length_u32 = u32x16::splat(table_length_u32);
            let table_length = self.table.len() as f32;
            let table_length = f32x16::splat(table_length);
            let table_offset = offset * table_length / period;
            let table_offset_low = table_offset.floor();
            let one = u32x16::splat(1);
            let table_idx1 = table_offset_low.cast::<u32>();
            let table_idx2 = (table_idx1 + one) % table_length_u32;
            let table_idx1 = table_idx1.cast::<usize>();
            let table_idx2 = table_idx2.cast::<usize>();

            let sample1 = f32x16::gather_or_default(self.table, table_idx1);
            let sample2 = f32x16::gather_or_default(self.table, table_idx2);

            {
                let y_rise = sample2 - sample1;
                let x_run = f32x16::splat(1.0);
                let x_value = table_offset - table_offset_low;
                let y_offset = sample1;

                let sample = line_y_value_with_y_offset_x16(y_rise, x_run, x_value, y_offset);

                let sample = sample.to_array();
                let sample = sample.map(|s| Bipolar(s));

                sample
            }
        }
    }
}

pub mod phased {
    use super::super::units::*;
    use super::basic;
    use std::simd::{f32x16, StdFloat};

    fn phased_offset(period: SampleOffset, phase: Unipolar<1>, offset: SampleOffset) -> SampleOffset {
        if !cfg!(feature = "fma") {
            let phase_offset = period.0 * phase.0;
            SampleOffset(offset.0 + phase_offset)
        } else {
            let offset = period.0.mul_add(phase.0, offset.0);
            SampleOffset(offset)
        }
    }

    fn phased_offset_x16(
        period: [SampleOffset; 16],
        phase: [Unipolar<1>; 16],
        offset: [SampleOffset; 16],
    ) -> [SampleOffset; 16] {
        let period = period.map(|p| p.0);
        let period = f32x16::from_array(period);
        let phase = phase.map(|p| p.0);
        let phase = f32x16::from_array(phase);
        let offset = offset.map(|o| o.0);
        let offset = f32x16::from_array(offset);

        if !cfg!(feature = "fma") {
            let phase_offset = period * phase;
            let new_offset = offset + phase_offset;
            let new_offset = new_offset.to_array();
            new_offset.map(|o| SampleOffset(o))
        } else {
            let new_offset = period.mul_add(phase, offset);
            let new_offset = new_offset.to_array();
            new_offset.map(|o| SampleOffset(o))
        }
    }

    pub struct SquareOscillator {
        pub period: SampleOffset,
        pub phase: Unipolar<1>,
    }

    impl SquareOscillator {
        pub fn sample(&self, offset: SampleOffset) -> Bipolar<1> {
            let offset = phased_offset(self.period, self.phase, offset);
            let basic_osc = basic::SquareOscillator {
                period: self.period,
            };
            basic_osc.sample(offset)
        }
    }

    pub struct SquareOscillatorX16 {
        pub period: [SampleOffset; 16],
        pub phase: [Unipolar<1>; 16],
    }

    impl SquareOscillatorX16 {
        pub fn sample(&self, offset: [SampleOffset; 16]) -> [Bipolar<1>; 16] {
            let offset = phased_offset_x16(self.period, self.phase, offset);
            let basic_osc = basic::SquareOscillatorX16 {
                period: self.period,
            };
            basic_osc.sample(offset)
        }
    }

    pub struct SawOscillator {
        pub period: SampleOffset,
        pub phase: Unipolar<1>,
    }

    impl SawOscillator {
        pub fn sample(&self, offset: SampleOffset) -> Bipolar<1> {
            let offset = phased_offset(self.period, self.phase, offset);
            let basic_osc = basic::SawOscillator {
                period: self.period,
            };
            basic_osc.sample(offset)
        }
    }

    pub struct SawOscillatorX16 {
        pub period: [SampleOffset; 16],
        pub phase: [Unipolar<1>; 16],
    }

    impl SawOscillatorX16 {
        pub fn sample(&self, offset: [SampleOffset; 16]) -> [Bipolar<1>; 16] {
            let offset = phased_offset_x16(self.period, self.phase, offset);
            let basic_osc = basic::SawOscillatorX16 {
                period: self.period,
            };
            basic_osc.sample(offset)
        }
    }

    pub struct TriangleOscillator {
        pub period: SampleOffset,
        pub phase: Unipolar<1>,
    }

    impl TriangleOscillator {
        pub fn sample(&self, offset: SampleOffset) -> Bipolar<1> {
            let offset = phased_offset(self.period, self.phase, offset);
            let basic_osc = basic::TriangleOscillator {
                period: self.period,
            };
            basic_osc.sample(offset)
        }
    }

    pub struct TriangleOscillatorX16 {
        pub period: [SampleOffset; 16],
        pub phase: [Unipolar<1>; 16],
    }

    impl TriangleOscillatorX16 {
        pub fn sample(&self, offset: [SampleOffset; 16]) -> [Bipolar<1>; 16] {
            let offset = phased_offset_x16(self.period, self.phase, offset);
            let basic_osc = basic::TriangleOscillatorX16 {
                period: self.period,
            };
            basic_osc.sample(offset)
        }
    }

    pub struct TableOscillator<'this> {
        pub table: &'this [f32],
        pub period: SampleOffset,
        pub phase: Unipolar<1>,
    }

    impl<'this> TableOscillator<'this> {
        pub fn sample(&self, offset: SampleOffset) -> Bipolar<1> {
            let offset = phased_offset(self.period, self.phase, offset);
            let basic_osc = basic::TableOscillator {
                table: self.table,
                period: self.period,
            };
            basic_osc.sample(offset)
        }
    }

    pub struct TableOscillatorX16<'this> {
        pub table: &'this [f32],
        pub period: [SampleOffset; 16],
        pub phase: [Unipolar<1>; 16],
    }

    impl<'this> TableOscillatorX16<'this> {
        pub fn sample(&self, offset: [SampleOffset; 16]) -> [Bipolar<1>; 16] {
            let offset = phased_offset_x16(self.period, self.phase, offset);
            let basic_osc = basic::TableOscillatorX16 {
                table: self.table,
                period: self.period,
            };
            basic_osc.sample(offset)
        }
    }
}

/// Stateful oscillators that can be frequency modulated.
///
/// They always resume sampling at the same phase as the previous sample, even
/// if the frequency has changed.
///
/// - <https://dsp.stackexchange.com/questions/2349/help-with-algorithm-for-modulating-oscillator-pitch-using-lfo>
/// - <https://dsp.stackexchange.com/questions/971/how-to-create-a-sine-wave-generator-that-can-smoothly-transition-between-frequen>
pub mod phase_accumulating {
    use super::super::units::*;
    use super::phased;

    #[derive(Default)]
    #[derive(Copy, Clone)]
    pub struct OscillatorState {
        pub phase_accum: Option<Unipolar<1>>,
    }

    pub enum Oscillator<'this> {
        Square(SquareOscillator<'this>),
        Saw(SawOscillator<'this>),
        Triangle(TriangleOscillator<'this>),
    }

    impl<'this> Oscillator<'this> {
        pub fn sample(&mut self) -> Bipolar<1> {
            match self {
                Oscillator::Square(osc) => osc.sample(),
                Oscillator::Triangle(osc) => osc.sample(),
                Oscillator::Saw(osc) => osc.sample(),
            }
        }
    }

    pub struct SquareOscillator<'this> {
        pub state: &'this mut OscillatorState,
        pub period: SampleOffset,
        pub phase: Unipolar<1>,
    }

    impl<'this> SquareOscillator<'this> {
        pub fn sample(&mut self) -> Bipolar<1> {
            let phase = self.state.phase_accum.unwrap_or(self.phase);

            let phased_osc = phased::SquareOscillator {
                period: self.period,
                phase,
            };
            let sample = phased_osc.sample(SampleOffset(0.0));

            self.state.phase_accum = Some(accum_phase(phase, self.period));

            sample
        }
    }

    pub struct SquareOscillatorX16<'this> {
        pub state: &'this mut OscillatorState,
        pub period: [SampleOffset; 16],
        pub phase: Unipolar<1>,
    }        

    impl<'this> SquareOscillatorX16<'this> {
        pub fn sample(&mut self) -> [Bipolar<1>; 16] {
            let init_phase = self.state.phase_accum.unwrap_or(self.phase);
            let (phase, phase_accum) = accum_phase_x16(init_phase, self.period);

            let phased_osc = phased::SquareOscillatorX16 {
                period: self.period,
                phase,
            };
            let sample = phased_osc.sample([SampleOffset(0.0); 16]);

            self.state.phase_accum = Some(phase_accum);

            sample
        }
    }

    pub struct SawOscillator<'this> {
        pub state: &'this mut OscillatorState,
        pub period: SampleOffset,
        pub phase: Unipolar<1>,
    }

    impl<'this> SawOscillator<'this> {
        pub fn sample(&mut self) -> Bipolar<1> {
            let phase = self.state.phase_accum.unwrap_or(self.phase);

            let phased_osc = phased::SawOscillator {
                period: self.period,
                phase,
            };
            let sample = phased_osc.sample(SampleOffset(0.0));

            self.state.phase_accum = Some(accum_phase(phase, self.period));

            sample
        }
    }

    pub struct SawOscillatorX16<'this> {
        pub state: &'this mut OscillatorState,
        pub period: [SampleOffset; 16],
        pub phase: Unipolar<1>,
    }

    impl<'this> SawOscillatorX16<'this> {
        pub fn sample(&mut self) -> [Bipolar<1>; 16] {
            let init_phase = self.state.phase_accum.unwrap_or(self.phase);
            let (phase, phase_accum) = accum_phase_x16(init_phase, self.period);

            let phased_osc = phased::SawOscillatorX16 {
                period: self.period,
                phase,
            };
            let sample = phased_osc.sample([SampleOffset(0.0); 16]);

            self.state.phase_accum = Some(phase_accum);

            sample
        }
    }

    pub struct TriangleOscillator<'this> {
        pub state: &'this mut OscillatorState,
        pub period: SampleOffset,
        pub phase: Unipolar<1>,
    }

    impl<'this> TriangleOscillator<'this> {
        pub fn sample(&mut self) -> Bipolar<1> {
            let phase = self.state.phase_accum.unwrap_or(self.phase);

            let phased_osc = phased::TriangleOscillator {
                period: self.period,
                phase,
            };
            let sample = phased_osc.sample(SampleOffset(0.0));

            self.state.phase_accum = Some(accum_phase(phase, self.period));

            sample
        }
    }

    pub struct TriangleOscillatorX16<'this> {
        pub state: &'this mut OscillatorState,
        pub period: [SampleOffset; 16],
        pub phase: Unipolar<1>,
    }

    impl<'this> TriangleOscillatorX16<'this> {
        pub fn sample(&mut self) -> [Bipolar<1>; 16] {
            let init_phase = self.state.phase_accum.unwrap_or(self.phase);
            let (phase, phase_accum) = accum_phase_x16(init_phase, self.period);

            let phased_osc = phased::TriangleOscillatorX16 {
                period: self.period,
                phase,
            };
            let sample = phased_osc.sample([SampleOffset(0.0); 16]);

            self.state.phase_accum = Some(phase_accum);

            sample
        }
    }

    pub struct TableOscillator<'this> {
        pub table: &'this [f32],
        pub state: &'this mut OscillatorState,
        pub period: SampleOffset,
        pub phase: Unipolar<1>,
    }

    impl<'this> TableOscillator<'this> {
        pub fn sample(&mut self) -> Bipolar<1> {
            let phase = self.state.phase_accum.unwrap_or(self.phase);

            let phased_osc = phased::TableOscillator {
                table: self.table,
                period: self.period,
                phase,
            };
            let sample = phased_osc.sample(SampleOffset(0.0));

            self.state.phase_accum = Some(accum_phase(phase, self.period));

            sample
        }
    }

    pub struct TableOscillatorX16<'this> {
        pub table: &'this [f32],
        pub state: &'this mut OscillatorState,
        pub period: [SampleOffset; 16],
        pub phase: Unipolar<1>,
    }

    impl<'this> TableOscillatorX16<'this> {
        pub fn sample(&mut self) -> [Bipolar<1>; 16] {
            let init_phase = self.state.phase_accum.unwrap_or(self.phase);
            let (phase, phase_accum) = accum_phase_x16(init_phase, self.period);

            let phased_osc = phased::TableOscillatorX16 {
                table: self.table,
                period: self.period,
                phase,
            };
            let sample = phased_osc.sample([SampleOffset(0.0); 16]);

            self.state.phase_accum = Some(phase_accum);

            sample
        }
    }

    fn accum_phase(phase: Unipolar<1>, period: SampleOffset) -> Unipolar<1> {
        let phase_delta = 1.0 / period.0;
        let new_phase = (phase.0 + phase_delta) % 1.0;
        Unipolar(new_phase)
    }

    /// Accumalate the phase for 16 consecutive frames, plus the next frame.
    ///
    /// This operates slightly differently than `accum_phase` since the phases
    /// for all lanes in the phase-accumulating oscillators need to be
    /// calculated prior to calling the stateless simd oscillators underlying
    /// them.
    fn accum_phase_x16(phase: Unipolar<1>, period: [SampleOffset; 16]) -> ([Unipolar<1>; 16], Unipolar<1>) {
        let mut phase_accum = phase;
        let mut phase = [phase; 16];
        for i in 1..16 {
            phase_accum = accum_phase(phase_accum, period[i - 1]);
            phase[i] = phase_accum;
        }
        phase_accum = accum_phase(phase_accum, period[15]);
        (phase, phase_accum)
    }
}
