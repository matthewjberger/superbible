use support::app::*;
use support::shader::*;

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

        #[rustfmt::skip]
        self.shader_program
            .vertex_shader("assets/shaders/tessellated-triangle/tessellated-triangle.vs.glsl")
            .tessellation_control_shader("assets/shaders/tessellated-triangle/tessellated-triangle.tcs.glsl")
            .tessellation_evaluation_shader("assets/shaders/geometry-shader/geometry-shader.tes.glsl")
            .geometry_shader("assets/shaders/geometry-shader/geometry-shader.gs.glsl")
            .fragment_shader("assets/shaders/geometry-shader/geometry-shader.fs.glsl")
            .link();
    }
}

impl App for DemoApp {
    fn initialize(&mut self, _: &mut glfw::Window) {
        self.load_shaders();
        unsafe {
            gl::CreateVertexArrays(1, &mut self.vao);
            gl::BindVertexArray(self.vao);
            gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);
            gl::PointSize(5.0);
        }
    }

    fn render(&mut self, _: f32) {
        let background_color: [GLfloat; 4] = [0.0, 0.0, 0.0, 1.0];
        self.shader_program.activate();
        unsafe {
            gl::ClearBufferfv(gl::COLOR, 0, &background_color as *const f32);
            gl::DrawArrays(gl::PATCHES, 0, 3);
        }
    }
}

fn main() {
    DemoApp::new().run("Geometry Shader");
}
