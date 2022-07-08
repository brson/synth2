use anyhow::Result;
use cpal::traits::{HostTrait, DeviceTrait, StreamTrait};
use cpal::{SampleFormat, Sample};
use std::sync::mpsc;

pub struct Player {
    pub buf_filled_tx: mpsc::Sender<Buffer>,
    pub buf_empty_rx: mpsc::Receiver<Buffer>,
    pub stream: Box<dyn StreamTrait>,
}

pub const BUFFER_FRAMES: usize = 4096;

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

        let (buf_filled_tx, buf_filled_rx) = mpsc::channel();
        let (buf_empty_tx, buf_empty_rx) = mpsc::channel();

        let handle_error = |error| {
            log::error!("audio output error: {}", error);
        };

        let stream = match sample_format {
            SampleFormat::I16 => {
                output_device.build_output_stream(
                    &config,
                    move |buffer: &mut [i16], info| {
                        fill_buffer(buffer, &buf_filled_rx, &buf_empty_tx)
                    },
                    handle_error,
                )?
            }
            SampleFormat::U16 => todo!(),
            SampleFormat::F32 => todo!(),
        };

        Ok(Some(Player {
            buf_filled_tx,
            buf_empty_rx,
            stream: Box::from(stream),
        }))
    } else {
        Ok(None)
    }
}

fn fill_buffer<S>(
    buffer: &mut [S],
    buf_filled_rx: &mpsc::Receiver<Buffer>,
    buf_empty_tx: &mpsc::Sender<Buffer>,
)
where S: Sample
{
    todo!()
}
