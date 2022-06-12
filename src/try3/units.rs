#[derive(Copy, Clone)]
pub struct Hz(pub f32);

#[derive(Copy, Clone)]
pub struct Ms(pub f32);

#[derive(Copy, Clone)]
pub struct Bipolar<const N: u16>(pub f32);

#[derive(Copy, Clone)]
pub struct Unipolar<const N: u16>(pub f32);

#[derive(Copy, Clone)]
pub struct SampleRateKhz(pub u32);

#[derive(Copy, Clone)]
pub struct SampleOffset(pub f32);

impl Ms {
    /// Get the time as samples
    pub fn as_samples(&self, sample_rate: SampleRateKhz) -> f32 {
        let sample_rate = sample_rate.0 as f32;
        let ms = self.0;
        let seconds = ms / 1000.0;
        let samples = sample_rate * seconds;
        samples
    }
}    
