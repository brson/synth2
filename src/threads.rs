use std::sync::mpsc::{self, Sender, Receiver};
use anyhow::Result;
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
    fn join(&self) -> Result<()> {
        todo!()
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
}

enum InputMsg {
}

enum SequencerMsg {
}

enum AudioServerMsg {
}

fn run_controller(ctx: ControllerContext) -> Result<()> {
    todo!()
}

fn run_input(ctx: InputContext) -> Result<()> {
    todo!()
}

fn run_sequencer(ctx: SequencerContext) -> Result<()> {
    todo!()
}

fn run_audio_server(ctx: AudioServerContext) -> Result<()> {
    todo!()
}
