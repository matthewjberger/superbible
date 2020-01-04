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
    texture: u32,
    wrapmode: u32,
}

impl DemoApp {
    pub fn new() -> DemoApp {
        DemoApp {
            wrapmode: gl::CLAMP_TO_BORDER,
            ..Default::default()
        }
    }

    fn load_shaders(&mut self) {
        self.shader_program = ShaderProgram::new();
        self.shader_program
            .vertex_shader("assets/shaders/mirrorclampedge/mirrorclampedge.vs.glsl")
            .fragment_shader("assets/shaders/mirrorclampedge/mirrorclampedge.fs.glsl")
            .link();
    }

    pub fn toggle_wrapmode(&mut self) {
        if self.wrapmode == gl::CLAMP_TO_BORDER {
            self.wrapmode = gl::MIRROR_CLAMP_TO_EDGE;
        } else {
            self.wrapmode = gl::CLAMP_TO_BORDER;
        };
    }
}

impl App for DemoApp {
    fn initialize(&mut self, _: &mut glfw::Window) {
        self.load_shaders();
        // NOTE: The 'flare.ktx' texture doesn't load properly in the sb7 example code
        //       or here. It's likely to just be a broken asset.
        //       The concept here can still be demonstrated with any other texture however.
        let (_, data) = load_ktx!("../../assets/textures/star.ktx").unwrap();
        self.texture = prepare_texture(&data);
        let mut vao = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut vao);
            gl::BindVertexArray(vao);
        }

        self.text_overlay.initialize(80, 50);
    }

    fn render(&mut self, _: f32) {
        self.shader_program.activate();
        unsafe {
            gl::ClearBufferfv(gl::COLOR, 0, BLACK as *const f32);

            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, self.texture);

            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, self.wrapmode as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, self.wrapmode as i32);

            gl::DrawArrays(gl::TRIANGLE_STRIP, 0, 4);
        }

        let message = if self.wrapmode == gl::CLAMP_TO_BORDER {
            "Mode = GL_CLAMP_TO_BORDER (M toggles)".to_string()
        } else {
            "Mode = GL_MIRROR_CLAMP_TO_EDGE (M toggles)".to_string()
        };

        self.text_overlay.clear();
        self.text_overlay.draw_text(message, 0, 0);
        self.text_overlay.render();
    }

    fn on_key(&mut self, key: Key, action: Action) {
        if action != glfw::Action::Release {
            return;
        }
        match (key, action) {
            (glfw::Key::M, glfw::Action::Release) => {
                self.toggle_wrapmode();
            }
            (glfw::Key::R, glfw::Action::Release) => {
                self.load_shaders();
            }
            _ => (),
        }
    }
}

fn main() {
    DemoApp::new().run("Mirror Clamp Edge");
}
