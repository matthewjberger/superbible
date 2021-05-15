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
        self.shader_program
            .vertex_shader("assets/shaders/point/point.vs.glsl")
            .fragment_shader("assets/shaders/point/point.fs.glsl")
            .link();
    }
}

impl App for DemoApp {
    fn initialize(&mut self, _window: &Window) -> Result<()> {
        self.load_shaders();
        unsafe {
            gl::CreateVertexArrays(1, &mut self.vao);
            gl::BindVertexArray(self.vao);
        }
        Ok(())
    }

    fn render(&mut self, _time: f32) -> Result<()> {
        self.shader_program.activate();
        unsafe {
            let background_color: [GLfloat; 4] = [1.0, 0.0, 0.0, 1.0];
            gl::ClearBufferfv(gl::COLOR, 0, &background_color as *const f32);
            gl::PointSize(40.0);
            gl::DrawArrays(gl::POINTS, 0, 1);
        }
        Ok(())
    }
}

fn main() -> Result<()> {
    let app = DemoApp::default();
    run_application(app, "Single Point")
}
