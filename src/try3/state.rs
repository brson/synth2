use soa_derive::StructOfArray;

#[derive(StructOfArray)]
#[derive(Default)]
pub struct Layer {
    #[nested_soa]
    pub lpf: LowPassFilter,
}

#[derive(StructOfArray)]
#[derive(Default)]
pub struct LowPassFilter {
    pub last: f32,
}
