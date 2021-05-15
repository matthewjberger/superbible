use anyhow::Result;
use gl::types::*;
use glutin::window::Window;
use nalgebra_glm as glm;
use std::{cmp, mem, ptr};
use support::{
    app::{run_application, App},
    shader::ShaderProgram,
};

const BACKGROUND_COLOR: &[GLfloat; 4] = &[0.0, 0.25, 0.0, 1.0];
const ONES: &[GLfloat; 1] = &[1.0];

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
    fn load_shaders(&mut self) {
        self.shader_program = ShaderProgram::new();
        self.shader_program
            .vertex_shader("assets/shaders/spinny-cube/spinny-cube.vs.glsl")
            .fragment_shader("assets/shaders/spinny-cube/spinny-cube.fs.glsl")
            .link();
    }

    fn update_aspect_ratio(&mut self, width: u32, height: u32) {
        self.aspect_ratio = width as f32 / cmp::max(height, 1) as f32;
    }
}

impl App for DemoApp {
    fn on_resize(&mut self, width: u32, height: u32) -> Result<()> {
        self.update_aspect_ratio(width, height);
        Ok(())
    }

    fn initialize(&mut self, window: &Window) -> Result<()> {
        let inner_size = window.inner_size();
        let (width, height) = (inner_size.width, inner_size.height);
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
        Ok(())
    }

    fn render(&mut self, time: f32) -> Result<()> {
        self.shader_program.activate();

        let modelview_matrix_location = self.shader_program.uniform_location("modelview_matrix");
        let projection_matrix_location = self.shader_program.uniform_location("projection_matrix");
        let projection =
            glm::perspective(self.aspect_ratio, 90_f32.to_radians(), 0.1_f32, 1000_f32);

        let factor: f32 = time * 0.3;

        let modelview = glm::translation(&glm::vec3(0.0, 0.0, -4.0))
            * glm::rotation((time * 45_f32).to_radians(), &glm::Vec3::y())
            * glm::rotation((time * 21_f32).to_radians(), &glm::Vec3::x())
            * glm::translation(&glm::vec3(
                (2.1 * factor).sin() * 0.5,
                (1.7 * factor).cos() * 0.5,
                (1.3 * factor).sin() * (1.5 * factor).cos() * 2.0,
            ));

        unsafe {
            gl::ClearBufferfv(gl::COLOR, 0, BACKGROUND_COLOR as *const f32);
            gl::ClearBufferfv(gl::DEPTH, 0, ONES as *const f32);

            gl::UniformMatrix4fv(
                projection_matrix_location,
                1,
                gl::FALSE,
                projection.as_ptr(),
            );

            gl::UniformMatrix4fv(modelview_matrix_location, 1, gl::FALSE, modelview.as_ptr());

            gl::DrawArrays(gl::TRIANGLES, 0, 36);
        }

        Ok(())
    }
}

fn main() -> Result<()> {
    let app = DemoApp::default();
    run_application(app, "Spinny Cube")
}
