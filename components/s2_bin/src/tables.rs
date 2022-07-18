pub fn build() -> anyhow::Result<()> {
    const SIN_SAMPLES: usize = 1024;

    println!("pub const SIN_TABLE: [f32; {SIN_SAMPLES}] = [");

    for i in 0..SIN_SAMPLES {
        let i = i as f32 / SIN_SAMPLES as f32;
        let i = i * std::f32::consts::PI * 2.0;
        let sample = i.sin();
        println!("    {sample:.30},");
    }
    
    println!("];");

    Ok(())
}
