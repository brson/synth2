use anyhow::Result;
use cpal::traits::{HostTrait, DeviceTrait, StreamTrait};
use cpal::{SampleFormat, Sample, ChannelCount};
use std::sync::mpsc;

pub struct Player {
    pub buf_filled_tx: mpsc::SyncSender<Buffer>,
    pub buf_empty_rx: mpsc::Receiver<Buffer>,
    stream: Box<dyn StreamTrait>,
}

pub const BUFFER_FRAMES: usize = 4096;

/// This buffer only takes a single channel.
pub struct Buffer(Box<[f32]>);

impl Buffer {
    fn as_slice_mut(&mut self) -> &mut [f32] {
        &mut self.0
    }
}

pub fn start_player() -> Result<Option<Player>> {
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

        let supported_config = output_device.default_output_config()?;
        log::info!("default output config: {:#?}", supported_config);

        let sample_format = supported_config.sample_format();
        let config = cpal::StreamConfig::from(supported_config);

        let (buf_filled_tx, buf_filled_rx) = mpsc::sync_channel(2);
        let (buf_empty_tx, buf_empty_rx) = mpsc::sync_channel(2);

        buf_empty_tx.send(Buffer(Box::from([0_f32; BUFFER_FRAMES])));
        buf_empty_tx.send(Buffer(Box::from([0_f32; BUFFER_FRAMES])));

        let handle_error = |error| {
            log::error!("audio output error: {}", error);
        };

        let mut state = State {
            output_channels: config.channels,
            pending_frames: Vec::with_capacity(BUFFER_FRAMES),
            buf_filled_rx,
            buf_empty_tx,
        };

        let stream = match sample_format {
            SampleFormat::I16 => {
                output_device.build_output_stream(
                    &config,
                    move |buffer: &mut [i16], info| {
                        fill_buffer(buffer, &mut state)
                    },
                    handle_error,
                )?
            }
            SampleFormat::U16 => {
                output_device.build_output_stream(
                    &config,
                    move |buffer: &mut [u16], info| {
                        fill_buffer(buffer, &mut state)
                    },
                    handle_error,
                )?
            }
            SampleFormat::F32 => {
                output_device.build_output_stream(
                    &config,
                    move |buffer: &mut [f32], info| {
                        fill_buffer(buffer, &mut state)
                    },
                    handle_error,
                )?
            }
        };

        stream.play()?;

        Ok(Some(Player {
            buf_filled_tx,
            buf_empty_rx,
            stream: Box::from(stream),
        }))
    } else {
        Ok(None)
    }
}

struct State {
    output_channels: ChannelCount,
    pending_frames: Vec<f32>,
    buf_filled_rx: mpsc::Receiver<Buffer>,
    buf_empty_tx: mpsc::SyncSender<Buffer>,
}

fn fill_buffer<S>(
    buffer: &mut [S],
    state: &mut State,
)
where S: Sample
{
    todo!()
}
