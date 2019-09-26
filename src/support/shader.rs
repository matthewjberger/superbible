pub use gl::types::*;
use std::ffi::CString;
use std::{fs, ptr};

pub enum ShaderKind {
    Vertex,
    Fragment,
    Geometry,
    TessellationControl,
    TessellationEvaluation,
    Compute,
}

#[derive(Default)]
pub struct Shader {
    pub id: GLuint,
}

impl Shader {
    pub fn new(shader_type: ShaderKind) -> Shader {
        Shader {
            id: unsafe { gl::CreateShader(Shader::map_type(&shader_type)) },
        }
    }

    pub fn load_file(&mut self, path: &str) {
        self.load(&fs::read_to_string(path).unwrap());
    }

    pub fn load(&mut self, source: &str) {
        let source_str = CString::new(source.as_bytes()).unwrap();
        unsafe {
            gl::ShaderSource(self.id, 1, &source_str.as_ptr(), ptr::null());
            gl::CompileShader(self.id);
        }
    }

    fn map_type(shader_type: &ShaderKind) -> GLuint {
        match shader_type {
            ShaderKind::Vertex => gl::VERTEX_SHADER,
            ShaderKind::Fragment => gl::FRAGMENT_SHADER,
            ShaderKind::Geometry => gl::GEOMETRY_SHADER,
            ShaderKind::TessellationControl => gl::TESS_CONTROL_SHADER,
            ShaderKind::TessellationEvaluation => gl::TESS_EVALUATION_SHADER,
            ShaderKind::Compute => gl::COMPUTE_SHADER,
        }
    }
}

#[derive(Default)]
pub struct ShaderProgram {
    pub id: GLuint,
    pub shader_ids: Vec<GLuint>,
}

impl ShaderProgram {
    pub fn new() -> Self {
        ShaderProgram {
            id: unsafe { gl::CreateProgram() },
            shader_ids: Vec::new(),
        }
    }

    fn attach(&mut self, kind: ShaderKind, path: &str) -> &mut Self {
        let mut shader = Shader::new(kind);
        shader.load_file(path);
        unsafe {
            gl::AttachShader(self.id, shader.id);
        }
        self.shader_ids.push(shader.id);
        self
    }

    pub fn vertex_shader(&mut self, path: &str) -> &mut Self {
        self.attach(ShaderKind::Vertex, path)
    }

    pub fn geometry_shader(&mut self, path: &str) -> &mut Self {
        self.attach(ShaderKind::Geometry, path)
    }

    pub fn tessellation_control_shader(&mut self, path: &str) -> &mut Self {
        self.attach(ShaderKind::TessellationControl, path)
    }

    pub fn tessellation_evaluation_shader(&mut self, path: &str) -> &mut Self {
        self.attach(ShaderKind::TessellationEvaluation, path)
    }

    pub fn compute_shader(&mut self, path: &str) -> &mut Self {
        self.attach(ShaderKind::Compute, path)
    }

    pub fn fragment_shader(&mut self, path: &str) -> &mut Self {
        self.attach(ShaderKind::Fragment, path)
    }

    pub fn link(&mut self) {
        unsafe {
            gl::LinkProgram(self.id);
            for id in &self.shader_ids {
                gl::DeleteShader(*id);
            }
        }
        self.shader_ids.clear();
    }

    pub fn activate(&self) {
        unsafe {
            gl::UseProgram(self.id);
        }
    }

    pub fn uniform_location(&self, name: &str) -> GLint {
        let name: CString = CString::new(name.as_bytes()).unwrap();
        unsafe { gl::GetUniformLocation(self.id, name.as_ptr()) }
    }
}

impl Drop for ShaderProgram {
    fn drop(&mut self) {
        unsafe { gl::DeleteProgram(self.id) }
    }
}
