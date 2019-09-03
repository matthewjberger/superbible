use support::app::*;
use support::shader::*;

static VERTEX_SHADER_SOURCE: &str = "
#version 450 core

layout (location = 0) in vec4 offset;

void main(void) {
    const vec4 vertices[3] = vec4[3](vec4( 0.25, -0.25, 0.5, 1.0),
                                     vec4(-0.25, -0.25, 0.5, 1.0),
                                     vec4( 0.25,  0.25, 0.5, 1.0));
    gl_Position = vertices[gl_VertexID] + offset;
}
";

static FRAGMENT_SHADER_SOURCE: &str = "
#version 420 core
out vec4 color;
void main(void)
{
    color = vec4(0.0, 0.8, 1.0, 1.0);
}
";

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
                title: "Moving Triangle".to_string(),
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

    fn render(&mut self, current_time: f32) {
        let color: [GLfloat; 4] = [
            (current_time.sin() * 0.5) + 0.5,
            (current_time.cos() * 0.5) + 0.5,
            0.0,
            1.0,
        ];

        let attribute: [GLfloat; 4] =
            [current_time.sin() * 0.5, current_time.cos() * 0.6, 0.0, 0.0];

        self.shader_program.activate();

        unsafe {
            gl::ClearBufferfv(gl::COLOR, 0, &color as *const f32);
            gl::VertexAttrib4fv(0, &attribute as *const f32);
            gl::DrawArrays(gl::TRIANGLES, 0, 3);
        }
    }
}

fn main() {
    run(&mut DemoApp::new());
}
