use anyhow::Result;
use gl::types::*;
use glutin::window::Window;
use nalgebra_glm as glm;
use std::{cmp, mem, ptr};
use support::{
    app::{run_application, App},
    load_object,
    object::{render_all, Object},
    shader::ShaderProgram,
};

#[derive(Default)]
struct DemoApp {
    clear_program: ShaderProgram,
    append_program: ShaderProgram,
    resolve_program: ShaderProgram,
    uniform_buffer: u32,
    atomic_counter_buffer: u32,
    head_pointer_image: u32,
    vao: u32,
    fragment_buffer: u32,
    object: Object,
    aspect_ratio: f32,
}

impl DemoApp {
    fn update_aspect_ratio(&mut self, width: u32, height: u32) {
        self.aspect_ratio = width as f32 / cmp::max(height, 1) as f32;
    }

    fn load_shaders(&mut self) {
        self.clear_program = ShaderProgram::new();
        self.append_program = ShaderProgram::new();
        self.resolve_program = ShaderProgram::new();

        self.clear_program
            .vertex_shader("assets/shaders/fragment-list/clear.vs.glsl")
            .fragment_shader("assets/shaders/fragment-list/clear.fs.glsl")
            .link();

        self.append_program
            .vertex_shader("assets/shaders/fragment-list/append.vs.glsl")
            .fragment_shader("assets/shaders/fragment-list/append.fs.glsl")
            .link();

        self.resolve_program
            .vertex_shader("assets/shaders/fragment-list/resolve.vs.glsl")
            .fragment_shader("assets/shaders/fragment-list/resolve.fs.glsl")
            .link();
    }
}

impl App for DemoApp {
    fn on_resize(&mut self, width: u32, height: u32) -> Result<()> {
        self.update_aspect_ratio(width, height);
        Ok(())
    }

    fn initialize(&mut self, window: &Window) -> Result<()> {
        let inner_size = window.inner_size();
        let (width, height) = (inner_size.width, inner_size.height);
        self.update_aspect_ratio(width, height);
        self.load_shaders();

        let (_, obj) = load_object!("../../assets/objects/dragon.sbm").unwrap();
        self.object = obj;

        unsafe {
            gl::GenBuffers(1, &mut self.uniform_buffer);
            gl::BindBuffer(gl::UNIFORM_BUFFER, self.uniform_buffer);
            gl::BufferData(
                gl::UNIFORM_BUFFER,
                (mem::size_of::<glm::Mat4>() * 3) as GLsizeiptr,
                ptr::null(),
                gl::DYNAMIC_DRAW,
            );

            gl::GenBuffers(1, &mut self.fragment_buffer);
            gl::BindBuffer(gl::SHADER_STORAGE_BUFFER, self.fragment_buffer);
            gl::BufferData(
                gl::SHADER_STORAGE_BUFFER,
                1024 * 1024 * 16,
                ptr::null(),
                gl::DYNAMIC_COPY,
            );

            gl::GenBuffers(1, &mut self.atomic_counter_buffer);
            gl::BindBuffer(gl::ATOMIC_COUNTER_BUFFER, self.atomic_counter_buffer);
            gl::BufferData(gl::ATOMIC_COUNTER_BUFFER, 4, ptr::null(), gl::DYNAMIC_COPY);

            gl::GenTextures(1, &mut self.head_pointer_image);
            gl::BindTexture(gl::TEXTURE_2D, self.head_pointer_image);
            gl::TexStorage2D(gl::TEXTURE_2D, 1, gl::R32UI, 1024, 1024);

            gl::GenVertexArrays(1, &mut self.vao);
            gl::BindVertexArray(self.vao);
        }

        Ok(())
    }

    fn render(&mut self, time: f32) -> Result<()> {
        let model_matrix = glm::scaling(&glm::vec3(7.0, 7.0, 7.0));
        let view_position = glm::vec3(
            (time * 0.35).cos() * 120.0,
            (time * 0.4).cos() * 30.0,
            (time * 0.35).sin() * 120.0,
        );
        let view_matrix = glm::look_at(&view_position, &glm::vec3(0.0, 30.0, 0.0), &glm::Vec3::y());
        let projection_matrix =
            glm::perspective(self.aspect_ratio, 90_f32.to_radians(), 0.1_f32, 1000_f32);
        let mvp_matrix = projection_matrix * view_matrix * model_matrix;
        let mvp_matrix_location = self.append_program.uniform_location("mvp");

        unsafe {
            barrier();

            self.clear_program.activate();
            gl::BindVertexArray(self.vao);
            gl::DrawArrays(gl::TRIANGLE_STRIP, 0, 4);

            self.append_program.activate();
            gl::UniformMatrix4fv(mvp_matrix_location, 1, gl::FALSE, mvp_matrix.as_ptr());
            gl::BindBufferBase(gl::ATOMIC_COUNTER_BUFFER, 0, self.atomic_counter_buffer);
            gl::BufferSubData(
                gl::ATOMIC_COUNTER_BUFFER,
                0,
                mem::size_of::<u32>() as GLsizeiptr,
                Box::into_raw(Box::new(0)) as *const GLvoid,
            );
            gl::BindBufferBase(gl::SHADER_STORAGE_BUFFER, 0, self.fragment_buffer);
            gl::BindImageTexture(
                0,
                self.head_pointer_image,
                0,
                gl::FALSE,
                0,
                gl::READ_WRITE,
                gl::R32UI,
            );

            barrier();
            render_all(&self.object);

            barrier();
            self.resolve_program.activate();
            gl::BindVertexArray(self.vao);

            barrier();
            gl::DrawArrays(gl::TRIANGLE_STRIP, 0, 4);
        }

        Ok(())
    }
}

unsafe fn barrier() {
    gl::MemoryBarrier(
        gl::SHADER_IMAGE_ACCESS_BARRIER_BIT
            | gl::ATOMIC_COUNTER_BARRIER_BIT
            | gl::SHADER_STORAGE_BARRIER_BIT,
    );
}

fn main() -> Result<()> {
    let app = DemoApp::default();
    run_application(app, "Fragment List")
}
