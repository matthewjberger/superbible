use support::app::*;
use support::load_ktx;
use support::shader::*;

static VERTEX_SHADER_SOURCE: &str = "
#version 450 core
void main(void) {
    const vec4 vertices[] = vec4[](vec4(-1.0, -1.0, 0.5, 1.0),
                                   vec4( 1.0, -1.0, 0.5, 1.0),
                                   vec4(-1.0,  1.0, 0.5, 1.0),
                                   vec4( 1.0,  1.0, 0.5, 1.0));
    gl_Position = vertices[gl_VertexID];
}
";

static FRAGMENT_SHADER_SOURCE: &str = "
#version 450 core

uniform sampler2D s;

uniform float exposure;

out vec4 color;

void main(void)
{
    color = texture(s, gl_FragCoord.xy / textureSize(s, 0)) * exposure;
}
";

static GREEN: &[GLfloat; 4] = &[0.0, 0.25, 0.0, 1.0];

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
    fn initialize(&mut self, _: &mut glfw::Window) {
        self.load_shaders();

        let mut texture = 0;

        // Load a texture
        let (_, data) = load_ktx!("../assets/textures/tree.ktx").unwrap();
        let ktx = data.header;
        let image = data.pixels;

        unsafe {
            gl::GenTextures(1, &mut texture);
            gl::BindTexture(gl::TEXTURE_2D, texture);

            gl::TexStorage2D(
                gl::TEXTURE_2D,
                ktx.mip_levels as i32,
                ktx.gl_internal_format,
                ktx.pixel_width as i32,
                ktx.pixel_height as i32,
            );

            gl::PixelStorei(gl::UNPACK_ALIGNMENT, 1);

            gl::TexSubImage2D(
                gl::TEXTURE_2D,
                0,
                0,
                0,
                ktx.pixel_width as i32,
                ktx.pixel_height as i32,
                ktx.gl_format,
                ktx.gl_type,
                image.as_ptr() as *const GLvoid,
            );

            gl::Viewport(0, 0, ktx.pixel_width as i32, ktx.pixel_height as i32);

            gl::GenVertexArrays(1, &mut self.vao);
            gl::BindVertexArray(self.vao);
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
