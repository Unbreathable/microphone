use std::{thread, time::Duration};

use cpal::{traits::{DeviceTrait, HostTrait, StreamTrait}, StreamConfig};

pub fn record() {
    thread::spawn(move || {
        // Get a cpal host
        let host = cpal::default_host(); // Current host on computer

        // Get input device (using new API)
        let device = host
            .default_input_device()
            .expect("no input device available"); // Current device

        // Create a stream config
        let default_config = device
            .default_input_config()
            .expect("no stream config found");
        
        let sample_rate: u32 = default_config.sample_rate().0;
        let _work_channels = 1; // Stereo doesn't work at the moment (will fix in the future or never)
        let mic_channels = default_config.channels();
        let config: StreamConfig = StreamConfig {
            channels: mic_channels,
            sample_rate: cpal::SampleRate(sample_rate),
            buffer_size: cpal::BufferSize::Fixed(4096),
        };

        // Create a stream
        let stream = match device.build_input_stream(
            &config.into(),
            move |data: &[f32], _: &_| {

                let mut max = 0.0;
                for sample in data.iter() {
                    if *sample > max {
                        max = *sample;
                    }
                }

                println!("Max: {}", max);
            },
            move |err| {
                println!("an error occurred on stream: {}", err);
            },
            None,
        ) {
            Ok(stream) => stream,
            Err(_) => {
                return;
            }
        };

        // Play the stream
        stream.play().unwrap();

        loop {
            thread::sleep(Duration::from_millis(100));
        }
    });
}