use gl::types::*;
use glfw::{Action, Context, Key};
use std::ffi::CString;
use std::ptr;

static VERTEX_SHADER_SOURCE: &'static str = "
#version 450 core
void main(void) {
    gl_Position = vec4(0.0, 0.0, 0.5, 1.0);
}
";

static FRAGMENT_SHADER_SOURCE: &'static str = "
#version 450 core
out vec4 color;
void main(void)
{
    color = vec4(0.0, 0.8, 1.0, 1.0);
}
";

fn main() {
    let mut context = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    let (mut window, events) = context
        .create_window(600, 600, "OpenGL", glfw::WindowMode::Windowed)
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
            handle_window_event(&mut window, event)
        }
        render();
        window.swap_buffers();
    }
}

fn render() {
    unsafe {
        gl::ClearColor(0.2, 0.3, 0.3, 1.0);
        // gl::Clear(gl::COLOR_BUFFER_BIT);
        let red: [GLfloat; 4] = [1.0, 0.0, 0.0, 0.0];
        gl::ClearBufferfv(gl::COLOR, 0, &red as *const f32);
    }
}

fn handle_window_event(window: &mut glfw::Window, event: glfw::WindowEvent) {
    if let glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) = event {
        window.set_should_close(true)
    }
}

fn compile_shaders() -> GLuint {
    let vertex_shader;
    let fragment_shader;
    let shader_program;

    unsafe {
        let vertex_src_str = CString::new(VERTEX_SHADER_SOURCE.as_bytes()).unwrap();
        vertex_shader = gl::CreateShader(gl::VERTEX_SHADER);
        gl::ShaderSource(vertex_shader, 1, &vertex_src_str.as_ptr(), ptr::null());
        gl::CompileShader(vertex_shader);

        let fragment_src_str = CString::new(FRAGMENT_SHADER_SOURCE.as_bytes()).unwrap();
        fragment_shader = gl::CreateShader(gl::FRAGMENT_SHADER);
        gl::ShaderSource(fragment_shader, 1, &fragment_src_str.as_ptr(), ptr::null());
        gl::CompileShader(fragment_shader);

        shader_program = gl::CreateProgram();
        gl::AttachShader(shader_program, vertex_shader);
        gl::AttachShader(shader_program, fragment_shader);
        gl::LinkProgram(shader_program);

        gl::DeleteShader(vertex_shader);
        gl::DeleteShader(fragment_shader);

        shader_program
    }
}
