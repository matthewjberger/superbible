// use cgmath::{perspective, Deg};
use gl::types::*;
use glfw::{Action, Context, Key};
use std::{ffi::CString, ptr};

const SCREEN_WIDTH: u32 = 600;
const SCREEN_HEIGHT: u32 = 600;

// TODO: Move shaders out to file
static VERTEX_SHADER_SOURCE: &'static str = "
#version 420 core

uniform mat4 mv_matrix;
uniform mat4 proj_matrix;

layout (location = 0) in vec4 position;
layout (location = 4) in vec2 tc;

out VS_OUT
{
    vec2 tc;
} vs_out;

void main(void)
{
    vec4 pos_vs = mv_matrix * position;

    vs_out.tc = tc;

    gl_Position = proj_matrix * pos_vs;
}
";

static FRAGMENT_SHADER_SOURCE: &'static str = "
#version 420 core

layout (binding = 0) uniform sampler2D tex_object;

in VS_OUT
{
    vec2 tc;
} fs_in;

out vec4 color;

void main(void)
{
    color = texture(tex_object, fs_in.tc * vec2(3.0, 1.0));
}

";

static GRAY: &'static [GLfloat; 4] = &[0.2, 0.2, 0.2, 1.0];
static ONES: &'static [GLfloat; 1] = &[1.0];

fn main() {
    let mut context = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    let (mut window, events) = context
        .create_window(
            SCREEN_WIDTH,
            SCREEN_HEIGHT,
            "Texture Coordinates",
            glfw::WindowMode::Windowed,
        )
        .expect("Failed to create GLFW window.");

    window.make_current();
    window.set_key_polling(true);
    window.set_framebuffer_size_polling(true);

    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

    let mut vao = 0;
    let mut texture = 0;

    // TODO: Load KTX pattern as well
    // TODO: Write object loader and load torus_nrms_tc.sbm

    unsafe {
        gl::GenTextures(1, &mut texture);
        gl::BindTexture(gl::TEXTURE_2D, texture);
        gl::TexStorage2D(gl::TEXTURE_2D, 1, gl::RGB8, 16, 16);

        // TODO: Generate checker pattern
        // TODO: Pass pattern data to texture
        // gl::TexSubImage2D(
        //     gl::TEXTURE_2D,
        //     0,
        //     0,
        //     0,
        //     16,
        //     16,
        //     gl::RGBA,
        //     gl::UNSIGNED_BYTE,
        //     data.as_ptr() as *const GLvoid,
        // );
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);

        gl::Enable(gl::DEPTH_TEST);
        gl::DepthFunc(gl::LEQUAL);

        gl::GenVertexArrays(1, &mut vao);
        gl::BindVertexArray(vao);
    }

    let shader_program = compile_shaders();
    // let mut aspect_ratio: f32 = SCREEN_WIDTH as f32 / SCREEN_HEIGHT as f32;

    // TODO: Make main app that can be run and overriden
    while !window.should_close() {
        context.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            match event {
                glfw::WindowEvent::FramebufferSize(width, height) => unsafe {
                    // aspect_ratio = width as f32 / height as f32;
                    gl::Viewport(0, 0, width, height)
                },
                glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                    window.set_should_close(true)
                }
                _ => {}
            }

            // let projection = perspective(Deg(50.0), aspect_ratio, 0.1 as f32, 1000 as f32);

            // Get uniform locations
            // let mv_matrix = gl::GetUniformLocation(shader_program, "mv_matrix");
            // let proj_matrix = gl::GetUniformLocation(shader_program, "proj_matrix");

            render(context.get_time(), shader_program, texture);

            window.swap_buffers();
        }
    }
}

fn render(_current_time: f64, shader_program: u32, texture: u32) {
    unsafe {
        gl::ClearBufferfv(gl::COLOR, 0, GRAY as *const f32);
        gl::ClearBufferfv(gl::DEPTH, 0, ONES as *const f32);

        // TODO: Swap between this and pattern
        gl::BindTexture(gl::TEXTURE_2D, texture);

        gl::UseProgram(shader_program);

        // TODO: Setup MVP matrix here
        // C++ -->
        // vmath::mat4 proj_matrix = vmath::perspective(60.0f, (float)info.windowWidth / (float)info.windowHeight, 0.1f, 1000.0f);
        // vmath::mat4 mv_matrix = vmath::translate(0.0f, 0.0f, -3.0f) *
        //     vmath::rotate((float)currentTime * 19.3f, 0.0f, 1.0f, 0.0f) *
        //     vmath::rotate((float)currentTime * 21.1f, 0.0f, 0.0f, 1.0f);
        // then set uniforms
        //
        // Rust -->
        // gl::UniformMatrix4fv(uniforms.mv_matrix, 1 gl::FALSE, mv_matrix);
        // gl::UniformMatrix4fv(uniforms.proj_matrix, 1 gl::FALSE, proj_matrix);

        // TODO: Render object
    }
}

// TODO: Write wrapper for compiling shaders
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
