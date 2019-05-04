use gl::types::*;
use glfw::{Action, Context, Key};
use std::ffi::CString;
use std::ptr;

static VERTEX_SHADER_SOURCE: &'static str = "
#version 450 core

void main(void)
{
    const vec4 vertices[] = vec4[](vec4( 0.25, -0.25, 0.5, 1.0),
                                   vec4(-0.25, -0.25, 0.5, 1.0),
                                   vec4( 0.25,  0.25, 0.5, 1.0));

    gl_Position = vertices[gl_VertexID];
}
";

static TESSELLATION_CONTROL_SHADER_SOURCE: &'static str = "
#version 450 core

layout (vertices = 3) out;

void main(void)
{
    if (gl_InvocationID == 0)
    {
        gl_TessLevelInner[0] = 5.0;
        gl_TessLevelOuter[0] = 5.0;
        gl_TessLevelOuter[1] = 5.0;
        gl_TessLevelOuter[2] = 5.0;
    }
    gl_out[gl_InvocationID].gl_Position = gl_in[gl_InvocationID].gl_Position;
}
";

static TESSELLATION_EVALUATION_SHADER_SOURCE: &'static str = "
#version 450 core

layout (triangles, equal_spacing, cw) in;

void main(void)
{
    gl_Position = (gl_TessCoord.x * gl_in[0].gl_Position) +
                  (gl_TessCoord.y * gl_in[1].gl_Position) +
                  (gl_TessCoord.z * gl_in[2].gl_Position);
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
        .create_window(600, 600, "Tessellated Triangle", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window.");

    window.make_current();
    window.set_key_polling(true);
    window.set_framebuffer_size_polling(true);

    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

    let shader_program = compile_shaders();

    let mut vao = 0;

    unsafe {
        gl::CreateVertexArrays(1, &mut vao);
        gl::BindVertexArray(vao);
        gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);
    }

    while !window.should_close() {
        context.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            if let glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) = event {
                window.set_should_close(true)
            }
        }
        render(shader_program);
        window.swap_buffers();
    }
}

fn render(shader_program: u32) {
    let background_color: [GLfloat; 4] = [0.0, 0.0, 0.0, 1.0];
    unsafe {
        gl::ClearBufferfv(gl::COLOR, 0, &background_color as *const f32);
        gl::UseProgram(shader_program);
        gl::DrawArrays(gl::PATCHES, 0, 3);
    }
}

fn compile_shaders() -> GLuint {
    let vertex_src_str = CString::new(VERTEX_SHADER_SOURCE.as_bytes()).unwrap();
    let fragment_src_str = CString::new(FRAGMENT_SHADER_SOURCE.as_bytes()).unwrap();
    let tessellation_control_src_str =
        CString::new(TESSELLATION_CONTROL_SHADER_SOURCE.as_bytes()).unwrap();
    let tessellation_evaluation_src_str =
        CString::new(TESSELLATION_EVALUATION_SHADER_SOURCE.as_bytes()).unwrap();

    let vertex_shader;
    let fragment_shader;
    let tessellation_control_shader;
    let tessellation_evaluation_shader;
    let shader_program;

    unsafe {
        vertex_shader = gl::CreateShader(gl::VERTEX_SHADER);
        gl::ShaderSource(vertex_shader, 1, &vertex_src_str.as_ptr(), ptr::null());
        gl::CompileShader(vertex_shader);

        tessellation_control_shader = gl::CreateShader(gl::TESS_CONTROL_SHADER);
        gl::ShaderSource(
            tessellation_control_shader,
            1,
            &tessellation_control_src_str.as_ptr(),
            ptr::null(),
        );
        gl::CompileShader(tessellation_control_shader);

        tessellation_evaluation_shader = gl::CreateShader(gl::TESS_EVALUATION_SHADER);
        gl::ShaderSource(
            tessellation_evaluation_shader,
            1,
            &tessellation_evaluation_src_str.as_ptr(),
            ptr::null(),
        );
        gl::CompileShader(tessellation_evaluation_shader);

        fragment_shader = gl::CreateShader(gl::FRAGMENT_SHADER);
        gl::ShaderSource(fragment_shader, 1, &fragment_src_str.as_ptr(), ptr::null());
        gl::CompileShader(fragment_shader);

        shader_program = gl::CreateProgram();
        gl::AttachShader(shader_program, vertex_shader);
        gl::AttachShader(shader_program, tessellation_control_shader);
        gl::AttachShader(shader_program, tessellation_evaluation_shader);
        gl::AttachShader(shader_program, fragment_shader);
        gl::LinkProgram(shader_program);

        gl::DeleteShader(vertex_shader);
        gl::DeleteShader(tessellation_control_shader);
        gl::DeleteShader(tessellation_evaluation_shader);
        gl::DeleteShader(fragment_shader);
    }

    shader_program
}
