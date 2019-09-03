pub use gl::types::*;
use std::ffi::CString;
use std::ptr;

pub enum ShaderType {
    Vertex,
    Fragment,
    Geometry,
    TesselationControl,
    TesselationEvaluation,
    Compute,
}

#[derive(Default)]
pub struct Shader {
    pub id: GLuint,
}

impl Shader {
    pub fn new(shader_type: ShaderType) -> Shader {
        Shader {
            id: unsafe { gl::CreateShader(Shader::map_type(shader_type)) },
        }
    }

    pub fn load(&mut self, source: &str) {
        let source_str = CString::new(source.as_bytes()).unwrap();
        unsafe {
            gl::ShaderSource(self.id, 1, &source_str.as_ptr(), ptr::null());
            gl::CompileShader(self.id);
        }
    }

    fn map_type(shader_type: ShaderType) -> GLuint {
        match shader_type {
            ShaderType::Vertex => gl::VERTEX_SHADER,
            ShaderType::Fragment => gl::FRAGMENT_SHADER,
            ShaderType::Geometry => gl::GEOMETRY_SHADER,
            ShaderType::TesselationControl => gl::TESS_CONTROL_SHADER,
            ShaderType::TesselationEvaluation => gl::TESS_EVALUATION_SHADER,
            ShaderType::Compute => gl::COMPUTE_SHADER,
        }
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteShader(self.id);
        }
    }
}

#[derive(Default)]
pub struct ShaderProgram {
    pub id: GLuint,
}

impl ShaderProgram {
    pub fn new() -> ShaderProgram {
        ShaderProgram {
            id: unsafe { gl::CreateProgram() },
        }
    }

    pub fn attach(&self, shader: Shader) -> &ShaderProgram {
        unsafe {
            gl::AttachShader(self.id, shader.id);
        }
        self
    }

    pub fn link(&self) {
        unsafe {
            gl::LinkProgram(self.id);
        }
    }
}
