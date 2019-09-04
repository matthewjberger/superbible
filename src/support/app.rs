pub use gl::types::*;
pub use glfw::{Action, Context, Key};

pub trait App {
    fn initialize(&mut self, _: &mut glfw::Window) {}
    fn update(&mut self) {}
    fn render(&mut self, _: f32) {}
    fn cleanup(&mut self) {}
    fn on_resize(&mut self, _: i32, _: i32) {}
    fn on_key(&mut self, _: Key, _: Action) {}
    fn run(&mut self, title: &str) {
        let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).expect("Failed to initialize glfw.");
        let (mut window, events) = glfw
            .create_window(800, 600, title, glfw::WindowMode::Windowed)
            .expect("Failed to create GLFW window.");

        window.make_current();
        window.set_key_polling(true);
        window.set_framebuffer_size_polling(true);

        gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

        self.initialize(&mut window);

        while !window.should_close() {
            glfw.poll_events();
            for (_, event) in glfw::flush_messages(&events) {
                self.handle_events(event, &mut window);
            }

            self.update();
            self.render(glfw.get_time() as f32);
            window.swap_buffers();
        }

        self.cleanup();
    }

    fn handle_events(&mut self, event: glfw::WindowEvent, window: &mut glfw::Window) {
        match event {
            glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                window.set_should_close(true)
            }
            glfw::WindowEvent::Key(key, _, action, _) => self.on_key(key, action),
            glfw::WindowEvent::FramebufferSize(width, height) => {
                unsafe {
                    gl::Viewport(0, 0, width, height);
                }
                self.on_resize(width, height);
            }
            _ => {}
        }
    }
}
