use support::app::*;
use support::ktx::prepare_texture;
use support::load_ktx;
use support::shader::*;
use support::text::*;

const BLACK: &[GLfloat; 4] = &[0.0, 0.0, 0.0, 0.0];

#[derive(Default)]
struct DemoApp {
    shader_program: ShaderProgram,
    text_overlay: TextOverlay,
}

impl DemoApp {
    pub fn new() -> DemoApp {
        DemoApp {
            ..Default::default()
        }
    }

    fn load_shaders(&mut self) {
        let mut vertex_shader = Shader::new(ShaderType::Vertex);
        vertex_shader.load_file("assets/shaders/mirrorclampedge/mirrorclampedge.vs.glsl");

        let mut fragment_shader = Shader::new(ShaderType::Fragment);
        fragment_shader.load_file("assets/shaders/mirrorclampedge/mirrorclampedge.fs.glsl");

        self.shader_program = ShaderProgram::new();
        self.shader_program
            .attach(vertex_shader)
            .attach(fragment_shader)
            .link();
    }
}

impl App for DemoApp {
    fn initialize(&mut self, window: &mut glfw::Window) {
        self.load_shaders();
        let (_, data) = load_ktx!("../assets/textures/flare.ktx").unwrap();
        let texture = prepare_texture(&data);
        let mut vao = 0;
        unsafe {
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, texture);
            gl::GenVertexArrays(1, &mut vao);
            gl::BindVertexArray(vao);
        }

        let (width, height) = window.get_size();
        self.text_overlay.initialize(width, height);
    }

    fn render(&mut self, _: f32) {
        self.shader_program.activate();
        unsafe {
            gl::ClearBufferfv(gl::COLOR, 0, BLACK as *const f32);

            gl::TexParameteri(
                gl::TEXTURE_2D,
                gl::TEXTURE_WRAP_S,
                gl::CLAMP_TO_BORDER as i32,
            );
            gl::TexParameteri(
                gl::TEXTURE_2D,
                gl::TEXTURE_WRAP_T,
                gl::CLAMP_TO_BORDER as i32,
            );

            gl::DrawArrays(gl::TRIANGLE_STRIP, 0, 4);
        }
        self.text_overlay.clear();
        self.text_overlay.draw_text("Testing!".to_string(), 0, 0);
        self.text_overlay.render();
    }
}

fn main() {
    DemoApp::new().run("Mirror Clamp Edge");
}
