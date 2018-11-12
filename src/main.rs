mod core;
use core::Application;

fn main() {
    let mut application = Application::new(300, 300, "title", glfw::WindowMode::Windowed);
}
