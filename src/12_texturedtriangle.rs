use support::app::*;
use support::shader::*;

static VERTEX_SHADER_SOURCE: &str = "
#version 450 core
void main(void) {
    const vec4 vertices[] = vec4[](vec4( 0.75, -0.75, 0.5, 1.0),
                                   vec4(-0.75, -0.75, 0.5, 1.0),
                                   vec4( 0.75,  0.75, 0.5, 1.0));
    gl_Position = vertices[gl_VertexID];
}
";

static FRAGMENT_SHADER_SOURCE: &str = "
#version 450 core

uniform sampler2D s;

out vec4 color;

void main(void)
{
    color = texture(s, gl_FragCoord.xy / textureSize(s, 0));
}
";

static GREEN: &[GLfloat; 4] = &[0.0, 0.25, 0.0, 1.0];

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
                title: "Textured Triangle".to_string(),
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
        let (width, height) = (256 as usize, 256 as usize);
        let mut texture = 0;

        // Define data to upload into the texture
        let mask = 0xFF;
        let max = 255.0;
        let mut data = vec![1.0; width * height * 4];
        for y in 0..height {
            for x in 0..width {
                let index = y * width + x;
                data[index * 4] = ((x & y) & mask) as f32 / max;
                data[index * 4 + 1] = ((x | y) & mask) as f32 / max;
                data[index * 4 + 2] = ((x ^ y) & mask) as f32 / max;
                data[index * 4 + 3] = 1.0;
            }
        }

        unsafe {
            // Generate a name for the texture
            gl::GenTextures(1, &mut texture);

            // Bind it to the context using the GL_TEXTURE_2D binding point
            gl::BindTexture(gl::TEXTURE_2D, texture);

            // Specify the amount of storage we want to use for this texture
            // * 2D texture
            // * 8 mipmap levels
            // * 32-bit floating point RGBA data
            // * 256 x 256 texels
            gl::TexStorage2D(gl::TEXTURE_2D, 8, gl::RGBA32F, width as i32, height as i32);

            // Specify a two dimensional texture subimage
            // * 2D texture
            // * Level 0
            // * Offset 0, 0
            // * 256 x 256 texels, replace entire image
            // * Four channel data
            // * Floating point data
            // * Pointer to data
            gl::TexSubImage2D(
                gl::TEXTURE_2D,
                0,
                0,
                0,
                256,
                256,
                gl::RGBA,
                gl::FLOAT,
                data.as_ptr() as *const GLvoid,
            );

            gl::GenVertexArrays(1, &mut self.vao);
            gl::BindVertexArray(self.vao);
        }
    }

    fn render(&mut self, _: f32) {
        self.shader_program.activate();
        unsafe {
            gl::ClearBufferfv(gl::COLOR, 0, GREEN as *const f32);
            gl::DrawArrays(gl::TRIANGLES, 0, 3);
        }
    }
}

fn main() {
    run(&mut DemoApp::new());
}
