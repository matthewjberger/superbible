use anyhow::Result;
use gl::types::*;
use glutin::window::Window;
use support::{
    app::{run_application, App},
    ktx::prepare_texture,
    load_ktx,
    shader::ShaderProgram,
};

static GREEN: &[GLfloat; 4] = &[0.0, 0.25, 0.0, 1.0];

#[derive(Default)]
struct DemoApp {
    shader_program: ShaderProgram,
    vao: u32,
    texture: u32,
}

impl DemoApp {
    fn load_shaders(&mut self) {
        self.shader_program = ShaderProgram::new();
        self.shader_program
            .vertex_shader("assets/shaders/ktx-viewer/ktx-viewer.vs.glsl")
            .fragment_shader("assets/shaders/ktx-viewer/ktx-viewer.fs.glsl")
            .link();
    }
}

impl App for DemoApp {
    fn initialize(&mut self, _window: &Window) -> Result<()> {
        self.load_shaders();
        let (_, data) = load_ktx!("../../assets/textures/tree.ktx").unwrap();
        self.texture = prepare_texture(&data);
        unsafe {
            gl::GenVertexArrays(1, &mut self.vao);
            gl::BindVertexArray(self.vao);
            gl::BindTexture(gl::TEXTURE_2D, self.texture);
            gl::Viewport(
                0,
                0,
                data.header.pixel_width as i32,
                data.header.pixel_height as i32,
            );
        }
        Ok(())
    }

    fn render(&mut self, time: f32) -> Result<()> {
        self.shader_program.activate();
        unsafe {
            gl::ClearBufferfv(gl::COLOR, 0, GREEN as *const f32);
            gl::Uniform1f(0, time.sin() as f32 * 16.0 + 16.0);
            gl::DrawArrays(gl::TRIANGLE_STRIP, 0, 4);
        }
        Ok(())
    }
}

fn main() -> Result<()> {
    let app = DemoApp::default();
    run_application(app, "KTX Viewer")
}
