use cgmath::prelude::*;
use cgmath::{perspective, vec3, Deg, Matrix, Matrix4};
use std::cmp;
use support::app::*;
use support::ktx::prepare_texture;
use support::load_ktx;
use support::load_object;
use support::object::{render_all, render_object, Object};
use support::shader::*;

const BACKGROUND_COLOR: [GLfloat; 4] = [0.0, 0.25, 0.0, 1.0];
const GRAY: &[GLfloat; 4] = &[0.2, 0.2, 0.2, 1.0];
const ONES: &[GLfloat; 1] = &[1.0];

fn create_procedural_texture() -> u32 {
    let (width, height) = (256 as usize, 256 as usize);
    let mut texture = 0;

    // Define data to upload into the texture
    let mask = 0xFF;
    let max = 255.0;
    let mut data = vec![1.0; width * height * 4];
    for y in 0..height {
        for x in 0..width {
            let index = y * width + x;
            data[index * 4] = ((x & y) & mask) as f32 / max;
            data[index * 4 + 1] = ((x | y) & mask) as f32 / max;
            data[index * 4 + 2] = ((x ^ y) & mask) as f32 / max;
            data[index * 4 + 3] = 1.0;
        }
    }

    unsafe {
        // Generate a name for the texture
        gl::GenTextures(1, &mut texture);

        // Bind it to the context using the GL_TEXTURE_2D binding point
        gl::BindTexture(gl::TEXTURE_2D, texture);

        // Specify the amount of storage we want to use for this texture
        // * 2D texture
        // * 8 mipmap levels
        // * 32-bit floating point RGBA data
        // * 256 x 256 texels
        gl::TexStorage2D(gl::TEXTURE_2D, 8, gl::RGBA32F, width as i32, height as i32);

        // Specify a two dimensional texture subimage
        // * 2D texture
        // * Level 0
        // * Offset 0, 0
        // * 256 x 256 texels, replace entire image
        // * Four channel data
        // * Floating point data
        // * Pointer to data
        gl::TexSubImage2D(
            gl::TEXTURE_2D,
            0,
            0,
            0,
            256,
            256,
            gl::RGBA,
            gl::FLOAT,
            data.as_ptr() as *const GLvoid,
        );
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
}

impl DemoApp {
    pub fn new() -> DemoApp {
        DemoApp {
            ..Default::default()
        }
    }

    fn load_shaders(&mut self) {
        let mut vertex_shader = Shader::new(ShaderType::Vertex);
        vertex_shader.load_file("assets/shaders/texture-coordinates/texture-coordinates.vs.glsl");

        let mut fragment_shader = Shader::new(ShaderType::Fragment);
        fragment_shader.load_file("assets/shaders/texture-coordinates/texture-coordinates.fs.glsl");

        self.shader_program = ShaderProgram::new();
        self.shader_program
            .attach(vertex_shader)
            .attach(fragment_shader)
            .link();

        unsafe {
            gl::Enable(gl::DEPTH_TEST);
            gl::DepthFunc(gl::LEQUAL);
        }
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

        let (_, data) = load_ktx!("../assets/textures/pattern1.ktx").unwrap();
        self.texture_1 = prepare_texture(&data);
        self.texture_2 = create_procedural_texture();

        let (_, obj) = load_object!("../assets/objects/torus_nrms_tc.sbm").unwrap();
        self.object = obj;

        unsafe {
            gl::Enable(gl::DEPTH_TEST);
            gl::DepthFunc(gl::LEQUAL);
        }
    }

    fn render(&mut self, current_time: f32) {
        unsafe {
            gl::ClearBufferfv(gl::COLOR, 0, GRAY as *const f32);
            gl::ClearBufferfv(gl::DEPTH, 0, ONES as *const f32);

            gl::BindTexture(gl::TEXTURE_2D, self.texture_2);
        }

        self.shader_program.activate();

        let modelview_matrix_location = self.shader_program.uniform_location("modelview_matrix");
        let projection_matrix_location = self.shader_program.uniform_location("projection_matrix");
        let projection = perspective(Deg(60.0), self.aspect_ratio, 0.1_f32, 1000_f32);

        unsafe {
            gl::UniformMatrix4fv(
                projection_matrix_location,
                1,
                gl::FALSE,
                projection.as_ptr(),
            );

            let factor: f32 = current_time * 0.3;

            let modelview = Matrix4::from_translation(vec3(0.0, 0.0, -3.0))
                * Matrix4::from_axis_angle(
                    vec3(0.0, 1.0, 0.0).normalize(),
                    Deg(current_time * 19.3),
                )
                * Matrix4::from_axis_angle(
                    vec3(0.0, 0.0, 1.0).normalize(),
                    Deg(current_time * 21.1),
                );

            gl::UniformMatrix4fv(modelview_matrix_location, 1, gl::FALSE, modelview.as_ptr());
        }

        render_all(&self.object);
    }
}

fn main() {
    DemoApp::new().run("Texture Coordinates");
}
