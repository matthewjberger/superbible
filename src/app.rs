extern crate glfw;

use self::glfw::{Action, Context, Key};

use std::sync::mpsc::Receiver;

pub struct Application {
    window: Option<glfw::Window>,
    app_context: Option<glfw::Glfw>,
    event_context: Option<Receiver<(f64, glfw::WindowEvent)>>
}

impl Application {

    pub fn new() -> Application {
        Application {
            window: None,
            app_context: None,
            event_context: None
        }
    }

    pub fn init(&mut self, width: u32, height: u32, title: &str, window_mode: glfw::WindowMode) {
        self.app_context = Some(glfw::init(glfw::FAIL_ON_ERRORS).unwrap());
        let (window, events) = self.app_context.as_ref().unwrap().create_window(width, height, title, window_mode)
            .expect("Failed to create GLFW window.");
        self.window = Some(window);
        self.event_context = Some(events);
        self.window.as_mut().unwrap().set_key_polling(true);
        self.window.as_mut().unwrap().make_current();
    }

    pub fn run(&mut self) {
        while !self.window.as_mut().unwrap().should_close() {
            self.app_context.as_mut().unwrap().poll_events();
            for (_, event) in glfw::flush_messages(&self.event_context.as_mut().unwrap()) {
                handle_window_event(&mut self.window.as_mut().unwrap(), event);
            }
        }
    }

}

fn handle_window_event(window: &mut glfw::Window, event: glfw::WindowEvent) {
    match event {
        glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
            window.set_should_close(true)
        }
        _ => {}
    }
}
