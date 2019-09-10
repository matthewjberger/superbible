use cgmath::prelude::*;
use cgmath::{perspective, vec3, Deg, Matrix, Matrix4};
use std::cmp;
use support::app::*;
use support::ktx::prepare_texture;
use support::load_ktx;
use support::load_object;
use support::object::{render_object, Object};
use support::shader::*;

const BACKGROUND_COLOR: [GLfloat; 4] = [0.0, 0.25, 0.0, 1.0];
const GRAY: &[GLfloat; 4] = &[0.2, 0.2, 0.2, 1.0];
const ONES: &[GLfloat; 1] = &[1.0];

#[derive(Default)]
struct DemoApp {
    shader_program: ShaderProgram,
    vao: u32,
    vbo: u32,
    aspect_ratio: f32,
    object: Object,
    texture: u32,
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
        let (_, obj) = load_object!("../assets/objects/torus_nrms_tc.sbm").unwrap();
        self.object = obj;

        unsafe {
            gl::Enable(gl::DEPTH_TEST);
            gl::DepthFunc(gl::LEQUAL);
        }

        // Load a texture
        let (_, data) = load_ktx!("../assets/textures/pattern1.ktx").unwrap();
        let (vao, texture) = prepare_texture(&data);
        self.vao = vao;
        self.texture = texture;
        unsafe {
            gl::BindVertexArray(self.vao);
            gl::BindTexture(gl::TEXTURE_2D, self.texture);
        }
    }

    fn render(&mut self, current_time: f32) {
        unsafe {
            gl::ClearBufferfv(gl::COLOR, 0, GRAY as *const f32);
            gl::ClearBufferfv(gl::DEPTH, 0, ONES as *const f32);
            gl::BindTexture(gl::TEXTURE_2D, self.texture);
        }

        self.shader_program.activate();

        let modelview_matrix_location = self.shader_program.uniform_location("modelview_matrix");
        let projection_matrix_location = self.shader_program.uniform_location("projection_matrix");
        let projection = perspective(Deg(60.0), self.aspect_ratio, 0.1_f32, 1000_f32);

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

            render_object(&self.object, 0, 1, 0);
        }
    }
}

fn main() {
    DemoApp::new().run("Texture Coordinates");
}
