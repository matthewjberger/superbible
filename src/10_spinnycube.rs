use cgmath::{perspective, Deg, Matrix, Vector3};
use gl::types::*;
use glfw::{Action, Context, Key};
use std::time::SystemTime;
use std::{ffi::CString, mem, os::raw::c_void, ptr};

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

const SCREEN_WIDTH: u32 = 600;
const SCREEN_HEIGHT: u32 = 600;

fn main() {
    let mut context = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    let (mut window, events) = context
        .create_window(
            SCREEN_WIDTH,
            SCREEN_HEIGHT,
            "Interpolation",
            glfw::WindowMode::Windowed,
        )
        .expect("Failed to create GLFW window.");

    window.make_current();
    window.set_key_polling(true);
    window.set_framebuffer_size_polling(true);

    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

    let start_time = SystemTime::now();

    let shader_program = compile_shaders();

    let (mut vao, mut vbo) = (0, 0);

    unsafe {
        gl::CreateVertexArrays(1, &mut vao);
        gl::BindVertexArray(vao);

        gl::GenBuffers(1, &mut vbo);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (VERTEX_POSITIONS.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
            &VERTEX_POSITIONS[0] as *const f32 as *const c_void,
            gl::STATIC_DRAW,
        );
        gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 0, ptr::null());
        gl::EnableVertexAttribArray(0);

        gl::Enable(gl::CULL_FACE);
        gl::FrontFace(gl::CW);
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

    unsafe {
        gl::Viewport(0, 0, SCREEN_WIDTH as i32, SCREEN_HEIGHT as i32);

        gl::ClearBufferfv(gl::COLOR, 0, &BACKGROUND_COLOR as *const f32);
        gl::ClearBufferfv(gl::DEPTH, 0, 1 as *const f32);

        gl::UseProgram(shader_program);

        let modelview_matrix_str: CString = CString::new("modelview_matrix").unwrap();
        let projection_matrix_str: CString = CString::new("projection_matrix").unwrap();

        let modelview_matrix_location =
            gl::GetUniformLocation(shader_program, modelview_matrix_str.as_ptr());

        let projection_matrix_location =
            gl::GetUniformLocation(shader_program, projection_matrix_str.as_ptr());

        let model = perspective(
            Deg(45.0),
            SCREEN_WIDTH as f32 / SCREEN_HEIGHT as f32,
            0.1 as f32,
            100 as f32,
        );

        gl::UniformMatrix4fv(projection_matrix_location, 1, gl::FALSE, model.as_ptr());

        // TODO: Complete this. Build the model and view matrices and multiply them to get the modelview matrix.
        //       Then assign it to the uniform
        // let model =
        //     Matrix4::from_translation(Vector3::new(0.0, 0.0, -6.0)) *
        //     Matrix4::from_

        gl::DrawArrays(gl::TRIANGLES, 0, 36);
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
