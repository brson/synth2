#![allow(unused)]

mod plotting;
//mod threads;

use anyhow::{Result, anyhow};
use clap::Parser;

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
    let (midi, midi_rx) = {
        use midir::{Ignore, MidiInput};

        let mut midi_in = MidiInput::new("midir test input")?;
        midi_in.ignore(Ignore::None);

        log::info!("available midi input ports:");
        for (i, p) in midi_in.ports().iter().enumerate() {
            log::info!("{}: {}", i, midi_in.port_name(p)?);
        }

        let (midi_tx, midi_rx) = std::sync::mpsc::channel();

        let port = midi_in.ports().get(0).cloned();
        match port {
            Some(port) => {
                let midi = midi_in.connect(
                    &port,
                    "midi",
                    move |stamp, msg, _| {
                        midi_tx.send(msg.to_vec());
                    },
                    ()
                )?;
                (Some(midi), midi_rx)
            }
            None => {
                (None, midi_rx)
            }
        }
    };

    {
        use cpal::traits::{HostTrait, DeviceTrait};

        let host = cpal::default_host();

        log::info!("audio devices:");
        for device in host.devices()? {
            log::info!("{}", device.name()?);
        }

        let output_device = host.default_output_device();

        if let Some(output_device) = output_device {
            log::info!("default output device: {}", output_device.name()?);

            log::info!("supported output configs:");
            for configs in output_device.supported_output_configs()? {
                log::info!("{:#?}", configs);
            }

            let config = output_device.default_output_config()?;
            log::info!("default output config: {:#?}", config);
        }
    }

    let (midi_exit_tx, midi_exit_rx) = std::sync::mpsc::channel();

    let midi_thread = std::thread::spawn(move || {
        loop {
            if midi_exit_rx.try_recv().is_ok() {
                break;
            }

            std::thread::yield_now();

            let midi_msg = midi_rx.try_recv();

            match midi_msg {
                Ok(midi_msg) => {
                    log::debug!("midi msg bytes: {:?}", midi_msg);

                    use muddy2::parser::{Parser, MessageParseOutcome, MessageParseOutcomeStatus};

                    let mut parser = Parser::new();
                    let parse = parser.parse(&midi_msg);

                    match parse {
                        Ok(parse) => {
                            if parse.bytes_consumed as usize != midi_msg.len() {
                                log::error!("did not consume entire midi message. len = {}, consumed = {}", midi_msg.len(), parse.bytes_consumed);
                            }
                            log::debug!("midi msg: {:#?}", parse.status);
                        }
                        Err(e) => {
                            log::error!("midi parse error: {}", e);
                            let mut maybe_source = e.source();
                            while let Some(source) = maybe_source {
                                log::error!("source: {}", source);
                                maybe_source = source.source();
                            }
                        }
                    }
                }
                _ => { }
            }
        }
    });

    std::io::stdin().read_line(&mut String::new());

    midi_exit_tx.send(());
    midi_thread.join();
    drop(midi);

    Ok(())
}
