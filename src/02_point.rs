use support::app::*;
use support::shader::*;

static VERTEX_SHADER_SOURCE: &'static str = "
#version 420 core
void main(void) {
    gl_Position = vec4(0.0, 0.0, 0.5, 1.0);
}
";

static FRAGMENT_SHADER_SOURCE: &'static str = "
#version 420 core
out vec4 color;
void main(void)
{
    color = vec4(0.0, 0.8, 1.0, 1.0);
}
";

static RED: &'static [GLfloat; 4] = &[1.0, 0.0, 0.0, 1.0];

#[derive(Default)]
struct DemoApp {
    settings: AppSettings,
    shader_program: ShaderProgram,
    vao: u32,
}

impl DemoApp {
    pub fn new() -> DemoApp {
        DemoApp {
            settings: AppSettings {
                title: "Single Point".to_string(),
                ..Default::default()
            },
            ..Default::default()
        }
    }

    fn load_shaders(&mut self) {
        let mut vertex_shader = Shader::new(ShaderType::Vertex);
        vertex_shader.load(VERTEX_SHADER_SOURCE);

        let mut fragment_shader = Shader::new(ShaderType::Fragment);
        fragment_shader.load(FRAGMENT_SHADER_SOURCE);

        self.shader_program = ShaderProgram::new();
        self.shader_program
            .attach(vertex_shader)
            .attach(fragment_shader)
            .link();
    }
}

impl App for DemoApp {
    fn settings(&mut self) -> &AppSettings {
        &self.settings
    }

    fn initialize(&mut self) {
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
    run(&mut DemoApp::new());
}
