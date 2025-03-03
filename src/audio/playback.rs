use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use ringbuf::HeapRb;
use std::fs::File;
use std::path::PathBuf;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;
use symphonia::core::audio::SampleBuffer;
use symphonia::core::codecs::DecoderOptions;
use symphonia::core::formats::FormatOptions;
use symphonia::core::io::MediaSourceStream;
use symphonia::core::meta::MetadataOptions;
use symphonia::core::probe::Hint;

use crate::structs::audio_playback_state::AudioPlaybackState;
use crate::structs::sample_wrapper::SampleWrapper;

pub fn start_audio_playback(
    path: PathBuf,
    tx: mpsc::Sender<SampleWrapper>,
    playback_state: std::sync::Arc<std::sync::Mutex<AudioPlaybackState>>,
) -> std::thread::JoinHandle<()> {
    thread::spawn(move || {
        println!("Starting audio playback thread...");

        let ring = HeapRb::new(32768);
        let (mut producer, mut consumer) = ring.split();

        // Set up audio output
        let host = cpal::default_host();
        let device = host.default_output_device().expect("no output device");
        let config = device
            .default_output_config()
            .expect("no default output config");
        let output_sample_rate = config.sample_rate().0 as f32;

        let stream = device
            .build_output_stream(
                &config.into(),
                move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                    for sample in data.iter_mut() {
                        *sample = consumer.pop().unwrap_or(0.0);
                    }
                },
                |err| eprintln!("Audio stream error: {}", err),
                None,
            )
            .expect("failed to build output stream");

        println!("Starting audio stream...");
        stream.play().expect("failed to play stream");

        // Decoder setup
        let codec_registry = symphonia::default::get_codecs();
        let probe = symphonia::default::get_probe();
        let file = Box::new(File::open(&path).unwrap());
        let mss = MediaSourceStream::new(file, Default::default());

        let probed = probe
            .format(
                &Hint::new(),
                mss,
                &FormatOptions::default(),
                &MetadataOptions::default(),
            )
            .expect("unsupported format");

        let mut format = probed.format;

        // Decode setup
        let track = format.default_track().unwrap();

        if let Some(track) = format.default_track() {
            // Get time base and number of frames
            if let (Some(tb), Some(frames)) =
                (track.codec_params.time_base, track.codec_params.n_frames)
            {
                // Calculate duration in seconds
                let duration = frames as f64 * tb.numer as f64 / tb.denom as f64;

                let mut state = playback_state.lock().unwrap();
                state.set_song_duration(duration as u32);
            }
        }

        let input_sample_rate = track.codec_params.sample_rate.unwrap() as f32;
        let sample_rate = track.codec_params.sample_rate.unwrap() as u32;
        let mut samples_played = 0;

        // Adjust this ratio with a small correction factor
        let correction_factor = 1.0; // Try adjusting this value between 1.0 and 1.1
        let sample_rate_ratio = (output_sample_rate / input_sample_rate) * correction_factor;

        let mut decoder = codec_registry
            .make(&track.codec_params, &DecoderOptions::default())
            .expect("unsupported codec");

        let mut sample_buf = None;
        let mut resampled_buffer = Vec::new();

        println!("Starting decode loop...");
        loop {
            // Check playback state
            {
                let state = playback_state.lock().unwrap();
                if state.should_stop {
                    println!("Playback stopped");
                    break;
                }
                if !state.is_playing {
                    continue;
                }
            }

            match format.next_packet() {
                Ok(packet) => {
                    let decoded = decoder.decode(&packet).expect("decode error");
                    let num_channels = decoded.spec().channels.count() as u32;
                    if sample_buf.is_none() {
                        sample_buf = Some(SampleBuffer::<f32>::new(
                            decoded.capacity() as u64,
                            *decoded.spec(),
                        ));
                    }

                    if let Some(buf) = &mut sample_buf {
                        buf.copy_interleaved_ref(decoded);
                        let samples: Vec<f32> = buf.samples().to_vec();

                        // Update position
                        samples_played += (samples.len() as u32) / num_channels;  // Divide by number of channels
                        let position = samples_played / sample_rate;
                        {
                            let mut state = playback_state.lock().unwrap();
                            state.set_song_position(position);
                        }

                        // Simple linear interpolation for resampling
                        resampled_buffer.clear();
                        let mut sample_index = 0.0;
                        while sample_index < samples.len() as f32 {
                            let index = sample_index as usize;
                            if index + 1 < samples.len() {
                                let fraction = sample_index - index as f32;
                                let sample = samples[index] * (1.0 - fraction)
                                    + samples[index + 1] * fraction;
                                resampled_buffer.push(sample);
                            }
                            sample_index += 1.0 / sample_rate_ratio;
                        }

                        tx.send(SampleWrapper {
                            samples: resampled_buffer.clone(),
                        })
                        .ok();

                        for &sample in resampled_buffer.iter() {
                            while producer.is_full() {
                                thread::sleep(Duration::from_micros(100));
                            }
                            producer.push(sample).ok();
                        }
                    }
                }
                Err(e) => {
                    println!("Error reading packet: {:?}", e);
                    break;
                }
            }
        }

        println!("Audio playback thread ending...");
    })
}