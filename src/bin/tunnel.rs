use anyhow::Result;
use gl::types::*;
use glutin::window::Window;
use nalgebra_glm as glm;
use std::cmp;
use support::{
    app::{run_application, App},
    ktx::prepare_texture,
    load_ktx,
    shader::ShaderProgram,
};

const BLACK: &[GLfloat; 4] = &[0.0, 0.0, 0.0, 0.0];

#[derive(Default)]
struct DemoApp {
    shader_program: ShaderProgram,
    aspect_ratio: f32,
    uniform_loc_mvp: i32,
    uniform_loc_offset: i32,
    textures: Vec<u32>,
    vao: u32,
}

impl DemoApp {
    fn load_shaders(&mut self) {
        self.shader_program = ShaderProgram::new();
        self.shader_program
            .vertex_shader("assets/shaders/tunnel/tunnel.vs.glsl")
            .fragment_shader("assets/shaders/tunnel/tunnel.fs.glsl")
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

        self.uniform_loc_mvp = self.shader_program.uniform_location("mvp");
        self.uniform_loc_offset = self.shader_program.uniform_location("offset");

        let (_, brick) = load_ktx!("../../assets/textures/brick.ktx").unwrap();
        let (_, ceiling) = load_ktx!("../../assets/textures/ceiling.ktx").unwrap();
        let (_, floor) = load_ktx!("../../assets/textures/floor.ktx").unwrap();

        let wall_texture = prepare_texture(&brick);
        self.textures = vec![
            wall_texture,
            prepare_texture(&floor),
            wall_texture,
            prepare_texture(&ceiling),
        ];

        for texture in self.textures.iter() {
            unsafe {
                gl::BindTexture(gl::TEXTURE_2D, *texture);
                gl::TexParameteri(
                    gl::TEXTURE_2D,
                    gl::TEXTURE_MIN_FILTER as u32,
                    gl::LINEAR_MIPMAP_LINEAR as i32,
                );
                gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
            }
        }

        unsafe {
            gl::GenVertexArrays(1, &mut self.vao);
            gl::BindVertexArray(self.vao);
        }

        Ok(())
    }

    fn render(&mut self, time: f32) -> Result<()> {
        unsafe {
            gl::ClearBufferfv(gl::COLOR, 0, BLACK as *const f32);
        }

        self.shader_program.activate();

        let projection_matrix =
            glm::perspective(self.aspect_ratio, 60_f32.to_radians(), 0.1_f32, 1000_f32);

        unsafe {
            gl::Uniform1f(self.uniform_loc_offset, time * 0.003);
        }

        for (index, texture) in self.textures.iter().enumerate() {
            let modelview_matrix =
                glm::rotation((90.0 * index as f32).to_radians(), &glm::Vec3::z())
                    * glm::translation(&glm::vec3(-0.5, 0.0, -10.0))
                    * glm::rotation(90_f32.to_radians(), &glm::Vec3::y())
                    * glm::scaling(&glm::vec3(30.0, 1.0, 1.0));

            let mvp_matrix = projection_matrix * modelview_matrix;

            unsafe {
                gl::UniformMatrix4fv(self.uniform_loc_mvp, 1, gl::FALSE, mvp_matrix.as_ptr());
                gl::BindTexture(gl::TEXTURE_2D, *texture);
                gl::DrawArrays(gl::TRIANGLE_STRIP, 0, 4);
            }
        }

        Ok(())
    }
}

fn main() -> Result<()> {
    let app = DemoApp::default();
    run_application(app, "Tunnel")
}
