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
        let mut vertex_shader = Shader::new(ShaderType::Vertex);
        vertex_shader.load(VERTEX_SHADER_SOURCE);

        let mut tessellation_control_shader = Shader::new(ShaderType::TessellationControl);
        tessellation_control_shader.load(TESSELLATION_CONTROL_SHADER_SOURCE);

        let mut tessellation_evaluation_shader = Shader::new(ShaderType::TessellationEvaluation);
        tessellation_evaluation_shader.load(TESSELLATION_EVALUATION_SHADER_SOURCE);

        let mut fragment_shader = Shader::new(ShaderType::Fragment);
        fragment_shader.load(FRAGMENT_SHADER_SOURCE);

        self.shader_program = ShaderProgram::new();
        self.shader_program
            .attach(vertex_shader)
            .attach(tessellation_control_shader)
            .attach(tessellation_evaluation_shader)
            .attach(fragment_shader)
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
    DemoApp::new().run("Tessellated Triangle");
}
