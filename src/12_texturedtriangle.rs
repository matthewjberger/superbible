use gl::types::*;
use glfw::{Action, Context, Key};
use std::ffi::CString;
use std::ptr;

static VERTEX_SHADER_SOURCE: &'static str = "
#version 450 core
void main(void) {
    const vec4 vertices[] = vec4[](vec4( 0.75, -0.75, 0.5, 1.0),
                                   vec4(-0.75, -0.75, 0.5, 1.0),
                                   vec4( 0.75,  0.75, 0.5, 1.0));
    gl_Position = vertices[gl_VertexID];
}
";

static FRAGMENT_SHADER_SOURCE: &'static str = "
#version 450 core

uniform sampler2D s;

out vec4 color;

void main(void)
{
    color = texture(s, gl_FragCoord.xy / textureSize(s, 0));
}
";

static GREEN: &'static [GLfloat; 4] = &[0.0, 0.25, 0.0, 1.0];

fn main() {
    let mut context = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    let (mut window, events) = context
        .create_window(800, 600, "Textured Triangle", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window.");

    window.make_current();
    window.set_key_polling(true);
    window.set_framebuffer_size_polling(true);

    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

    let (width, height) = (256 as usize, 256 as usize);
    let mut vao = 0;
    let mut texture = 0;

    // Define data to upload into the texture
    let mask = 0xFF;
    let max = 255.0;
    let mut data = vec![1.0; width * height * 4];
    for y in 0..height {
        for x in 0..width {
            let index = y * width + x;
            data[index * 4] = ((x & y) & mask) as f32 / max;
            data[index * 4 + 1] = ((x | y) & mask) as f32 / max;
            data[index * 4 + 2] = ((x ^ y) & mask) as f32 / max;
            data[index * 4 + 3] = 1.0;
        }
    }

    unsafe {
        // Generate a name for the texture
        gl::GenTextures(1, &mut texture);

        // Bind it to the context using the GL_TEXTURE_2D binding point
        gl::BindTexture(gl::TEXTURE_2D, texture);

        // Specify the amount of storage we want to use for this texture
        // * 2D texture
        // * 8 mipmap levels
        // * 32-bit floating point RGBA data
        // * 256 x 256 texels
        gl::TexStorage2D(gl::TEXTURE_2D, 8, gl::RGBA32F, width as i32, height as i32);

        // Specify a two dimensional texture subimage
        // * 2D texture
        // * Level 0
        // * Offset 0, 0
        // * 256 x 256 texels, replace entire image
        // * Four channel data
        // * Floating point data
        // * Pointer to data
        gl::TexSubImage2D(
            gl::TEXTURE_2D,
            0,
            0,
            0,
            256,
            256,
            gl::RGBA,
            gl::FLOAT,
            data.as_ptr() as *const GLvoid,
        );

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
        render(shader_program);
        window.swap_buffers();
    }
}

fn render(shader_program: u32) {
    unsafe {
        gl::ClearBufferfv(gl::COLOR, 0, GREEN as *const f32);
        gl::UseProgram(shader_program);
        gl::DrawArrays(gl::TRIANGLES, 0, 3);
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
