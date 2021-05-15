use anyhow::Result;
use gl::types::*;
use glutin::window::Window;
use nalgebra_glm as glm;
use std::{cmp, mem, ptr};
use support::{
    app::run_application, app::App, ktx::prepare_texture, load_ktx, shader::ShaderProgram,
};

const BLACK: &[GLfloat; 4] = &[0.0, 0.0, 0.0, 1.0];
const ONES: &[GLfloat; 1] = &[1.0];

#[rustfmt::skip]
static GRASS_BLADE_VERTICES: &[GLfloat; 12] =
    &[
       -0.3,  0.0,
        0.3,  0.0,
       -0.20, 1.0,
        0.1,  1.3,
       -0.05, 2.3,
        0.0,  3.3
    ];

#[derive(Default)]
struct DemoApp {
    shader_program: ShaderProgram,
    aspect_ratio: f32,
    vao: u32,
    vbo: u32,
    texture_color: u32,
    texture_length: u32,
    texture_orientation: u32,
    texture_bend: u32,
}

impl DemoApp {
    fn load_shaders(&mut self) {
        self.shader_program = ShaderProgram::new();
        self.shader_program
            .vertex_shader("assets/shaders/grass/grass.vs.glsl")
            .fragment_shader("assets/shaders/grass/grass.fs.glsl")
            .link();

        unsafe {
            gl::Enable(gl::DEPTH_TEST);
            gl::DepthFunc(gl::LEQUAL);
        }
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
            gl::GenBuffers(1, &mut self.vbo);
            gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (GRASS_BLADE_VERTICES.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                GRASS_BLADE_VERTICES.as_ptr() as *const gl::types::GLvoid,
                gl::STATIC_DRAW,
            );
            gl::GenVertexArrays(1, &mut self.vao);
            gl::BindVertexArray(self.vao);

            gl::VertexAttribPointer(0, 2, gl::FLOAT, gl::FALSE, 0, ptr::null());
            gl::EnableVertexAttribArray(0);
        }

        unsafe {
            gl::ActiveTexture(gl::TEXTURE1);
        }
        let (_, texture_length) = load_ktx!("../../assets/textures/grass_length.ktx").unwrap();
        self.texture_length = prepare_texture(&texture_length);

        unsafe {
            gl::ActiveTexture(gl::TEXTURE2);
        }
        let (_, texture_orientation) =
            load_ktx!("../../assets/textures/grass_orientation.ktx").unwrap();
        self.texture_orientation = prepare_texture(&texture_orientation);

        unsafe {
            gl::ActiveTexture(gl::TEXTURE3);
        }
        let (_, texture_color) = load_ktx!("../../assets/textures/grass_color.ktx").unwrap();
        self.texture_color = prepare_texture(&texture_color);

        unsafe {
            gl::ActiveTexture(gl::TEXTURE4);
        }
        let (_, texture_bend) = load_ktx!("../../assets/textures/grass_bend.ktx").unwrap();
        self.texture_bend = prepare_texture(&texture_bend);

        Ok(())
    }

    fn render(&mut self, time: f32) -> Result<()> {
        unsafe {
            gl::ClearBufferfv(gl::COLOR, 0, BLACK as *const f32);
            gl::ClearBufferfv(gl::DEPTH, 0, ONES as *const f32);
        }

        let radius = 550.0;
        let factor = time * 0.02;
        let mvp_matrix =
            glm::perspective(self.aspect_ratio, 45_f32.to_radians(), 0.1_f32, 1000_f32)
                * glm::look_at(
                    &glm::vec3(factor.sin() * radius, 25.0, factor.cos() * radius),
                    &glm::vec3(0.0, -50.0, 0.0),
                    &glm::Vec3::y(),
                );

        self.shader_program.activate();

        let mvp_matrix_location = self.shader_program.uniform_location("mvpMatrix");

        unsafe {
            gl::UniformMatrix4fv(mvp_matrix_location, 1, gl::FALSE, mvp_matrix.as_ptr());
            gl::Enable(gl::DEPTH_TEST);
            gl::DepthFunc(gl::LEQUAL);
            gl::BindVertexArray(self.vao);
            gl::DrawArraysInstanced(gl::TRIANGLE_STRIP, 0, 6, 1024 * 1024);
        }

        Ok(())
    }
}

fn main() -> Result<()> {
    let app = DemoApp::default();
    run_application(app, "Grass")
}
