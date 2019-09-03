pub use gl::types::*;
pub use glfw::{Action, Context, Key};

pub struct AppSettings {
    pub initial_width: u32,
    pub initial_height: u32,
    pub title: String,
}

impl Default for AppSettings {
    fn default() -> Self {
        AppSettings {
            initial_width: 800,
            initial_height: 600,
            title: "OpenGL Application".to_string(),
        }
    }
}

pub trait App {
    fn settings(&mut self) -> &AppSettings;
    fn initialize(&mut self) {}
    fn update(&mut self) {}
    fn render(&mut self, _: f32) {}
    fn cleanup(&mut self) {}
    fn on_resize(&mut self, _: i32, _: i32) {}
    fn on_key(&mut self, _: Key, _: Action) {}
}

pub fn run<T: App>(app: &mut T) {
    let settings = app.settings();
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).expect("Failed to initialize glfw.");
    let (mut window, events) = glfw
        .create_window(
            settings.initial_width,
            settings.initial_height,
            &settings.title,
            glfw::WindowMode::Windowed,
        )
        .expect("Failed to create GLFW window.");

    window.make_current();
    window.set_key_polling(true);
    window.set_framebuffer_size_polling(true);

    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

    app.initialize();

    while !window.should_close() {
        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            match event {
                glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                    window.set_should_close(true)
                }
                glfw::WindowEvent::Key(key, _, action, _) => app.on_key(key, action),
                glfw::WindowEvent::FramebufferSize(width, height) => {
                    unsafe {
                        gl::Viewport(0, 0, width, height);
                    }
                    app.on_resize(width, height);
                }
                _ => {}
            }
        }

        app.update();
        app.render(glfw.get_time() as f32);
        window.swap_buffers();
    }

    app.cleanup();
}
