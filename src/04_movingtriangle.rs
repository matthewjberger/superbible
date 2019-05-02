use gl::types::*;
use glfw::{Action, Context, Key};
use std::ffi::CString;
use std::ptr;
use std::time::SystemTime;

static VERTEX_SHADER_SOURCE: &'static str = "
#version 450 core

layout (location = 0) in vec4 offset;

void main(void) {
    const vec4 vertices[3] = vec4[3](vec4( 0.25, -0.25, 0.5, 1.0),
                                     vec4(-0.25, -0.25, 0.5, 1.0),
                                     vec4( 0.25,  0.25, 0.5, 1.0));
    gl_Position = vertices[gl_VertexID] + offset;
}
";

static FRAGMENT_SHADER_SOURCE: &'static str = "
#version 420 core
out vec4 color;
void main(void)
{
    color = vec4(0.0, 0.8, 1.0, 1.0);
}
";

fn main() {
    let mut context = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    let (mut window, events) = context
        .create_window(600, 600, "Moving Triangle", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window.");

    window.make_current();
    window.set_key_polling(true);
    window.set_framebuffer_size_polling(true);

    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

    let start_time = SystemTime::now();

    let shader_program = compile_shaders();

    let mut vao = 0;

    unsafe {
        gl::CreateVertexArrays(1, &mut vao);
        gl::BindVertexArray(vao);
    }

    while !window.should_close() {
        context.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            if let glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) = event {
                window.set_should_close(true)
            }
        }
        render(start_time, shader_program);
        window.swap_buffers();
    }
}

fn render(start_time: SystemTime, shader_program: u32) {
    let elapsed_time = start_time.elapsed().unwrap();
    let current_time = elapsed_time.as_secs() as f32 + elapsed_time.subsec_nanos() as f32 * 1e-9;
    let color: [GLfloat; 4] = [
        (current_time.sin() * 0.5) + 0.5,
        (current_time.cos() * 0.5) + 0.5,
        0.0,
        1.0,
    ];

    let attribute: [GLfloat; 4] = [current_time.sin() * 0.5, current_time.cos() * 0.6, 0.0, 0.0];

    unsafe {
        gl::ClearBufferfv(gl::COLOR, 0, &color as *const f32);
        gl::UseProgram(shader_program);
        gl::VertexAttrib4fv(0, &attribute as *const f32);
        gl::DrawArrays(gl::TRIANGLES, 0, 3);
    }
}

fn compile_shaders() -> GLuint {
    let vertex_shader;
    let fragment_shader;
    let shader_program;

    let vertex_src_str = CString::new(VERTEX_SHADER_SOURCE.as_bytes()).unwrap();
    let fragment_src_str = CString::new(FRAGMENT_SHADER_SOURCE.as_bytes()).unwrap();

    unsafe {
        vertex_shader = gl::CreateShader(gl::VERTEX_SHADER);
        gl::ShaderSource(vertex_shader, 1, &vertex_src_str.as_ptr(), ptr::null());
        gl::CompileShader(vertex_shader);

        fragment_shader = gl::CreateShader(gl::FRAGMENT_SHADER);
        gl::ShaderSource(fragment_shader, 1, &fragment_src_str.as_ptr(), ptr::null());
        gl::CompileShader(fragment_shader);

        shader_program = gl::CreateProgram();
        gl::AttachShader(shader_program, vertex_shader);
        gl::AttachShader(shader_program, fragment_shader);
        gl::LinkProgram(shader_program);

        gl::DeleteShader(vertex_shader);
        gl::DeleteShader(fragment_shader);
    }

    shader_program
}
