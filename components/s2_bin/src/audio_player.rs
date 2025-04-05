use anyhow::Result;
use cpal::traits::{HostTrait, DeviceTrait, StreamTrait};
use cpal::{SampleFormat, Sample, FromSample, ChannelCount};
use std::sync::mpsc;

pub struct Player {
    pub channels: PlayerChannels,
    pub stream: PlayerStream,
}

pub struct PlayerChannels {
    pub sample_rate: u32,
    pub buf_filled_tx: mpsc::SyncSender<Buffer>,
    pub buf_empty_rx: mpsc::Receiver<Buffer>,
}

pub struct PlayerStream {
    stream: Box<dyn StreamTrait>,
}

pub const BUFFER_FRAMES: usize = 2048;

/// This buffer only takes a single channel.
pub struct Buffer(Box<[f32]>);

impl Buffer {
    pub fn as_slice_mut(&mut self) -> &mut [f32] {
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
            pending_buffer: None,
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
                    None,
                )?
            }
            SampleFormat::U16 => {
                output_device.build_output_stream(
                    &config,
                    move |buffer: &mut [u16], info| {
                        fill_buffer(buffer, &mut state)
                    },
                    handle_error,
                    None,
                )?
            }
            SampleFormat::F32 => {
                output_device.build_output_stream(
                    &config,
                    move |buffer: &mut [f32], info| {
                        fill_buffer(buffer, &mut state)
                    },
                    handle_error,
                    None,
                )?
            }
            _ => todo!(),
        };

        stream.play()?;

        Ok(Some(Player {
            channels: PlayerChannels {
                sample_rate: config.sample_rate.0,
                buf_filled_tx,
                buf_empty_rx,
            },
            stream: PlayerStream {
                stream: Box::from(stream),
            },
        }))
    } else {
        Ok(None)
    }
}

struct State {
    output_channels: ChannelCount,
    pending_buffer: Option<PendingBuffer>,
    buf_filled_rx: mpsc::Receiver<Buffer>,
    buf_empty_tx: mpsc::SyncSender<Buffer>,
}

struct PendingBuffer {
    buf: Buffer,
    consumed: usize,
}

fn fill_buffer<S>(
    buffer: &mut [S],
    state: &mut State,
)
where S: Sample,
      S: FromSample<f32>,
{
    let output_channels = state.output_channels as usize;
    assert!(buffer.len() % output_channels == 0);

    let total_frames_to_write = buffer.len() / output_channels;

    // todo we can probably cut BUFFER_FRAMES in half
    // and make this total_frames_to_write > BUFFER_FRAMES * 2,
    // make two attempts to recv.
    if total_frames_to_write > BUFFER_FRAMES {
        log::error!("audio device requesting {} frames, but buffer is only {} frames", total_frames_to_write, BUFFER_FRAMES);
    }

    // First fill the output buffer from a buffer we already have.
    let initial_frames_written = fill_buffer_from_pending(buffer, state);
    assert!(initial_frames_written <= total_frames_to_write);

    if initial_frames_written == total_frames_to_write {
        return;
    }

    assert!(state.pending_buffer.is_none());

    let buffer = &mut buffer[output_channels * initial_frames_written ..];
    let final_frames_to_write = total_frames_to_write - initial_frames_written;

    let fill_remaining_zeros = |buffer: &mut [S]| {
        let out_frames = buffer.chunks_mut(output_channels);
        for out_frame in out_frames {
            for sample in out_frame.iter_mut() {
                *sample = S::from_sample(0.0);
            }
        }
    };

    // Receive a new buffer.
    match state.buf_filled_rx.try_recv() {
        Ok(new_buffer) => {
            state.pending_buffer = Some(PendingBuffer {
                buf: new_buffer,
                consumed: 0,
            });
            let final_frames_written = fill_buffer_from_pending(buffer, state);
            assert!(final_frames_written <= final_frames_to_write);

            let buffer = &mut buffer[output_channels * final_frames_written ..];
            fill_remaining_zeros(buffer);
        }
        Err(mpsc::TryRecvError::Disconnected) => {
            /* shutting down */
            fill_remaining_zeros(buffer);
        }
        Err(mpsc::TryRecvError::Empty) => {
            log::warn!("didn't receive buffer in time for audio out");
            fill_remaining_zeros(buffer);
        }
    }
}

fn fill_buffer_from_pending<S>(
    buffer: &mut [S],
    state: &mut State,
) -> usize
where S: Sample,
      S: FromSample<f32>,
{
    let output_channels = state.output_channels as usize;
    assert!(buffer.len() % output_channels == 0);

    let frames_to_write = buffer.len() / output_channels;
    let mut frames_written = 0;

    if let Some(mut pending_buffer) = state.pending_buffer.take() {

        assert!(pending_buffer.consumed < pending_buffer.buf.0.len());

        let remaining_buffer = &pending_buffer.buf.0[pending_buffer.consumed..];
        let frames_to_write = frames_to_write.min(remaining_buffer.len());
        let in_frames = remaining_buffer.iter().take(frames_to_write);
        let out_frames = buffer.chunks_mut(output_channels).take(frames_to_write);
        let frames = in_frames.zip(out_frames);

        for (in_frame, out_frame) in frames {
            for sample in out_frame.iter_mut() {
                *sample = S::from_sample(*in_frame);
            }
        }

        pending_buffer.consumed += frames_to_write;
        frames_written += frames_to_write;

        assert!(pending_buffer.consumed <= pending_buffer.buf.0.len());

        if pending_buffer.consumed < pending_buffer.buf.0.len() {
            state.pending_buffer = Some(pending_buffer);
        } else {
            match state.buf_empty_tx.try_send(pending_buffer.buf) {
                Ok(_) => { },
                Err(mpsc::TrySendError::Disconnected(_)) => {
                    /* shutting down */
                }
                Err(mpsc::TrySendError::Full(_)) => {
                    panic!("full channel");
                }
            }
        }
    }

    frames_written
}
