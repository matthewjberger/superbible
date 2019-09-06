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
        vertex_shader.load_file("assets/shaders/moving-triangle/moving-triangle.vs.glsl");

        let mut fragment_shader = Shader::new(ShaderType::Fragment);
        fragment_shader.load_file("assets/shaders/moving-triangle/moving-triangle.fs.glsl");

        self.shader_program = ShaderProgram::new();
        self.shader_program
            .attach(vertex_shader)
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
    DemoApp::new().run("Moving Triangle");
}
