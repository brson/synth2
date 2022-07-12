pub use basic::*;

pub mod basic {
    use super::super::math::*;
    use super::super::units::*;

    pub enum Oscillator {
        Square(SquareOscillator),
        Saw(SawOscillator),
        Triangle(TriangleOscillator),
    }

    pub struct SquareOscillator {
        pub period: SampleOffset,
    }

    pub struct SawOscillator {
        pub period: SampleOffset,
    }

    pub struct TriangleOscillator {
        pub period: SampleOffset,
    }

    impl Oscillator {
        pub fn sample(&self, offset: SampleOffset) -> Bipolar<1> {
            match self {
                Oscillator::Square(osc) => osc.sample(offset),
                Oscillator::Triangle(osc) => osc.sample(offset),
                Oscillator::Saw(osc) => osc.sample(offset),
            }
        }
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

            let x_rise = -2.0;
            let x_run = period;
            let x_value = offset;
            let y_offset = 1.0;

            let sample = line_y_value_with_y_offset(x_rise, x_run, x_value, y_offset);

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
                let x_rise = -2.0;
                let x_run = half_period;
                let x_value = offset;
                let y_offset = 1.0;

                line_y_value_with_y_offset(x_rise, x_run, x_value, y_offset)
            } else {
                let x_rise = 2.0;
                let x_run = half_period;
                let x_value = offset - half_period;
                let y_offset = -1.0;

                line_y_value_with_y_offset(x_rise, x_run, x_value, y_offset)
            };

            Bipolar(sample)
        }
    }
}

mod phased {
    use super::super::units::*;

    pub struct SquareOscillator {
        pub period: SampleOffset,
        pub phase: Unipolar<1>,
    }

    impl SquareOscillator {
        pub fn sample(&self, offset: SampleOffset) -> Bipolar<1> {
            todo!()
        }
    }
}

/// Stateful oscillators that can be frequency modulated.
///
/// They always resume sampling at the same phase as the previous sample, even
/// if the frequency has changed.
mod phase_accumulating {
}
