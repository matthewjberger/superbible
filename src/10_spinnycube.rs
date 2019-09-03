use cgmath::prelude::*;
use cgmath::{perspective, vec3, Deg, Matrix, Matrix4};
use std::{cmp, mem, ptr};
use support::app::*;
use support::shader::*;

static VERTEX_SHADER_SOURCE: &str = "
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

static FRAGMENT_SHADER_SOURCE: &str = "
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
static VERTEX_POSITIONS: &[GLfloat; 108] =
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

#[derive(Default)]
struct DemoApp {
    settings: AppSettings,
    shader_program: ShaderProgram,
    vao: u32,
    vbo: u32,
    aspect_ratio: f32,
}

impl DemoApp {
    pub fn new() -> DemoApp {
        let settings = AppSettings {
            title: "Spinny Cube".to_string(),
            ..Default::default()
        };
        let aspect_ratio = settings.initial_width as f32 / settings.initial_height as f32;
        DemoApp {
            settings,
            aspect_ratio,
            ..Default::default()
        }
    }

    fn load_shaders(&mut self) {
        let mut vertex_shader = Shader::new(ShaderType::Vertex);
        vertex_shader.load(VERTEX_SHADER_SOURCE);

        let mut fragment_shader = Shader::new(ShaderType::Fragment);
        fragment_shader.load(FRAGMENT_SHADER_SOURCE);

        self.shader_program = ShaderProgram::new();
        self.shader_program
            .attach(vertex_shader)
            .attach(fragment_shader)
            .link();
    }
}

impl App for DemoApp {
    fn settings(&mut self) -> &AppSettings {
        &self.settings
    }

    fn on_resize(&mut self, width: i32, height: i32) {
        self.aspect_ratio = width as f32 / cmp::max(height, 0) as f32;
    }

    fn initialize(&mut self) {
        self.load_shaders();
        unsafe {
            gl::GenVertexArrays(1, &mut self.vao);
            gl::BindVertexArray(self.vao);

            gl::GenBuffers(1, &mut self.vbo);
            gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);
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
    }

    fn render(&mut self, current_time: f32) {
        self.shader_program.activate();

        let modelview_matrix_location = self.shader_program.uniform_location("modelview_matrix");

        let projection_matrix_location = self.shader_program.uniform_location("projection_matrix");

        let projection = perspective(Deg(50.0), self.aspect_ratio, 0.1_f32, 1000_f32);

        unsafe {
            gl::ClearBufferfv(gl::COLOR, 0, &BACKGROUND_COLOR as *const f32);

            // This is the line from the book, but it crashes the program...
            // gl::ClearBufferfv(gl::DEPTH, 0, 1 as *const f32);

            gl::Clear(gl::DEPTH_BUFFER_BIT);

            gl::UniformMatrix4fv(
                projection_matrix_location,
                1,
                gl::FALSE,
                projection.as_ptr(),
            );

            let factor: f32 = current_time * 0.3;

            let modelview = Matrix4::from_translation(vec3(0.0, 0.0, -4.0))
                * Matrix4::from_axis_angle(
                    vec3(0.0, 1.0, 0.0).normalize(),
                    Deg(current_time * 45_f32),
                )
                * Matrix4::from_axis_angle(
                    vec3(1.0, 0.0, 0.0).normalize(),
                    Deg(current_time * 21_f32),
                )
                * Matrix4::from_translation(vec3(
                    (2.1 * factor).sin() * 0.5,
                    (1.7 * factor).cos() * 0.5,
                    (1.3 * factor).sin() * (1.5 * factor).cos() * 2.0,
                ));

            gl::UniformMatrix4fv(modelview_matrix_location, 1, gl::FALSE, modelview.as_ptr());

            gl::DrawArrays(gl::TRIANGLES, 0, 36);
        }
    }
}

fn main() {
    run(&mut DemoApp::new());
}
