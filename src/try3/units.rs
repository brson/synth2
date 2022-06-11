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
