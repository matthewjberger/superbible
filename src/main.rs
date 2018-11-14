extern crate glfw;
use self::glfw::{Action, Key};

mod core;
use core::Application;

fn main() {
    let mut _application = Application::new(300, 300, "title", glfw::WindowMode::Windowed);
    _application.run(render, handle_window_event);
}

fn render() {
    
}


fn handle_window_event(window: &mut glfw::Window, event: glfw::WindowEvent) {
    match event {
        glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
            window.set_should_close(true)
        }
        _ => {}
    }
}
