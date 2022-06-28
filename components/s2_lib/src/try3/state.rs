use soa_derive::StructOfArray;

#[derive(StructOfArray, Default)]
pub struct Layer {
    #[nested_soa]
    pub lpf: LowPassFilter,
}

#[derive(StructOfArray, Default)]
pub struct LowPassFilter {
    pub last: f32,
}
