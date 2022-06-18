use super::math::*;
use super::units::*;

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
    pub fn sample(&self, offset: SampleOffset) -> Unipolar<1> {
        match self {
            Oscillator::Square(osc) => osc.sample(offset),
            Oscillator::Triangle(osc) => osc.sample(offset),
            Oscillator::Saw(osc) => osc.sample(offset),
        }
    }
}

impl SquareOscillator {
    pub fn sample(&self, offset: SampleOffset) -> Unipolar<1> {
        todo!()
    }
}

impl SawOscillator {
    pub fn sample(&self, offset: SampleOffset) -> Unipolar<1> {
        todo!()
    }
}

impl TriangleOscillator {
    pub fn sample(&self, offset: SampleOffset) -> Unipolar<1> {
        todo!()
    }
}
