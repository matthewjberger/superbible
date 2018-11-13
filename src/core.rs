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
        let context = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
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

    pub fn run(&mut self, handle_window_event: fn(&mut glfw::Window, glfw::WindowEvent)) {
        while !self.window.should_close() {
            self.app_context.poll_events();
            for (_, event) in glfw::flush_messages(&self.event_context) {
                match event {
                    glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                        self.window.set_should_close(true)
                    },
                    _ => ()
                }
                handle_window_event(&mut self.window, event)
            }
        }
    }
}
