use std::sync::mpsc::{self, Sender, Receiver};
use anyhow::Result;
use std::thread::{self, JoinHandle};

pub fn run() -> Result <()> {
    let (tx_controller, rx_controller) = mpsc::channel::<ControllerMsg>();
    let (tx_input, rx_input) = mpsc::channel::<InputMsg>();
    let (tx_sequencer, rx_sequencer) = mpsc::channel::<SequencerMsg>();
    let (tx_audio_server, rx_audio_server) = mpsc::channel::<AudioServerMsg>();

    let controller_thread = ControllerThread {
        rx: rx_controller,
        tx_input,
        tx_sequencer,
        tx_audio_server,
    };

    let input_thread = InputThread {
        rx: rx_input,
        tx_controller: tx_controller.clone(),
    };

    let sequencer_thread = SequencerThread {
        rx: rx_sequencer,
        tx_controller: tx_controller.clone(),
    };

    let audio_server_thread = AudioServerThread {
        rx: rx_audio_server,
        tx_controller: tx_controller.clone(),
    };

    drop(tx_controller);

    
    todo!()
}

struct Threads {
    controller: JoinHandle<Result<()>>,
    input: JoinHandle<Result<()>>,
    sequencer: JoinHandle<Result<()>>,
    audio_server: JoinHandle<Result<()>>,
}

struct ControllerThread {
    rx: Receiver<ControllerMsg>,
    tx_input: Sender<InputMsg>,
    tx_sequencer: Sender<SequencerMsg>,
    tx_audio_server: Sender<AudioServerMsg>,
}

struct InputThread {
    rx: Receiver<InputMsg>,
    tx_controller: Sender<ControllerMsg>,
}

struct SequencerThread {
    rx: Receiver<SequencerMsg>,
    tx_controller: Sender<ControllerMsg>,
}

struct AudioServerThread {
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
