use cgmath::prelude::*;
use cgmath::{perspective, vec3, Deg, Matrix, Matrix4};
use std::cmp;
use support::app::*;
use support::ktx::prepare_texture;
use support::load_ktx;
use support::shader::*;

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
    pub fn new() -> DemoApp {
        DemoApp {
            ..Default::default()
        }
    }

    fn load_shaders(&mut self) {
        self.shader_program = ShaderProgram::new();
        self.shader_program
            .vertex_shader("assets/shaders/tunnel/tunnel.vs.glsl")
            .fragment_shader("assets/shaders/tunnel/tunnel.fs.glsl")
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
    }

    fn render(&mut self, current_time: f32) {
        unsafe {
            gl::ClearBufferfv(gl::COLOR, 0, BLACK as *const f32);
        }

        self.shader_program.activate();

        let projection_matrix = perspective(Deg(60.0), self.aspect_ratio, 0.1_f32, 1000_f32);

        unsafe {
            gl::Uniform1f(self.uniform_loc_offset, current_time * 0.003);
        }

        for (index, texture) in self.textures.iter().enumerate() {
            let modelview_matrix =
                Matrix4::from_axis_angle(vec3(0.0, 0.0, 1.0).normalize(), Deg(90.0 * index as f32))
                    * Matrix4::from_translation(vec3(-0.5, 0.0, -10.0))
                    * Matrix4::from_axis_angle(vec3(0.0, 1.0, 0.0).normalize(), Deg(90.0))
                    * Matrix4::from_nonuniform_scale(30.0, 1.0, 1.0);

            let mvp_matrix = projection_matrix * modelview_matrix;

            unsafe {
                gl::UniformMatrix4fv(self.uniform_loc_mvp, 1, gl::FALSE, mvp_matrix.as_ptr());
                gl::BindTexture(gl::TEXTURE_2D, *texture);
                gl::DrawArrays(gl::TRIANGLE_STRIP, 0, 4);
            }
        }
    }
}

fn main() {
    DemoApp::new().run("Tunnel");
}
