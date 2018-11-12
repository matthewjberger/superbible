mod app;

fn main() {
    let mut application = app::Application::new();
    application.init(300,300,"test", glfw::WindowMode::Windowed);
    application.run();
}
