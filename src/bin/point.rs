use support::app::*;
use support::shader::*;

const RED: &[GLfloat; 4] = &[1.0, 0.0, 0.0, 1.0];

#[derive(Default)]
struct DemoApp {
    shader_program: ShaderProgram,
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
            .vertex_shader("assets/shaders/point/point.vs.glsl")
            .fragment_shader("assets/shaders/point/point.fs.glsl")
            .link();
    }
}

impl App for DemoApp {
    fn initialize(&mut self, _: &mut glfw::Window) {
        self.load_shaders();
        unsafe {
            gl::CreateVertexArrays(1, &mut self.vao);
            gl::BindVertexArray(self.vao);
        }
    }

    fn render(&mut self, _: f32) {
        self.shader_program.activate();
        unsafe {
            gl::ClearBufferfv(gl::COLOR, 0, RED as *const f32);
            gl::PointSize(40.0);
            gl::DrawArrays(gl::POINTS, 0, 1);
        }
    }
}

fn main() {
    DemoApp::new().run("Single Point");
}
