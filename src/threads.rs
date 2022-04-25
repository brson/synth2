use std::sync::mpsc::{self, Sender, Receiver, TryRecvError};
use anyhow::{Result, anyhow};
use std::thread::{self, JoinHandle};
use std::io::Read;

const BUFFER_SIZE: usize = 256 * 4 * 4;

pub fn run() -> Result <()> {
    println!("press 'e' to exit");

    let (tx_controller, rx_controller) = mpsc::channel::<ControllerMsg>();
    let (tx_input, rx_input) = mpsc::channel::<InputMsg>();
    let (tx_sequencer, rx_sequencer) = mpsc::channel::<SequencerMsg>();
    let (tx_audio_server, rx_audio_server) = mpsc::channel::<AudioServerMsg>();

    let controller_context = ControllerContext {
        rx: rx_controller,
        tx_input,
        tx_sequencer,
        tx_audio_server,
    };

    let input_context = InputContext {
        rx: rx_input,
        tx_controller: tx_controller.clone(),
    };

    let sequencer_context = SequencerContext {
        rx: rx_sequencer,
        tx_controller: tx_controller.clone(),
    };

    let audio_server_context = AudioServerContext {
        rx: rx_audio_server,
        tx_controller: tx_controller.clone(),
    };

    drop(tx_controller);

    let threads = Threads {
        controller: thread::spawn(move || run_controller(controller_context)),
        input: thread::spawn(move || run_input(input_context)),
        sequencer: thread::spawn(move || run_sequencer(sequencer_context)),
        audio_server: thread::spawn(move || run_audio_server(audio_server_context)),
    };
    
    threads.join()
}

struct Threads {
    controller: JoinHandle<Result<()>>,
    input: JoinHandle<Result<()>>,
    sequencer: JoinHandle<Result<()>>,
    audio_server: JoinHandle<Result<()>>,
}

impl Threads {
    fn join(self) -> Result<()> {
        let controller_res = self.controller.join();
        let input_res = self.input.join();
        let sequencer_res = self.sequencer.join();
        let audio_server_res = self.audio_server.join();

        let mut errors = false;
        
        for res in [controller_res,
                    input_res,
                    sequencer_res,
                    audio_server_res]
        {
            match res {
                Err(e) => std::panic::resume_unwind(e),
                Ok(Err(e)) => {
                    log::error!("{}", e);
                    errors = true;
                }
                Ok(Ok(_)) => (/* pass */),
            }
        }

        if errors {
            Err(anyhow!("some threads failed"))
        } else {
            Ok(())
        }
    }
}

struct ControllerContext {
    rx: Receiver<ControllerMsg>,
    tx_input: Sender<InputMsg>,
    tx_sequencer: Sender<SequencerMsg>,
    tx_audio_server: Sender<AudioServerMsg>,
}

struct InputContext {
    rx: Receiver<InputMsg>,
    tx_controller: Sender<ControllerMsg>,
}

struct SequencerContext {
    rx: Receiver<SequencerMsg>,
    tx_controller: Sender<ControllerMsg>,
}

struct AudioServerContext {
    rx: Receiver<AudioServerMsg>,
    tx_controller: Sender<ControllerMsg>,
}

enum ControllerMsg {
    Input(ControllerInputMsg),
    Sequencer(ControllerSequencerMsg),
    AudioServer(ControllerAudioServerMsg),
}

enum ControllerInputMsg {
    Exit,
}

enum ControllerSequencerMsg {
    BufferFilled(Vec<f64>),
}

enum ControllerAudioServerMsg {
    BufferPlayed(Vec<f64>),
}

enum InputMsg {
    Exit,
}

enum SequencerMsg {
    Exit,
    FillBuffer(Vec<f64>),
}

enum AudioServerMsg {
    Exit,
    PlayBuffer(Vec<f64>),
}

fn run_controller(ctx: ControllerContext) -> Result<()> {

    let mut buffers = vec![
        vec![0_f64; BUFFER_SIZE],
        vec![0_f64; BUFFER_SIZE],
    ];

    ctx.tx_sequencer.send(SequencerMsg::FillBuffer(buffers.pop().expect("buf")))?;
    ctx.tx_sequencer.send(SequencerMsg::FillBuffer(buffers.pop().expect("buf")))?;
    
    loop {
        match ctx.rx.recv()? {
            ControllerMsg::Input(
                ControllerInputMsg::Exit
            ) => {
                ctx.tx_input.send(InputMsg::Exit)?;
                ctx.tx_sequencer.send(SequencerMsg::Exit)?;
                ctx.tx_audio_server.send(AudioServerMsg::Exit)?;
                break;
            }
            ControllerMsg::Sequencer(
                ControllerSequencerMsg::BufferFilled(buffer)
            ) => {
                ctx.tx_audio_server.send(AudioServerMsg::PlayBuffer(buffer))?;
            }
            ControllerMsg::AudioServer(
                ControllerAudioServerMsg::BufferPlayed(buffer)
            ) => {
                ctx.tx_sequencer.send(SequencerMsg::FillBuffer(buffer))?;
            }
        }
    }

    Ok(())
}

fn run_input(ctx: InputContext) -> Result<()> {
    use termion::AsyncReader;
    use termion::raw::IntoRawMode;

    let raw_stdout = std::io::stdout().into_raw_mode()?;

    let mut stdin = termion::async_stdin();
    let mut readchar = || {
        let mut buf = [0; 1];
        let res = stdin.read(&mut buf).ok();
        if res == Some(1) {
            Some(buf[0])
        } else {
            None
        }
    };

    loop {
        let ch = readchar();
        if ch == Some(b'e') {
            ctx.tx_controller.send(
                ControllerMsg::Input(ControllerInputMsg::Exit)
            )?;
        } else if ch == Some(b'a') {
            println!("\r");
        }

        match ctx.rx.try_recv() {
            Ok(InputMsg::Exit) => {
                break;
            }
            Err(TryRecvError::Empty) => { },
            Err(TryRecvError::Disconnected) => {
                Err(TryRecvError::Disconnected)?;
            }
        }

        #[allow(deprecated)]
        thread::sleep_ms(5);
    }

    Ok(())
}

fn run_sequencer(ctx: SequencerContext) -> Result<()> {

    let mut seq = crate::seq::Sequencer::new();
    
    use crate::f64;
    use crate::math::*;

    let mut offset = 0;
    let osc = f64::square_osc();
    let mut lpf = f64::LowPassFilter::new(
        ZPos64::assert_from(440.0 * 1.5),
        f64::SAMPLE_RATE_KHZ,
    );

    loop {
        match ctx.rx.recv()? {
            SequencerMsg::Exit => {
                break;
            }
            SequencerMsg::FillBuffer(mut buffer) => {
                for i in 0..buffer.len() {
                    /*let sample = osc.sample(Snat32::assert_from(offset)).into();
                    let sample = lpf.process(sample);
                    buffer[i] = sample;
                    offset += 1;*/

                    let sample = seq.next_sample();
                    buffer[i] = sample;
                }
                ctx.tx_controller.send(
                    ControllerMsg::Sequencer(
                        ControllerSequencerMsg::BufferFilled(buffer)
                    )
                )?;
            }
        }
    }

    Ok(())
}

static WEBSOCKET_URL: &str = "127.0.0.1:9110";

fn run_audio_server(ctx: AudioServerContext) -> Result<()> {

    use std::net::TcpListener;
    use std::io;
    use tungstenite::protocol::Message;
    use tungstenite::error::Error as WsError;
    use std::time::{Instant, Duration};
    use std::collections::VecDeque;

    const SAMPLE_RATE_KHZ: usize = 32_000;

    // A queue of buffers to return to the controller thread. By rate limiting
    // when these are returned we feed audio to the websocket at a consistent
    // rate.
    type Buffer = Vec<f64>;
    type ReceivedAt = Instant;
    type ReleaseAfter = Duration;
    let mut queued_buffers: VecDeque<(Buffer, ReceivedAt, ReleaseAfter)> = VecDeque::new();

    let mut websocket: Option<tungstenite::WebSocket<std::net::TcpStream>> = None;

    let listener = TcpListener::bind(WEBSOCKET_URL)?;
    listener.set_nonblocking(true)?;

    loop {

        match listener.accept() {
            Ok((socket, _addr)) => {
                if let Some(websocket) = websocket.as_mut() {
                    websocket.close(None);
                }
                println!("websocket connected\r");
                websocket = Some(tungstenite::accept(socket)?);
            }
            Err(e) if e.kind() == io::ErrorKind::WouldBlock => {
                // pass
            }
            Err(e) => {
                Err(e)?;
            }
        }
        
        match ctx.rx.try_recv() {
            Ok(AudioServerMsg::Exit) => {
                break;
            },
            Ok(AudioServerMsg::PlayBuffer(buffer)) => {
                let mut drop_websocket = false;
                if let Some(websocket) = websocket.as_mut() {
                    let buffer32: Vec<f32> = buffer.iter().map(|f| *f as f32).collect();
                    let text = serde_json::to_string(&buffer32)?;
                    let msg = Message::Text(text);

                    loop {
                        let res = websocket.write_message(msg.clone());

                        match res {
                            Err(WsError::Io(e)) if e.kind() == io::ErrorKind::WouldBlock => {
                                println!("wouldblock - repeating\r");
                                continue;
                            }
                            Ok(()) => {
                                drop_websocket = false;
                            }
                            Err(e) => {
                                println!("eee");
                                println!("websocket.write_message: {}\r", e);
                                drop_websocket = true;
                            }
                        }

                        break;
                    }
                }

                if drop_websocket {
                    websocket = None;
                }

                let buffer_instant = Instant::now();
                let buffer_duration_s = buffer.len() as f64 / SAMPLE_RATE_KHZ as f64;
                let buffer_duration_us = buffer_duration_s * 1000_000_f64;
                let buffer_duration_us = buffer_duration_us as u64;
                let buffer_duration = Duration::from_micros(buffer_duration_us);

                let mut duration_before_release = buffer_duration;

                // Check if the controller sent this buffer before the time
                // duration of the previous buffer, and delay the release of
                // this buffer if so. This happens on the first frame when the
                // controller immediately sends two bufers.
                if let Some(&(_, prev_instant, prev_duration)) = queued_buffers.back() {
                    let duration_since_last = buffer_instant - prev_instant;
                    if (duration_since_last < prev_duration) {
                        let duration_to_add = prev_duration - duration_since_last;
                        duration_before_release = duration_before_release.saturating_add(duration_to_add);
                    }
                }

                queued_buffers.push_back(
                    (buffer, buffer_instant, duration_before_release)
                );
            }
            Err(TryRecvError::Empty) => { },
            Err(TryRecvError::Disconnected) => {
                Err(TryRecvError::Disconnected)?;
            }
        }

        // See if it's time to release a buffer
        if let Some((buffer, buffer_instant, buffer_duration)) = queued_buffers.pop_front() {
            let now = Instant::now();
            let duration_since = now.duration_since(buffer_instant);
            let should_release = duration_since > buffer_duration;
            if should_release {
                ctx.tx_controller.send(
                    ControllerMsg::AudioServer(
                        ControllerAudioServerMsg::BufferPlayed(buffer)
                    )
                )?;
            } else {
                queued_buffers.push_front(
                    (buffer, buffer_instant, buffer_duration)
                );
            }
        }

        #[allow(deprecated)]
        thread::sleep_ms(5);
    }

    Ok(())
}
