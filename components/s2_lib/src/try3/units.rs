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

impl Hz {
    pub fn as_samples(&self, sample_rate: SampleRateKhz) -> SampleOffset {
        let sample_rate = sample_rate.0 as f32;
        let hz = self.0;
        let samples = sample_rate / hz;
        SampleOffset(samples)
    }
}

impl Ms {
    /// Get the time as samples
    pub fn as_samples(&self, sample_rate: SampleRateKhz) -> SampleOffset {
        let sample_rate = sample_rate.0 as f32;
        let ms = self.0;
        let seconds = ms / 1000.0;
        let samples = sample_rate * seconds;
        SampleOffset(samples)
    }
}

impl<const N: u16> TryFrom<f32> for Unipolar<N> {
    type Error = anyhow::Error;

    fn try_from(other: f32) -> anyhow::Result<Unipolar<N>> {
        if other >= 0.0 && other <= N as f32 {
            Ok(Unipolar(other))
        } else {
            Err(anyhow::anyhow!("float out of [0, {}] range", N))
        }
    }
}
