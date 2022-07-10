use soa_derive::StructOfArray;

#[derive(StructOfArray, Default)]
#[derive(Copy, Clone)]
pub struct Layer {
    #[nested_soa]
    pub lpf: LowPassFilter,
}

#[derive(StructOfArray, Default)]
#[derive(Copy, Clone)]
pub struct LowPassFilter {
    pub last: f32,
}
