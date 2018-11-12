extern crate glfw;

use self::glfw::{Action, Context, Key};
use std::sync::mpsc::Receiver;

pub struct Application {
    window: glfw::Window,
    app_context: glfw::Glfw,
    event_context: Receiver<(f64, glfw::WindowEvent)>
}

impl Application {
    pub fn new(width: u32, height: u32, title: &str, window_mode: glfw::WindowMode) -> Application {
        let mut context = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
        let (mut window, events) = context.create_window(width, height, title, window_mode)
            .expect("Failed to create GLFW window.");
        window.set_key_polling(true);
        window.make_current();

        Application {
            window: window,
            app_context: context,
            event_context: events
        }
    }
}
