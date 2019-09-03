use std::ffi::CString;
use std::ptr;
use support::app::*;

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
    shader_program: u32,
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
}

impl App for DemoApp {
    fn settings(&mut self) -> &AppSettings {
        &self.settings
    }

    fn initialize(&mut self) {
        self.shader_program = compile_shaders();

        unsafe {
            gl::CreateVertexArrays(1, &mut self.vao);
            gl::BindVertexArray(self.vao);
        }
    }

    fn render(&mut self, _: f32) {
        unsafe {
            gl::ClearBufferfv(gl::COLOR, 0, RED as *const f32);
            gl::UseProgram(self.shader_program);
            gl::PointSize(40.0);
            gl::DrawArrays(gl::POINTS, 0, 1);
        }
    }
}

fn compile_shaders() -> GLuint {
    let vertex_src_str = CString::new(VERTEX_SHADER_SOURCE.as_bytes()).unwrap();
    let fragment_src_str = CString::new(FRAGMENT_SHADER_SOURCE.as_bytes()).unwrap();

    let vertex_shader;
    let fragment_shader;
    let shader_program;

    unsafe {
        vertex_shader = gl::CreateShader(gl::VERTEX_SHADER);
        gl::ShaderSource(vertex_shader, 1, &vertex_src_str.as_ptr(), ptr::null());
        gl::CompileShader(vertex_shader);

        fragment_shader = gl::CreateShader(gl::FRAGMENT_SHADER);
        gl::ShaderSource(fragment_shader, 1, &fragment_src_str.as_ptr(), ptr::null());
        gl::CompileShader(fragment_shader);

        shader_program = gl::CreateProgram();
        gl::AttachShader(shader_program, vertex_shader);
        gl::AttachShader(shader_program, fragment_shader);
        gl::LinkProgram(shader_program);

        gl::DeleteShader(vertex_shader);
        gl::DeleteShader(fragment_shader);
    }

    shader_program
}

fn main() {
    run(&mut DemoApp::new());
}
