use anyhow::{Result, anyhow};
use crate::math::*;

pub enum Oscillator {
    Square(SquareOscillator),
    Triangle(TriangleOscillator),
    Saw(SawOscillator),
    Sine(SineOscillator),
    Noise(NoiseOscillator),
}

pub struct SquareOscillator {
    pub sample_rate: SampleRateKhz,
    pub freq: Hz64,
    pub period: u32,
}

pub struct TriangleOscillator {
    pub sample_rate: SampleRateKhz,
    pub freq: Hz64,
    pub period: u32,
}

pub struct SawOscillator {
    pub sample_rate: SampleRateKhz,
    pub freq: Hz64,
    pub period: u32,
}

pub struct SineOscillator {
    pub sample_rate: SampleRateKhz,
    pub freq: Hz64,
    pub period: u32,
}

pub struct NoiseOscillator {
    pub seed: u32,
}

impl Oscillator {
    pub fn sample(&self, offset: u32) -> One64 {
        match self {
            Oscillator::Square(osc) => osc.sample(offset),
            Oscillator::Triangle(osc) => osc.sample(offset),
            Oscillator::Saw(osc) => osc.sample(offset),
            Oscillator::Sine(osc) => osc.sample(offset),
            Oscillator::Noise(osc) => osc.sample(offset),
        }
    }
}

impl SquareOscillator {
    pub fn sample(&self, offset: u32) -> One64 {
        todo!()
    }
}

impl TriangleOscillator {
    pub fn sample(&self, offset: u32) -> One64 {
        todo!()
    }
}

impl SawOscillator {
    pub fn sample(&self, offset: u32) -> One64 {
        todo!()
    }
}

impl SineOscillator {
    pub fn sample(&self, offset: u32) -> One64 {
        todo!()
    }
}

impl NoiseOscillator {
    pub fn sample(&self, offset: u32) -> One64 {
        todo!()
    }
}

