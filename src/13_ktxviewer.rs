use support::app::*;
use support::ktx::prepare_texture;
use support::load_ktx;
use support::shader::*;

static GREEN: &[GLfloat; 4] = &[0.0, 0.25, 0.0, 1.0];

#[derive(Default)]
struct DemoApp {
    shader_program: ShaderProgram,
    vao: u32,
    texture: u32,
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
            .vertex_shader("assets/shaders/ktx-viewer/ktx-viewer.vs.glsl")
            .fragment_shader("assets/shaders/ktx-viewer/ktx-viewer.fs.glsl")
            .link();
    }
}

impl App for DemoApp {
    fn initialize(&mut self, _: &mut glfw::Window) {
        self.load_shaders();
        let (_, data) = load_ktx!("../assets/textures/tree.ktx").unwrap();
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
    }

    fn render(&mut self, current_time: f32) {
        self.shader_program.activate();
        unsafe {
            gl::ClearBufferfv(gl::COLOR, 0, GREEN as *const f32);
            gl::Uniform1f(0, current_time.sin() as f32 * 16.0 + 16.0);
            gl::DrawArrays(gl::TRIANGLE_STRIP, 0, 4);
        }
    }
}

fn main() {
    DemoApp::new().run("KTX Viewer");
}
