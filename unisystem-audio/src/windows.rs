use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use std::sync::{Arc, Mutex};
use anyhow::Result;

pub fn start_audio_capture<F>(callback: F, stop_flag: Arc<Mutex<bool>>) -> Result<()>
where
    F: Fn(&[f32]) + Send + 'static,
{
    let host = cpal::default_host();
    
    // For Windows loopback, we open the default output device and build an input stream on it.
    let device = host.default_output_device()
        .ok_or_else(|| anyhow::anyhow!("No default output device available for loopback capture"))?;
    
    let config = device.default_output_config()?;
    let stream_config = config.config();

    let stream = match config.sample_format() {
        cpal::SampleFormat::F32 => {
            device.build_input_stream(
                &stream_config,
                move |data: &[f32], _: &_| callback(data),
                |err| eprintln!("Audio stream error: {}", err),
                None
            )?
        }
        cpal::SampleFormat::I16 => {
            device.build_input_stream(
                &stream_config,
                move |data: &[i16], _: &_| {
                    let f32_data: Vec<f32> = data.iter().map(|&s| s as f32 / i16::MAX as f32).collect();
                    callback(&f32_data);
                },
                |err| eprintln!("Audio stream error: {}", err),
                None
            )?
        }
        cpal::SampleFormat::U16 => {
            device.build_input_stream(
                &stream_config,
                move |data: &[u16], _: &_| {
                    let f32_data: Vec<f32> = data.iter().map(|&s| (s as f32 - u16::MAX as f32 / 2.0) / (u16::MAX as f32 / 2.0)).collect();
                    callback(&f32_data);
                },
                |err| eprintln!("Audio stream error: {}", err),
                None
            )?
        }
        _ => return Err(anyhow::anyhow!("Unsupported sample format for capture")),
    };

    stream.play()?;

    // Block thread until stop_flag is true
    loop {
        if *stop_flag.lock().unwrap() {
            break;
        }
        std::thread::sleep(std::time::Duration::from_millis(100));
    }

    Ok(())
}
