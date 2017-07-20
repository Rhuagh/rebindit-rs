#[macro_use]
extern crate log;

extern crate input;
extern crate glutin;

use input::raw::{RawInputSource, RawInput, RawInputEvent, RawInputAction, RawInputModifiers};
use input::types::{DeviceType, WindowPosition};

pub struct GlutinInputSource {
    events_loop : glutin::EventsLoop,
    last_cursor_position : Option<WindowPosition>,
    current_size : (f64, f64),
}

impl GlutinInputSource {
    pub fn new(events_loop: glutin::EventsLoop, current_size : (f64, f64)) -> GlutinInputSource {
        GlutinInputSource {
            events_loop : events_loop,
            last_cursor_position : None,
            current_size: current_size,
        }
    }
}

impl RawInputSource for GlutinInputSource {

    fn process(&mut self) -> Vec<RawInput> {
        let mut raw = vec![];
        self.events_loop.poll_events(|event| {
            println!("{:?}", event);
        });
        raw
    }

}
