mod plotting;
//mod threads;

use clap::Parser;
use anyhow::Result;

#[derive(Parser)]
enum Command {
    Midi,
}

fn main() -> Result<()> {
    env_logger::Builder::new()
        .filter_level(log::LevelFilter::Info)
        .parse_default_env()
        .init();

    let opts = Command::parse();

    match opts {
        Command::Midi => {
        }
    }

    Ok(())
}

