use std::sync::mpsc::{self, Sender, Receiver};
use anyhow::{Result, anyhow};
use std::thread::{self, JoinHandle};

pub fn run() -> Result <()> {
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

    const BUFFER_SIZE: usize = 256 * 4 * 4;

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

    loop {
        match ctx.rx.recv()? {
            InputMsg::Exit => break,
        }
    }

    Ok(())
}

fn run_sequencer(ctx: SequencerContext) -> Result<()> {

    loop {
        match ctx.rx.recv()? {
            SequencerMsg::Exit => break,
            SequencerMsg::FillBuffer(mut buffer) => {
                todo!()
            }
        }
    }

    Ok(())
}

fn run_audio_server(ctx: AudioServerContext) -> Result<()> {

    loop {
        match ctx.rx.recv()? {
            AudioServerMsg::Exit => break,
            AudioServerMsg::PlayBuffer(buffer) => {
                todo!()
            }
        }
    }

    Ok(())
}
