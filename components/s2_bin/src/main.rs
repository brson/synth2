#![feature(let_else)]
#![allow(unused)]

mod plotting;
//mod threads;
mod audio_player;
mod synth;

use anyhow::{Result, anyhow};
use clap::Parser;
use std::sync::mpsc;

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
    let audio_player = audio_player::start_player()?;
    let (audio_player_channels, audio_player_stream) =
        audio_player.map(|player| {
            (Some(player.channels), Some(player.stream))
        }).unwrap_or((None, None));

    let (midi, midi_rx) = {
        use midir::{Ignore, MidiInput};

        let mut midi_in = MidiInput::new("midir test input")?;
        midi_in.ignore(Ignore::None);

        log::info!("available midi input ports:");
        for (i, p) in midi_in.ports().iter().enumerate() {
            log::info!("{}: {}", i, midi_in.port_name(p)?);
        }

        const MAX_MIDI_MESSAGES: usize = 1024;

        let (midi_tx, midi_rx) = mpsc::sync_channel(MAX_MIDI_MESSAGES);

        let port = midi_in.ports().get(0).cloned();
        match port {
            Some(port) => {
                let midi = midi_in.connect(
                    &port,
                    "midi",
                    move |stamp, msg, _| {
                        match midi_tx.try_send(msg.to_vec()) {
                            Ok(_) => { },
                            Err(mpsc::TrySendError::Disconnected(_)) => {
                                /* shutting down? */
                            },
                            Err(mpsc::TrySendError::Full(_)) => {
                                log::error!("midi channel full");
                            }
                        }
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

    let synth_thread = std::thread::Builder::new()
        .name("synth".to_string())
        .spawn(move || {
            run_synth(audio_player_channels, midi_rx);
        })?;

    std::io::stdin().read_line(&mut String::new());

    drop(midi);
    drop(audio_player_stream);

    let threads = [
        synth_thread,
    ];

    let thread_results = threads.map(|t| {
        (
            t.thread().name().unwrap_or("unknown").to_owned(),
            t.join()
        )
    });

    for (thread_name, thread_result) in thread_results {
        if let Err(e) = thread_result {
            log::error!("thread {} panicked: {:?}", thread_name, e);
        }
    }

    Ok(())
}

fn parse_midi_message(midi_msg: &[u8]) -> Option<muddy2::message::Message> {
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
            match parse.status {
                MessageParseOutcomeStatus::Message(msg) => Some(msg),
                _ => None,
            }
        }
        Err(e) => {
            log::error!("midi parse error: {}", e);
            let mut maybe_source = e.source();
            while let Some(source) = maybe_source {
                log::error!("source: {}", source);
                maybe_source = source.source();
            }
            None
        }
    }
}

fn run_synth(
    audio_player_channels: Option<audio_player::PlayerChannels>,
    midi_rx: mpsc::Receiver<Vec<u8>>,
) {
    use s2_lib::try3::units::SampleRateKhz;

    let Some(audio_player_channels) = audio_player_channels else {
        log::info!("no audio player");
        return;
    };

    let sample_rate = SampleRateKhz(audio_player_channels.sample_rate);
    let mut synth = synth::Synth::new();

    loop {
        match audio_player_channels.buf_empty_rx.recv() {
            Ok(mut buffer) => {
                loop {
                    match midi_rx.try_recv() {
                        Ok(midi_msg) => {
                            apply_middi(&midi_msg, &mut synth);
                        }
                        Err(_) => {
                            break;
                        }
                    }
                }

                synth.sample(buffer.as_slice_mut(), sample_rate);

                match audio_player_channels.buf_filled_tx.try_send(buffer) {
                    Ok(_) => { },
                    Err(mpsc::TrySendError::Disconnected(_)) => {
                        /* shutting down */
                    }
                    Err(mpsc::TrySendError::Full(_)) => {
                        panic!("full channel");
                    }
                }
            }
            Err(mpsc::RecvError) => {
                break;
            }
        }
    }

    drop(audio_player_channels);

    log::info!("synth thread exiting");
}

fn apply_middi(midi_msg: &[u8], synth: &mut synth::Synth) {
    use muddy2::message::{Message, ChannelMessage, ChannelMessageType, ChannelVoiceMessage};
    use s2_lib::try3::units::Unipolar;

    let midi_msg = parse_midi_message(&midi_msg);
    match midi_msg {
        Some(Message::Channel(ch_msg)) => {
            match ch_msg.message {
                ChannelMessageType::ChannelVoice(
                    ChannelVoiceMessage::NoteOn(note_on)
                ) => {
                    let note = synth::Note(u8::from(note_on.note_number.0));
                    let velocity = synth::Velocity(Unipolar(f32::from(u8::from(note_on.velocity.0)) / 127.0));
                    synth.note_on(note, velocity);
                }
                ChannelMessageType::ChannelVoice(
                    ChannelVoiceMessage::NoteOff(note_off)
                ) => {
                    let note = synth::Note(u8::from(note_off.note_number.0));
                    synth.note_off(note);
                }
                _ => { }
            }
        }
        _ => { }
    }
}
