use anyhow::Result;
use cpal::traits::{HostTrait, DeviceTrait, StreamTrait};
use std::sync::mpsc;

pub struct Player {
    pub buf_filled_tx: mpsc::Sender<Buffer>,
    pub buf_empty_rx: mpsc::Sender<Buffer>,
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

        let config = output_device.default_output_config()?;
        log::info!("default output config: {:#?}", config);
        let config = cpal::StreamConfig::from(config);

        let stream = output_device.build_output_stream(
            &config,
            |buffer: &mut [f32], info| {
            },
            |error| {
                log::error!("audio output error: {}", error);
            }
        )?;

        todo!();
    } else {
        Ok(None)
    }
}
