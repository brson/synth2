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
            do_midi()?;
        }
    }

    Ok(())
}

fn do_midi() -> Result<()> {
    use midir::{MidiInput, Ignore};

    let mut midi_in = MidiInput::new("midir test input")?;
    midi_in.ignore(Ignore::None);

    println!("Available input ports:");
    for (i, p) in midi_in.ports().iter().enumerate() {
        println!("{}: {}", i, midi_in.port_name(p)?);
    }

    Ok(())
}
