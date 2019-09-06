use cgmath::prelude::*;
use cgmath::{perspective, vec3, Deg, Matrix, Matrix4};
use std::{cmp, mem, ptr};
use support::app::*;
use support::shader::*;

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
    shader_program: ShaderProgram,
    vao: u32,
    vbo: u32,
    aspect_ratio: f32,
}

impl DemoApp {
    pub fn new() -> DemoApp {
        DemoApp {
            ..Default::default()
        }
    }

    fn load_shaders(&mut self) {
        let mut vertex_shader = Shader::new(ShaderType::Vertex);
        vertex_shader.load_file("../assets/shaders/spinny-cube/spinny-cube.vs.glsl");

        let mut fragment_shader = Shader::new(ShaderType::Fragment);
        fragment_shader.load_file("../assets/shaders/spinny-cube/spinny-cube.fs.glsl");

        self.shader_program = ShaderProgram::new();
        self.shader_program
            .attach(vertex_shader)
            .attach(fragment_shader)
            .link();
    }

    fn update_aspect_ratio(&mut self, width: i32, height: i32) {
        self.aspect_ratio = width as f32 / cmp::max(height, 0) as f32;
    }
}

impl App for DemoApp {
    fn on_resize(&mut self, width: i32, height: i32) {
        self.update_aspect_ratio(width, height);
    }

    fn initialize(&mut self, window: &mut glfw::Window) {
        let (width, height) = window.get_size();
        self.update_aspect_ratio(width, height);
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
    DemoApp::new().run("Spinny Cube");
}
