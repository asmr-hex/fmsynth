extern crate cpal;

use self::cpal::{ EventLoop, Device, Format, StreamId };


pub struct Generator {
    event_loop: EventLoop,
    device: Device,
    format: Format,
    stream: StreamId,
}

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
}
