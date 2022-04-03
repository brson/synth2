use std::sync::mpsc::{self, Sender, Receiver};
use anyhow::Result;
use std::thread::JoinHandle;

pub fn run() -> Result <()> {
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
    input_tx: Sender<InputMsg>,
}

struct InputThread {
    rx: Receiver<InputMsg>,
    contoller_tx: Sender<ControllerMsg>,
}

struct SequencerThread {
    rx: Receiver<SequencerMsg>,
    controller_tx: Sender<ControllerMsg>,
}

struct AudioServerThread {
    rx: Receiver<AudioServerMsg>,
    controller_tx: Sender<ControllerMsg>,
}

enum ControllerMsg {
}

enum InputMsg {
}

enum SequencerMsg {
}

enum AudioServerMsg {
}
