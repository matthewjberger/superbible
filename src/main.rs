extern crate gl;
extern crate glfw;

// use self::gl::types::*;
use self::glfw::{Action, Key};

mod core;
use core::Application;

fn main() {
    let mut _application = Application::new(600, 600, "OpenGL", glfw::WindowMode::Windowed);
    _application.run(render, handle_window_event);
}

fn render() {
    unsafe {
        gl::ClearColor(0.2, 0.3, 0.3, 1.0);
        gl::Clear(gl::COLOR_BUFFER_BIT);
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
