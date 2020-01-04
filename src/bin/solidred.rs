use gl::types::*;
use glfw::{Action, Context, Key};

fn main() {
    let mut context = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    let (mut window, events) = context
        .create_window(600, 600, "Solid Red", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window.");

    window.make_current();
    window.set_key_polling(true);
    window.set_framebuffer_size_polling(true);

    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

    while !window.should_close() {
        context.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            if let glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) = event {
                window.set_should_close(true)
            }
        }
        let red: [GLfloat; 4] = [1.0, 0.0, 0.0, 0.0];
        unsafe {
            gl::ClearBufferfv(gl::COLOR, 0, &red as *const f32);
        }
        window.swap_buffers();
    }
}
