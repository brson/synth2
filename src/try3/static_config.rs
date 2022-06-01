pub const NUM_LAYERS: usize = 2;

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
    pub osc_freq: f64,
    pub lpf_freq: f64,
}

pub enum Oscillator {
    Square,
    Saw,
    Triangle,
    Sine,
    Noise,
}

pub struct LowPassFilter {
    pub freq: Hz,
}

pub struct Adsr {
    pub attack: Ms,
    pub decay: Ms,
    pub sustain: ZOne,
    pub release: Ms,
}

pub struct Hz(pub f64);
pub struct Ms(pub f64);
pub struct ZOne(pub f64);

