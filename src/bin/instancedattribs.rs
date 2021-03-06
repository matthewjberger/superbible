use std::{cmp, mem, ptr};
use support::app::*;
use support::shader::*;

const BLACK: &[GLfloat; 4] = &[0.0, 0.0, 0.0, 1.0];

#[rustfmt::skip]
static SQUARE_VERTICES: &[GLfloat; 16] =
    &[
       -1.0, -1.0, 0.0, 1.0,
        1.0, -1.0, 0.0, 1.0,
        1.0,  1.0, 0.0, 1.0,
       -1.0,  1.0, 0.0, 1.0
    ];

#[rustfmt::skip]
static INSTANCE_COLORS: &[GLfloat; 16] =
    &[
        1.0, 0.0, 0.0, 1.0,
        0.0, 1.0, 0.0, 1.0,
        0.0, 0.0, 1.0, 1.0,
        1.0, 1.0, 0.0, 1.0
    ];

#[rustfmt::skip]
static INSTANCE_POSITIONS: &[GLfloat; 16] =
    &[
       -2.0, -2.0, 0.0, 0.0,
        2.0, -2.0, 0.0, 0.0,
        2.0,  2.0, 0.0, 0.0,
       -2.0,  2.0, 0.0, 0.0
    ];

#[derive(Default)]
struct DemoApp {
    shader_program: ShaderProgram,
    aspect_ratio: f32,
    vao: u32,
}

impl DemoApp {
    pub fn new() -> DemoApp {
        DemoApp {
            ..Default::default()
        }
    }

    fn load_shaders(&mut self) {
        self.shader_program = ShaderProgram::new();
        self.shader_program
            .vertex_shader("assets/shaders/instanced-attribs/instanced-attribs.vs.glsl")
            .fragment_shader("assets/shaders/instanced-attribs/instanced-attribs.fs.glsl")
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

        let mut vbo = 0;
        let square_vertices_size =
            (SQUARE_VERTICES.len() * mem::size_of::<GLfloat>()) as GLsizeiptr;
        let instance_colors_size =
            (INSTANCE_COLORS.len() * mem::size_of::<GLfloat>()) as GLsizeiptr;
        let instance_positions_size =
            (INSTANCE_POSITIONS.len() * mem::size_of::<GLfloat>()) as GLsizeiptr;

        unsafe {
            gl::GenVertexArrays(1, &mut self.vao);
            gl::GenBuffers(1, &mut vbo);
            gl::BindVertexArray(self.vao);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                square_vertices_size + instance_colors_size + instance_positions_size,
                ptr::null(),
                gl::STATIC_DRAW,
            );

            gl::BufferSubData(
                gl::ARRAY_BUFFER,
                0,
                square_vertices_size,
                SQUARE_VERTICES.as_ptr() as *const GLvoid,
            );

            gl::BufferSubData(
                gl::ARRAY_BUFFER,
                square_vertices_size,
                instance_colors_size,
                INSTANCE_COLORS.as_ptr() as *const GLvoid,
            );

            gl::BufferSubData(
                gl::ARRAY_BUFFER,
                square_vertices_size + instance_colors_size,
                instance_positions_size,
                INSTANCE_POSITIONS.as_ptr() as *const GLvoid,
            );

            gl::VertexAttribPointer(0, 4, gl::FLOAT, gl::FALSE, 0, ptr::null());

            gl::VertexAttribPointer(
                1,
                4,
                gl::FLOAT,
                gl::FALSE,
                0,
                square_vertices_size as *const GLvoid,
            );

            gl::VertexAttribPointer(
                2,
                4,
                gl::FLOAT,
                gl::FALSE,
                0,
                (square_vertices_size + instance_colors_size) as *const GLvoid,
            );

            gl::EnableVertexAttribArray(0);
            gl::EnableVertexAttribArray(1);
            gl::EnableVertexAttribArray(2);

            gl::VertexAttribDivisor(1, 1);
            gl::VertexAttribDivisor(2, 1);
        }
    }

    fn render(&mut self, _: f32) {
        unsafe {
            gl::ClearBufferfv(gl::COLOR, 0, BLACK as *const f32);
        }
        self.shader_program.activate();
        unsafe {
            gl::BindVertexArray(self.vao);
            gl::DrawArraysInstanced(gl::TRIANGLE_FAN, 0, 4, 4);
        }
    }
}

fn main() {
    DemoApp::new().run("Instanced Attributes");
}
