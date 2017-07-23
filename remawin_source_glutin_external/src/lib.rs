extern crate remawin;
extern crate glutin;
extern crate time;
extern crate remawin_glutin_mapper;

use remawin::raw::{RawInputSource, RawInput};
use remawin::types::WindowPosition;
use remawin_glutin_mapper::{NextData, GlutinEventMapper};

pub struct GlutinExternalInputSource {
    last_cursor_position : Option<WindowPosition>,
    current_size : (f64, f64),
    events : Vec<glutin::Event>
}

impl GlutinExternalInputSource {
    pub fn new(current_size : (f64, f64)) -> GlutinExternalInputSource {
        GlutinExternalInputSource {
            last_cursor_position: None,
            current_size: current_size,
            events: Vec::default()
        }
    }

    pub fn push_events(&mut self, events : &mut Vec<glutin::Event>) {
        self.events.append(events);
    }

}

impl RawInputSource for GlutinExternalInputSource {

    fn process(&mut self) -> Vec<RawInput> {
        let mut events = Vec::new();
        events.append(&mut self.events);
        let mut next = NextData {
            size : self.current_size.clone(),
            cursor_position : self.last_cursor_position.clone()
        };
        let raw = events.iter().flat_map(|e| GlutinEventMapper::process_event(&e, &mut next)).collect();
        self.last_cursor_position = next.cursor_position;
        self.current_size = next.size;

        raw
    }

}
