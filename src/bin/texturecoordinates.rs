use anyhow::Result;
use gl::types::*;
use glutin::{event::ElementState, event::VirtualKeyCode, window::Window};
use nalgebra_glm as glm;
use std::cmp;
use support::{
    app::{run_application, App},
    ktx::prepare_texture,
    load_ktx, load_object,
    object::{render_all, Object},
    shader::ShaderProgram,
};

const GRAY: &[GLfloat; 4] = &[0.2, 0.2, 0.2, 1.0];
const ONES: &[GLfloat; 1] = &[1.0];

fn create_procedural_texture() -> u32 {
    let mut texture = 0;

    let dimension: i32 = 16;
    let mut data = Vec::new();
    for column in 0..dimension {
        for row in 0..dimension {
            let value = match (row % 2, column % 2) {
                (0, 0) | (1, 1) => 0xFFFF_FFFF,
                (_, _) => 0_u32,
            };
            data.push(value);
        }
    }

    unsafe {
        gl::GenTextures(1, &mut texture);
        gl::BindTexture(gl::TEXTURE_2D, texture);
        gl::TexStorage2D(gl::TEXTURE_2D, 1, gl::RGB8, dimension, dimension);
        gl::TexSubImage2D(
            gl::TEXTURE_2D,
            0,
            0,
            0,
            dimension,
            dimension,
            gl::RGBA,
            gl::UNSIGNED_BYTE,
            data.as_ptr() as *const GLvoid,
        );
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
    }
    texture
}

#[derive(Default)]
struct DemoApp {
    shader_program: ShaderProgram,
    aspect_ratio: f32,
    object: Object,
    texture_1: u32,
    texture_2: u32,
    current_texture: u32,
}

impl DemoApp {
    fn bind_texture(&mut self, texture: u32) {
        self.current_texture = texture;
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.current_texture);
        }
    }

    fn toggle_texture(&mut self) {
        if self.current_texture == self.texture_1 {
            self.bind_texture(self.texture_2)
        } else {
            self.bind_texture(self.texture_1);
        };
    }

    fn load_shaders(&mut self) {
        self.shader_program = ShaderProgram::new();
        self.shader_program
            .vertex_shader("assets/shaders/texture-coordinates/texture-coordinates.vs.glsl")
            .fragment_shader("assets/shaders/texture-coordinates/texture-coordinates.fs.glsl")
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

    fn on_key(&mut self, keycode: &VirtualKeyCode, keystate: &ElementState) -> Result<()> {
        match (keycode, keystate) {
            (VirtualKeyCode::T, ElementState::Pressed) => {
                self.toggle_texture();
            }
            (VirtualKeyCode::R, ElementState::Pressed) => {
                self.load_shaders();
            }
            _ => (),
        }

        Ok(())
    }

    fn initialize(&mut self, window: &Window) -> Result<()> {
        let inner_size = window.inner_size();
        let (width, height) = (inner_size.width, inner_size.height);
        self.update_aspect_ratio(width, height);
        self.load_shaders();

        let (_, data) = load_ktx!("../../assets/textures/pattern1.ktx").unwrap();
        self.texture_1 = prepare_texture(&data);
        self.texture_2 = create_procedural_texture();
        self.bind_texture(self.texture_1);

        let (_, obj) = load_object!("../../assets/objects/torus_nrms_tc.sbm").unwrap();
        self.object = obj;

        unsafe {
            gl::Enable(gl::DEPTH_TEST);
            gl::DepthFunc(gl::LEQUAL);
        }

        Ok(())
    }

    fn render(&mut self, time: f32) -> Result<()> {
        unsafe {
            gl::ClearBufferfv(gl::COLOR, 0, GRAY as *const f32);
            gl::ClearBufferfv(gl::DEPTH, 0, ONES as *const f32);
        }

        self.shader_program.activate();

        let modelview_matrix_location = self.shader_program.uniform_location("modelview_matrix");
        let projection_matrix_location = self.shader_program.uniform_location("projection_matrix");
        let projection =
            glm::perspective(self.aspect_ratio, 60_f32.to_radians(), 0.1_f32, 1000_f32);

        unsafe {
            gl::UniformMatrix4fv(
                projection_matrix_location,
                1,
                gl::FALSE,
                projection.as_ptr(),
            );

            let modelview = glm::translation(&glm::vec3(0.0, 0.0, -3.0))
                * glm::rotation((time * 19.3).to_radians(), &glm::Vec3::y())
                * glm::rotation((time * 21.1).to_radians(), &glm::Vec3::z());

            gl::UniformMatrix4fv(modelview_matrix_location, 1, gl::FALSE, modelview.as_ptr());
        }

        render_all(&self.object);

        Ok(())
    }
}

fn main() -> Result<()> {
    let app = DemoApp::default();
    run_application(app, "Texture Coordinates")
}
