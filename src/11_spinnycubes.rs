use cgmath::prelude::*;
use cgmath::{perspective, vec3, Deg, Matrix, Matrix4};
use gl::types::*;
use glfw::{Action, Context, Key};
use std::{ffi::CString, mem, ptr};

const SCREEN_WIDTH: u32 = 600;
const SCREEN_HEIGHT: u32 = 600;

static VERTEX_SHADER_SOURCE: &'static str = "
#version 450 core

in vec4 position;

out VS_OUT
{
    vec4 color;
} vs_out;

uniform mat4 modelview_matrix;
uniform mat4 projection_matrix;

void main(void)
{
    gl_Position = projection_matrix * modelview_matrix * position;
    vs_out.color = position * 2.0 + vec4(0.5, 0.5, 0.5, 0.0);
}
";

static FRAGMENT_SHADER_SOURCE: &'static str = "
#version 450 core

out vec4 color;

in VS_OUT
{
    vec4 color;
} fs_in;

void main(void)
{
    color = fs_in.color;
}
";

const BACKGROUND_COLOR: [GLfloat; 4] = [0.0, 0.25, 0.0, 1.0];

#[rustfmt::skip]
static VERTEX_POSITIONS: &'static [GLfloat; 108] =
        &[
            -0.25,  0.25, -0.25,
            -0.25, -0.25, -0.25,
             0.25, -0.25, -0.25,

             0.25, -0.25, -0.25,
             0.25,  0.25, -0.25,
            -0.25,  0.25, -0.25,

             0.25, -0.25, -0.25,
             0.25, -0.25,  0.25,
             0.25,  0.25, -0.25,

             0.25, -0.25,  0.25,
             0.25,  0.25,  0.25,
             0.25,  0.25, -0.25,

             0.25, -0.25,  0.25,
            -0.25, -0.25,  0.25,
             0.25,  0.25,  0.25,

            -0.25, -0.25,  0.25,
            -0.25,  0.25,  0.25,
             0.25,  0.25,  0.25,

            -0.25, -0.25,  0.25,
            -0.25, -0.25, -0.25,
            -0.25,  0.25,  0.25,

            -0.25, -0.25, -0.25,
            -0.25,  0.25, -0.25,
            -0.25,  0.25,  0.25,

            -0.25, -0.25,  0.25,
             0.25, -0.25,  0.25,
             0.25, -0.25, -0.25,

             0.25, -0.25, -0.25,
            -0.25, -0.25, -0.25,
            -0.25, -0.25,  0.25,

            -0.25,  0.25, -0.25,
             0.25,  0.25, -0.25,
             0.25,  0.25,  0.25,

             0.25,  0.25,  0.25,
            -0.25,  0.25,  0.25,
            -0.25,  0.25, -0.25
        ];

fn main() {
    let mut context = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    let (mut window, events) = context
        .create_window(
            SCREEN_WIDTH,
            SCREEN_HEIGHT,
            "Spinny Cubes",
            glfw::WindowMode::Windowed,
        )
        .expect("Failed to create GLFW window.");

    window.make_current();
    window.set_key_polling(true);
    window.set_framebuffer_size_polling(true);

    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

    let shader_program = compile_shaders();

    let (mut vao, mut vbo) = (0, 0);

    unsafe {
        gl::GenVertexArrays(1, &mut vao);
        gl::BindVertexArray(vao);

        gl::GenBuffers(1, &mut vbo);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (VERTEX_POSITIONS.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
            VERTEX_POSITIONS.as_ptr() as *const gl::types::GLvoid,
            gl::STATIC_DRAW,
        );
        gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 0, ptr::null());
        gl::EnableVertexAttribArray(0);

        gl::Enable(gl::CULL_FACE);
        gl::FrontFace(gl::CW);

        gl::Enable(gl::DEPTH_TEST);
        gl::DepthFunc(gl::LEQUAL);
    }

    let mut aspect_ratio: f32 = SCREEN_WIDTH as f32 / SCREEN_HEIGHT as f32;

    while !window.should_close() {
        context.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            match event {
                glfw::WindowEvent::FramebufferSize(width, height) => unsafe {
                    aspect_ratio = width as f32 / height as f32;
                    gl::Viewport(0, 0, width, height)
                },
                glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                    window.set_should_close(true)
                }
                _ => {}
            }
        }

        let projection = perspective(Deg(50.0), aspect_ratio, 0.1 as f32, 1000 as f32);

        render(projection, context.get_time() as f32, shader_program);
        window.swap_buffers();
    }
}

fn render(projection: Matrix4<f32>, current_time: f32, shader_program: u32) {
    unsafe {
        gl::ClearBufferfv(gl::COLOR, 0, &BACKGROUND_COLOR as *const f32);

        // This is the line from the book, but it seems to crash the program...
        // gl::ClearBufferfv(gl::DEPTH, 0, 1 as *const f32);

        gl::Clear(gl::DEPTH_BUFFER_BIT);

        gl::UseProgram(shader_program);

        let modelview_matrix_str: CString = CString::new("modelview_matrix").unwrap();
        let projection_matrix_str: CString = CString::new("projection_matrix").unwrap();

        let modelview_matrix_location =
            gl::GetUniformLocation(shader_program, modelview_matrix_str.as_ptr());

        let projection_matrix_location =
            gl::GetUniformLocation(shader_program, projection_matrix_str.as_ptr());

        gl::UniformMatrix4fv(
            projection_matrix_location,
            1,
            gl::FALSE,
            projection.as_ptr(),
        );

        for cube_id in 0..24 {
            let factor: f32 = cube_id as f32 + current_time * 0.3;
            let modelview = Matrix4::from_translation(vec3(0.0, 0.0, -4.0))
                * Matrix4::from_axis_angle(
                    vec3(0.0, 1.0, 0.0).normalize(),
                    Deg(current_time * 45 as f32),
                )
                * Matrix4::from_axis_angle(
                    vec3(1.0, 0.0, 0.0).normalize(),
                    Deg(current_time * 21 as f32),
                )
                * Matrix4::from_translation(vec3(
                    (2.1 * factor).sin() * 2.0,
                    (1.7 * factor).cos() * 2.0,
                    (1.3 * factor).sin() * (1.5 * factor).cos() * 2.0,
                ));

            gl::UniformMatrix4fv(modelview_matrix_location, 1, gl::FALSE, modelview.as_ptr());
            gl::DrawArrays(gl::TRIANGLES, 0, 36);
        }
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
