extern crate gl;
extern crate glfw;

use self::gl::types::*;
use self::glfw::{Action, Key};

use std::ptr;
use std::ffi::CString;

mod core;
use core::Application;


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
    let mut _application = Application::new(600, 600, "OpenGL", glfw::WindowMode::Windowed);
    _application.run(render, handle_window_event);
}

fn render() {
    unsafe {
        gl::ClearColor(0.2, 0.3, 0.3, 1.0);
        // gl::Clear(gl::COLOR_BUFFER_BIT);
        let red: [GLfloat;4] = [1.0, 0.0, 0.0, 0.0];
        gl::ClearBufferfv(gl::COLOR, 0, &red as *const f32);
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

fn compile_shaders() -> GLuint
{
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
