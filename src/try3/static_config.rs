pub const NUM_LAYERS: usize = 4;

pub struct Synth {
    pub layers: [Layer; NUM_LAYERS],
}

pub struct Layer {
    pub osc: Oscillator,
    pub lpf: LowPassFilter,
    pub amp_env: Adsr,
    pub mod_env: Adsr,
    pub mods: Modulations,
}

pub struct Modulations {
    pub mod_env_to_osc_freq: f32,
    pub mod_env_to_lpf_freq: f32,
}

pub enum Oscillator {
    Square,
    Saw,
    Triangle,
    Noise,
}

pub struct LowPassFilter {
    pub freq: Hz,
}

pub struct Adsr {
    pub attack: Ms,
    pub decay: Ms,
    pub sustain: Unipolar,
    pub release: Ms,
}

pub struct Hz(pub f32);
pub struct Ms(pub f32);
pub struct Unipolar(pub f32);

