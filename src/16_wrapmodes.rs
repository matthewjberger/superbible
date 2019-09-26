use support::app::*;
use support::ktx::prepare_texture;
use support::load_ktx;
use support::shader::*;

const GREEN: &[GLfloat; 4] = &[0.0, 0.1, 0.0, 1.0];
const YELLOW: &[GLfloat; 4] = &[0.4, 0.4, 0.0, 1.0];
const OFFSETS: &[[GLfloat; 2]; 4] = &[[-0.5, -0.5], [0.5, -0.5], [-0.5, 0.5], [0.5, 0.5]];
const WRAP_MODES: &[GLenum; 4] = &[
    gl::CLAMP_TO_EDGE,
    gl::REPEAT,
    gl::CLAMP_TO_BORDER,
    gl::MIRRORED_REPEAT,
];

#[derive(Default)]
struct DemoApp {
    shader_program: ShaderProgram,
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
            .vertex_shader("assets/shaders/wrapmodes/wrapmodes.vs.glsl")
            .fragment_shader("assets/shaders/wrapmodes/wrapmodes.fs.glsl")
            .link();
    }
}

impl App for DemoApp {
    fn initialize(&mut self, _: &mut glfw::Window) {
        self.load_shaders();
        let (_, data) = load_ktx!("../assets/textures/rightarrows.ktx").unwrap();
        let mut vao = 0;
        let texture = prepare_texture(&data);
        unsafe {
            gl::GenVertexArrays(1, &mut vao);
            gl::BindVertexArray(vao);
            gl::BindTexture(gl::TEXTURE_2D, texture);
        }
    }

    fn render(&mut self, _: f32) {
        self.shader_program.activate();
        unsafe {
            gl::ClearBufferfv(gl::COLOR, 0, GREEN as *const f32);

            gl::TexParameterfv(
                gl::TEXTURE_2D,
                gl::TEXTURE_BORDER_COLOR,
                YELLOW as *const f32,
            );

            for (index, offset) in OFFSETS.iter().enumerate() {
                gl::Uniform2fv(0, 1, offset.as_ptr() as *const f32);
                gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, WRAP_MODES[index] as i32);
                gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, WRAP_MODES[index] as i32);
                gl::DrawArrays(gl::TRIANGLE_STRIP, 0, 4);
            }
        }
    }
}

fn main() {
    DemoApp::new().run("Wrap Modes");
}
