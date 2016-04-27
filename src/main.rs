#![feature(op_assign_traits)]

#[macro_use]
extern crate log;
#[macro_use]
extern crate bitflags;
extern crate glfw;
extern crate time;
extern crate yaml_rust;

use std::sync::mpsc::Receiver;
use glfw::Context;

use input::raw::Adapter;
use input::raw::glfw::GlfwAdapter;
use input::InputHandler;

pub mod logger;
pub mod input;

fn window_init(width : u32, height: u32, title : &str) -> (glfw::Glfw, glfw::Window, Receiver<(f64, glfw::WindowEvent)>) {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    glfw.window_hint(glfw::WindowHint::ContextVersion(3, 2));
    glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));
    glfw.set_error_callback(glfw::FAIL_ON_ERRORS);
    let (mut window, events) =
        match glfw.create_window(width, height, title, glfw::WindowMode::Windowed) {
            Some(d) => d,
            None => panic!("Window was not created")
        };
    window.set_all_polling(true);
    (glfw, window, events)
}

fn main() {
    logger::init().unwrap();
    debug!("Starting");
    let (glfw, mut window, events) = window_init(1024, 768, "Test");
    debug!("Window initialized");

    let mut input_handler = InputHandler::new()
        .with_raw_adapter(GlfwAdapter::new(glfw, events))
        .with_constants_file("config/constants.yml")
        .with_contexts_file("config/contexts.yml");

    while !window.should_close() {
        for (_, _, v) in input_handler.process() {
            match v {
                input::Input::Action(1, _) => {
                    window.set_should_close(true);
                },
                _ => {
                    debug!("{:?}", v);
                }
            }
        }

        window.swap_buffers();
    }

    return
}
