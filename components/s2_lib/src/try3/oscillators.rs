pub use basic::*;

pub mod basic {
    use super::super::math::*;
    use super::super::units::*;

    pub struct SquareOscillator {
        pub period: SampleOffset,
    }

    pub struct SawOscillator {
        pub period: SampleOffset,
    }

    pub struct TriangleOscillator {
        pub period: SampleOffset,
    }

    pub struct TableOscillator<'this> {
        pub table: &'this [f32],
        pub period: SampleOffset,
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
}

pub mod phased {
    use super::super::units::*;
    use super::basic;

    pub struct SquareOscillator {
        pub period: SampleOffset,
        pub phase: Unipolar<1>,
    }

    impl SquareOscillator {
        pub fn sample(&self, offset: SampleOffset) -> Bipolar<1> {
            let phase_offset = self.period.0 * self.phase.0;
            let offset = SampleOffset(offset.0 + phase_offset);
            let basic_osc = basic::SquareOscillator {
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
            let phase_offset = self.period.0 * self.phase.0;
            let offset = SampleOffset(offset.0 + phase_offset);
            let basic_osc = basic::SawOscillator {
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
            let phase_offset = self.period.0 * self.phase.0;
            let offset = SampleOffset(offset.0 + phase_offset);
            let basic_osc = basic::TriangleOscillator {
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
            let phase_offset = self.period.0 * self.phase.0;
            let offset = SampleOffset(offset.0 + phase_offset);
            let basic_osc = basic::TableOscillator {
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

    fn accum_phase(phase: Unipolar<1>, period: SampleOffset) -> Unipolar<1> {
        let phase_delta = 1.0 / period.0;
        let new_phase = (phase.0 + phase_delta) % 1.0;
        Unipolar(new_phase)
    }
}
