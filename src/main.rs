use cpal::traits::{DeviceTrait, EventLoopTrait, HostTrait};
use cpal::{StreamData, UnknownTypeOutputBuffer};
use std::f64::consts::PI;

fn main() {
    let host = cpal::default_host();
    let event_loop = host.event_loop();

    let device = host
        .default_output_device()
        .expect("no output device available");

    let mut supported_formats_range = device
        .supported_output_formats()
        .expect("error while querying formats");

    let format = supported_formats_range
        .next()
        .expect("no supported format?!")
        .with_max_sample_rate();

    let stream_id = event_loop.build_output_stream(&device, &format).unwrap();

    event_loop
        .play_stream(stream_id)
        .expect("failed to play_stream");

    let mut sample_clock = 0f32;
    let sample_rate = 20100.0;
    let start_note_fq = 320.0;
    let end_note_fq = 440.0;
    let mut note_fq = 0f32;
    // let mut accumulator = 0f32;
    let mut next_value = move || {
        note_fq = if note_fq == end_note_fq {
            start_note_fq
        } else {
            note_fq + 1.0
        };
        sample_clock = (sample_clock + 1.0) % sample_rate;
        (sample_clock * note_fq * PI as f32 / sample_rate).sin()
    };

    event_loop.run(move |stream_id, stream_result| {
        let stream_data = match stream_result {
            Ok(data) => data,
            Err(err) => {
                eprintln!("an error occurred on stream {:?}: {}", stream_id, err);
                return;
            }
        };

        match stream_data {
            StreamData::Output {
                buffer: UnknownTypeOutputBuffer::U16(mut buffer),
            } => {
                for elem in buffer.iter_mut() {
                    *elem = u16::max_value() / 2;
                }
            }
            StreamData::Output {
                buffer: UnknownTypeOutputBuffer::I16(mut buffer),
            } => {
                for elem in buffer.iter_mut() {
                    *elem = 0;
                }
            }
            StreamData::Output {
                buffer: UnknownTypeOutputBuffer::F32(mut buffer),
            } => {
                for elem in buffer.iter_mut() {
                    *elem = next_value();
                }
            }
            _ => (),
        }
    })
}
