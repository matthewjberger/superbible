use crate::ktx::*;
use crate::shader::*;

#[derive(Default)]
pub struct TextOverlay {
    vbo: GLuint,
    vao: GLuint,
    font_texture: GLuint,
    shader_program: ShaderProgram,
    dirty: bool,
    buffer_width: i32,
    buffer_height: i32,
    screen_buffer: String,
    cursor_x: u32,
    cursor_y: u32,
}

impl TextOverlay {
    pub fn new() -> Self {
        TextOverlay {
            ..Default::default()
        }
    }

    pub fn initialize(&mut self, width: i32, height: i32) {
        let mut vertex_shader = Shader::new(ShaderType::Vertex);
        vertex_shader.load_file("assets/shaders/text-overlay/textoverlay.vs.glsl");

        let mut fragment_shader = Shader::new(ShaderType::Fragment);
        fragment_shader.load_file("assets/shaders/text-overlay/textoverlay.fs.glsl");

        self.shader_program = ShaderProgram::new();
        self.shader_program
            .attach(vertex_shader)
            .attach(fragment_shader)
            .link();

        self.buffer_width = width;
        self.buffer_height = height;

        unsafe {
            gl::GenVertexArrays(1, &mut self.vao);
            gl::BindVertexArray(self.vao);

            gl::GenTextures(1, &mut self.vbo);
            gl::BindTexture(gl::TEXTURE_2D, self.vbo);
            gl::TexStorage2D(
                gl::TEXTURE_2D,
                1,
                gl::R8UI,
                self.buffer_width as i32,
                self.buffer_height as i32,
            );
        }

        let (_, data) = load_ktx!("../../assets/textures/cp437_9x16.ktx").unwrap();
        self.font_texture = prepare_texture(&data);
        self.clear();
    }

    pub fn render(&mut self) {
        self.shader_program.activate();
        unsafe {
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, self.vbo);
        }
        if self.dirty {
            unsafe {
                gl::TexSubImage2D(
                    gl::TEXTURE_2D,
                    0,
                    0,
                    0,
                    self.buffer_width as i32,
                    self.buffer_height as i32,
                    gl::RED_INTEGER,
                    gl::UNSIGNED_BYTE,
                    self.screen_buffer.as_ptr() as *const GLvoid,
                )
            }
            self.dirty = false;
        }

        unsafe {
            gl::ActiveTexture(gl::TEXTURE1);
            gl::BindTexture(gl::TEXTURE_2D_ARRAY, self.font_texture);
            gl::BindVertexArray(self.vao);
            gl::DrawArrays(gl::TRIANGLE_STRIP, 0, 4);
        }
    }

    pub fn draw_text(&mut self, text: String, x_position: i32, y_position: i32) {
        let index = (y_position * self.buffer_width + x_position) as usize;
        self.dirty = true;
        self.screen_buffer.insert_str(index, &text);
        println!("{:?}", self.screen_buffer.trim_end_matches(" "));
    }

    pub fn clear(&mut self) {
        self.screen_buffer = " ".repeat(self.buffer_width as usize * self.buffer_height as usize);
        self.dirty = true;
        self.cursor_x = 0;
        self.cursor_y = 0;
    }
}
