extern crate cpal;

use self::cpal::{
    EventLoop,
    Device,
    Format,
    StreamId,
    StreamData,
    UnknownTypeOutputBuffer,
};


pub struct Generator {
    event_loop: EventLoop,
    device: Device,
    format: Format,
    stream: StreamId,
}

// impl Debug for Generator {
    
// }

impl Generator {
    pub fn new() -> Generator {
        let event_loop = EventLoop::new();
        let device = cpal::default_output_device()
            .expect("no output device available");
        let mut supported_formats_range = device
            .supported_output_formats()
            .expect("error while querying formats");
        let format = supported_formats_range.next()
            .expect("no supported format?!")
            .with_max_sample_rate();
        let stream_id = event_loop
            .build_output_stream(&device, &format)
            .unwrap();

        Generator {event_loop, device, format, stream: stream_id}
    }

    pub fn play(&self) {
        let sample_rate = self.format.sample_rate.0 as f32;
        let mut sample_clock = 0f32;

        // Produce a sinusoid of maximum amplitude.
        let mut next_value = || {
            sample_clock = (sample_clock + 1.0) % sample_rate;
            (sample_clock * 440.0 * 2.0 * 3.141592 / sample_rate).sin()
        };
        

        self.event_loop.run(move |_, data| {
            match data {
                cpal::StreamData::Output { buffer: cpal::UnknownTypeOutputBuffer::U16(mut buffer) } => {
                    for sample in buffer.chunks_mut(self.format.channels as usize) {
                        let value = ((next_value() * 0.5 + 0.5) * std::u16::MAX as f32) as u16;
                        for out in sample.iter_mut() {
                            *out = value;
                        }
                    }
                },
                cpal::StreamData::Output { buffer: cpal::UnknownTypeOutputBuffer::I16(mut buffer) } => {
                    for sample in buffer.chunks_mut(self.format.channels as usize) {
                        let value = (next_value() * std::i16::MAX as f32) as i16;
                        for out in sample.iter_mut() {
                            *out = value;
                        }
                    }
                },
                cpal::StreamData::Output { buffer: cpal::UnknownTypeOutputBuffer::F32(mut buffer) } => {
                    for sample in buffer.chunks_mut(self.format.channels as usize) {
                        let value = next_value();
                        for out in sample.iter_mut() {
                            *out = value;
                        }
                    }
                },
                _ => (),
            }
        })
    }
}
