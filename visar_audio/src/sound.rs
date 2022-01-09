use anyhow::{anyhow, Result};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
pub use crossbeam::channel::{unbounded, Receiver, Sender};
pub struct AudioController {
    device: cpal::Device,
    config: cpal::SupportedStreamConfig,
    pub sample_format: Format,
    pub command_sender: crossbeam::channel::Sender<Message>,
    command_receiver: crossbeam::channel::Receiver<Message>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Message {
    Frequency(f32),
    Note(f32),
}
pub enum Format {
    F32format,
    I16format,
    U16format,
}

impl AudioController {
    pub fn new() -> Result<AudioController, anyhow::Error> {
        let host = cpal::default_host();

        let device = match host.default_output_device() {
            Some(device) => device,
            None => {
                return Err(anyhow!("Failed to get audio device"));
            }
        };

        let (command_sender, command_receiver) = crossbeam::channel::unbounded();

        let config = device.default_output_config()?;
        Ok(AudioController {
            device,
            sample_format: match &config.sample_format() {
                cpal::SampleFormat::F32 => Format::F32format,
                cpal::SampleFormat::I16 => Format::I16format,
                cpal::SampleFormat::U16 => Format::U16format,
            },
            config,
            command_sender,
            command_receiver,
        })
    }
    pub fn run<T>(audio_controller: AudioController) -> Result<(), anyhow::Error>
    where
        T: cpal::Sample,
    {
        let sample_rate = audio_controller.config.sample_rate().0 as f32;
        let channels = audio_controller.config.channels() as usize;

        let err_fn = |err| eprintln!("an error occured on stream: {}", err);

        let mut phi = 0.0f32;
        let mut frequency = 440.0;
        let amplitude = 0.01;
        let mut note = 0.0;

        let stream = audio_controller.device.build_output_stream(
            &audio_controller.config.into(),
            move |data: &mut [T], _: &cpal::OutputCallbackInfo| {
                for frame in data.chunks_mut(channels) {
                    while let Ok(command) = audio_controller.command_receiver.try_recv() {
                        match command {
                            Message::Note(val) => {
                                note = val;
                            }
                            Message::Frequency(val) => {
                                frequency = (val * (2000.0 - 440.0)) + 440.0;
                            }
                        }
                    }

                    phi = (phi + (frequency / sample_rate)).fract();

                    let sound =
                        |phi: f32| -> f32 { amplitude * note * (2.0f32 * 3.141592 * phi).sin() };

                    let value: T = cpal::Sample::from::<f32>(&sound(phi));

                    for sample in frame.iter_mut() {
                        *sample = value;
                    }
                }
            },
            err_fn,
        )?;

        stream.play()?;

        std::thread::park();

        Ok(())
    }
}
