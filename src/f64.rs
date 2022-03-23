use anyhow::Result;
use std::path::{Path, PathBuf};

const SAMPLE_RATE_KHZ: i32 = 32_000;
const A440_SAMPLES: i32 = SAMPLE_RATE_KHZ / 440;

