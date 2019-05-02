use gl::types::*;
use glfw::{Action, Context, Key};
use std::time::SystemTime;

fn main() {
    let mut context = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    let (mut window, events) = context
        .create_window(600, 600, "Fading Colors", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window.");

    window.make_current();
    window.set_key_polling(true);
    window.set_framebuffer_size_polling(true);

    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

    let start_time = SystemTime::now();

    while !window.should_close() {
        context.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            if let glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) = event {
                window.set_should_close(true)
            }
            handle_window_event(&mut window, event)
        }
        render(start_time);
        window.swap_buffers();
    }
}

fn render(start_time: SystemTime) {
    let elapsed_time = start_time.elapsed().unwrap();
    let current_time = elapsed_time.as_secs() as f32 + elapsed_time.subsec_nanos() as f32 * 1e-9;

    let color: [GLfloat; 4] = [
        (current_time.sin() * 0.5) + 0.5,
        (current_time.cos() * 0.5) + 0.5,
        0.0,
        1.0,
    ];

    unsafe {
        gl::ClearBufferfv(gl::COLOR, 0, &color as *const f32);
    }
}

fn handle_window_event(window: &mut glfw::Window, event: glfw::WindowEvent) {
    if let glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) = event {
        window.set_should_close(true)
    }
}
