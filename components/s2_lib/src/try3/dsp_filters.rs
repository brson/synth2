/// Filters from _DSP Filters_ by John Lane et. al.
///
/// `x` variables are inputs,
/// with `x1` being the first delayed input sample.
///
/// `y` variables are outputs,
/// with `y1` being the first delayed (feedback) output sample.

use super::units::*;
use std::f32::consts::PI;

#[derive(Default)]
#[derive(Copy, Clone)]
pub struct FirstOrderLowPassFilterState {
    x1: f32,
    y1: f32,
}

pub struct FirstOrderLowPassFilter<'this> {
    pub state: &'this mut FirstOrderLowPassFilterState,
    pub sample_rate: SampleRateKhz,
    pub cutoff_freq: Hz,
}

impl<'this> FirstOrderLowPassFilter<'this> {
    pub fn process(&mut self, input: f32) -> f32 {
        let sample_rate = self.sample_rate.0 as f32;
        let cutoff_freq = self.cutoff_freq.0;

        let theta_cutoff = 2.0 * PI * cutoff_freq / sample_rate;
        let gamma = theta_cutoff.cos() / ( 1.0 + theta_cutoff.sin());
        let alpha = (1.0 - gamma) / 2.0;

        let x1 = self.state.x1;
        let y1 = self.state.y1;

        let x = input;
        let y = alpha * (x + x1) + gamma * y1;

        self.state.x1 = x;
        self.state.y1 = y;

        y
    }
}

#[derive(Default)]
#[derive(Copy, Clone)]
pub struct FirstOrderHighPassFilterState {
    x1: f32,
    y1: f32,
}

pub struct FirstOrderHighPassFilter<'this> {
    pub state: &'this mut FirstOrderHighPassFilterState,
    pub sample_rate: SampleRateKhz,
    pub cutoff_freq: Hz,
}

impl<'this> FirstOrderHighPassFilter<'this> {
    pub fn process(&mut self, input: f32) -> f32 {
        let sample_rate = self.sample_rate.0 as f32;
        let cutoff_freq = self.cutoff_freq.0;

        let theta_cutoff = 2.0 * PI * cutoff_freq / sample_rate;
        let gamma = theta_cutoff.cos() / ( 1.0 + theta_cutoff.sin());
        let alpha = (1.0 + gamma) / 2.0;

        let x1 = self.state.x1;
        let y1 = self.state.y1;

        let x = input;
        let y = alpha * (x - x1) + gamma * y1;

        self.state.x1 = x;
        self.state.y1 = y;

        y
    }
}

#[derive(Default)]
#[derive(Copy, Clone)]
pub struct SecondOrderLowPassFilterState {
    x1: f32,
    x2: f32,
    y1: f32,
    y2: f32,
}

pub struct SecondOrderLowPassFilter<'this> {
    pub state: &'this mut SecondOrderLowPassFilterState,
    pub sample_rate: SampleRateKhz,
    pub cutoff_freq: Hz,
    /// Lower is narrower, more resonant.
    pub damping_factor: Unipolar<10>,
}

impl<'this> SecondOrderLowPassFilter<'this> {
    pub fn process(&mut self, input: f32) -> f32 {
        let sample_rate = self.sample_rate.0 as f32;
        let cutoff_freq = self.cutoff_freq.0;
        let damping_factor = self.damping_factor.0;

        let theta_cutoff = 2.0 * PI * cutoff_freq / sample_rate;
        let beta = (1.0 / 2.0)
            * ((1.0 - damping_factor / 2.0 * theta_cutoff.sin())
               / (1.0 + damping_factor / 2.0 * theta_cutoff.sin()));
        let gamma = (1.0 / 2.0 + beta) * theta_cutoff.cos();
        let alpha = (1.0 / 2.0 + beta - gamma) / 4.0;

        let x1 = self.state.x1;
        let x2 = self.state.x2;
        let y1 = self.state.y1;
        let y2 = self.state.y2;

        let x = input;
        let y = 2.0
            * (alpha
                * (x + 2.0 * x1 + x2)
                + gamma * y1 - beta * y2);

        self.state.x2 = x1;
        self.state.x1 = x;
        self.state.y2 = y1;
        self.state.y1 = y;

        y
    }
}

#[derive(Default)]
#[derive(Copy, Clone)]
pub struct SecondOrderHighPassFilterState {
    x1: f32,
    x2: f32,
    y1: f32,
    y2: f32,
}

pub struct SecondOrderHighPassFilter<'this> {
    pub state: &'this mut SecondOrderHighPassFilterState,
    pub sample_rate: SampleRateKhz,
    pub cutoff_freq: Hz,
    /// Lower is narrower, more resonant.
    pub damping_factor: Unipolar<10>,
}

impl<'this> SecondOrderHighPassFilter<'this> {
    pub fn process(&mut self, input: f32) -> f32 {
        let sample_rate = self.sample_rate.0 as f32;
        let cutoff_freq = self.cutoff_freq.0;
        let damping_factor = self.damping_factor.0;

        let theta_cutoff = 2.0 * PI * cutoff_freq / sample_rate;
        let beta = (1.0 / 2.0)
            * ((1.0 - damping_factor / 2.0 * theta_cutoff.sin())
               / (1.0 + damping_factor / 2.0 * theta_cutoff.sin()));
        let gamma = (1.0 / 2.0 + beta) * theta_cutoff.cos();
        let alpha = (1.0 / 2.0 + beta + gamma) / 4.0;

        let x1 = self.state.x1;
        let x2 = self.state.x2;
        let y1 = self.state.y1;
        let y2 = self.state.y2;

        let x = input;
        let y = 2.0
            * (alpha
                * (x - 2.0 * x1 + x2)
                + gamma * y1 - beta * y2);

        self.state.x2 = x1;
        self.state.x1 = x;
        self.state.y2 = y1;
        self.state.y1 = y;

        y
    }
}

#[derive(Default)]
#[derive(Copy, Clone)]
pub struct SecondOrderBandPassFilterState {
    x1: f32,
    x2: f32,
    y1: f32,
    y2: f32,
}

pub struct SecondOrderBandPassFilter<'this> {
    pub state: &'this mut SecondOrderBandPassFilterState,
    pub sample_rate: SampleRateKhz,
    pub center_freq: Hz,
    /// Higher is narrower.
    pub quality_factor: Unipolar<10>,
}

impl<'this> SecondOrderBandPassFilter<'this> {
    pub fn process(&mut self, input: f32) -> f32 {
        let sample_rate = self.sample_rate.0 as f32;
        let center_freq = self.center_freq.0;
        let quality_factor = self.quality_factor.0;

        let theta_center = 2.0 * PI * center_freq / sample_rate;
        let beta = (1.0 / 2.0)
            * ((1.0 - (theta_center / (2.0 * quality_factor)).tan())
               / (1.0 + (theta_center / (2.0 * quality_factor)).tan()));
        let gamma = (1.0 / 2.0 + beta) * theta_center.cos();
        let alpha = (1.0 / 2.0 - beta) / 2.0;

        let x1 = self.state.x1;
        let x2 = self.state.x2;
        let y1 = self.state.y1;
        let y2 = self.state.y2;

        let x = input;
        let y = 2.0
            * (alpha
                * (x - x2)
                + gamma * y1 - beta * y2);

        self.state.x2 = x1;
        self.state.x1 = x;
        self.state.y2 = y1;
        self.state.y1 = y;

        y
    }
}
