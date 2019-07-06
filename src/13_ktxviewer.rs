mod support;
use support::*;

use gl::types::*;
use glfw::{Action, Context, Key};
use std::ffi::CString;
use std::ptr;

static VERTEX_SHADER_SOURCE: &'static str = "
#version 450 core
void main(void) {
    const vec4 vertices[] = vec4[](vec4(-1.0, -1.0, 0.5, 1.0),
                                   vec4( 1.0, -1.0, 0.5, 1.0),
                                   vec4(-1.0,  1.0, 0.5, 1.0),
                                   vec4( 1.0,  1.0, 0.5, 1.0));
    gl_Position = vertices[gl_VertexID];
}
";

static FRAGMENT_SHADER_SOURCE: &'static str = "
#version 450 core

uniform sampler2D s;

uniform float exposure;

out vec4 color;

void main(void)
{
    color = texture(s, gl_FragCoord.xy / textureSize(s, 0)) * exposure;
}
";

static GREEN: &'static [GLfloat; 4] = &[0.0, 0.25, 0.0, 1.0];

fn main() {
    let mut context = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    let (mut window, events) = context
        .create_window(800, 600, "KTX Viewer", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window.");

    window.make_current();
    window.set_key_polling(true);
    window.set_framebuffer_size_polling(true);

    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

    let mut vao = 0;
    let mut texture = 0;

    // Load a texture
    let (_, data) = load_ktx!("../assets/textures/tree.ktx").unwrap();
    let ktx = data.header;
    let image = data.pixels;

    unsafe {
        gl::GenTextures(1, &mut texture);
        gl::BindTexture(gl::TEXTURE_2D, texture);

        gl::TexStorage2D(
            gl::TEXTURE_2D,
            ktx.mip_levels as i32,
            ktx.gl_internal_format,
            ktx.pixel_width as i32,
            ktx.pixel_height as i32,
        );

        gl::PixelStorei(gl::UNPACK_ALIGNMENT, 1);

        gl::TexSubImage2D(
            gl::TEXTURE_2D,
            0,
            0,
            0,
            ktx.pixel_width as i32,
            ktx.pixel_height as i32,
            ktx.gl_format,
            ktx.gl_type,
            image.as_ptr() as *const GLvoid,
        );

        gl::Viewport(0, 0, ktx.pixel_width as i32, ktx.pixel_height as i32);

        gl::GenVertexArrays(1, &mut vao);
        gl::BindVertexArray(vao);
    }

    let shader_program = compile_shaders();

    while !window.should_close() {
        context.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            if let glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) = event {
                window.set_should_close(true)
            }
        }
        render(context.get_time(), shader_program);
        window.swap_buffers();
    }
}

fn render(current_time: f64, shader_program: u32) {
    unsafe {
        gl::ClearBufferfv(gl::COLOR, 0, GREEN as *const f32);
        gl::UseProgram(shader_program);
        gl::Uniform1f(0, current_time.sin() as f32 * 16.0 + 16.0);
        gl::DrawArrays(gl::TRIANGLE_STRIP, 0, 4);
    }
}

fn compile_shaders() -> GLuint {
    let vertex_src_str = CString::new(VERTEX_SHADER_SOURCE.as_bytes()).unwrap();
    let fragment_src_str = CString::new(FRAGMENT_SHADER_SOURCE.as_bytes()).unwrap();

    let vertex_shader;
    let fragment_shader;
    let shader_program;

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
