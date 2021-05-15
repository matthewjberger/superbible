use anyhow::Result;
use gl::types::*;
use glutin::window::Window;
use support::{app::run_application, app::App, shader::ShaderProgram};

#[derive(Default)]
struct DemoApp {
    shader_program: ShaderProgram,
    vao: u32,
}

impl DemoApp {
    fn load_shaders(&mut self) {
        self.shader_program = ShaderProgram::new();

        #[rustfmt::skip]
        self.shader_program
            .vertex_shader("assets/shaders/tessellated-triangle/tessellated-triangle.vs.glsl")
            .tessellation_control_shader("assets/shaders/tessellated-triangle/tessellated-triangle.tcs.glsl")
            .tessellation_evaluation_shader("assets/shaders/tessellated-triangle/tessellated-triangle.tes.glsl")
            .fragment_shader("assets/shaders/tessellated-triangle/tessellated-triangle.fs.glsl")
            .link();
    }
}

impl App for DemoApp {
    fn initialize(&mut self, _window: &Window) -> Result<()> {
        self.load_shaders();
        unsafe {
            gl::CreateVertexArrays(1, &mut self.vao);
            gl::BindVertexArray(self.vao);
            gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);
        }
        Ok(())
    }

    fn render(&mut self, _time: f32) -> Result<()> {
        let background_color: [GLfloat; 4] = [0.0, 0.0, 0.0, 1.0];
        self.shader_program.activate();
        unsafe {
            gl::ClearBufferfv(gl::COLOR, 0, &background_color as *const f32);
            gl::DrawArrays(gl::PATCHES, 0, 3);
        }
        Ok(())
    }
}

fn main() -> Result<()> {
    let app = DemoApp::default();
    run_application(app, "Tessellated Triangle")
}
