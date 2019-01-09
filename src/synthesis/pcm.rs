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
        self.event_loop.run(move |_stream_id, mut stream_data| {
            match stream_data {
                StreamData::Output { buffer: UnknownTypeOutputBuffer::U16(mut buffer) } => {
                    for elem in buffer.iter_mut() {
                        *elem = u16::max_value() / 2;
                    }
                },
                StreamData::Output { buffer: UnknownTypeOutputBuffer::I16(mut buffer) } => {
                    for elem in buffer.iter_mut() {
                        *elem = 0;
                    }
                },
                StreamData::Output { buffer: UnknownTypeOutputBuffer::F32(mut buffer) } => {
                    for elem in buffer.iter_mut() {
                        *elem = 0.0;
                    }
                },
                _ => (),
            }
        })
    }
}
