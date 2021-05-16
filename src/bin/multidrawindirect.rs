use anyhow::Result;
use gl::types::*;
use glutin::{event::ElementState, event::VirtualKeyCode, window::Window};
use nalgebra_glm as glm;
use support::{
    app::{run_application, App},
    load_object,
    object::Object,
    shader::ShaderProgram,
};

#[derive(Default, Copy, Clone)]
struct DrawArraysIndirectCommand {
    count: GLuint,
    prim_count: GLuint,
    first: GLuint,
    base_instance: GLuint,
}

#[derive(Default)]
struct DemoApp {
    shader_program: ShaderProgram,
    asteroids: Object,
    multidraw_active: bool,
}

const NUM_DRAWS: usize = 50000;

impl DemoApp {
    fn load_shaders(&mut self) {
        self.shader_program = ShaderProgram::new();
        self.shader_program
            .vertex_shader("assets/shaders/multidrawindirect/render.vs.glsl")
            .fragment_shader("assets/shaders/multidrawindirect/render.fs.glsl")
            .link();
    }
}

impl App for DemoApp {
    fn initialize(&mut self, _window: &Window) -> Result<()> {
        self.multidraw_active = true;
        self.load_shaders();
        let (_, object) = load_object!("../../assets/objects/asteroids.sbm")?;
        self.asteroids = object;

        unsafe {
            let mut indirect_draw_buffer = 0;
            gl::GenBuffers(1, &mut indirect_draw_buffer);
            gl::BindBuffer(gl::DRAW_INDIRECT_BUFFER, indirect_draw_buffer);
            gl::BufferData(
                gl::DRAW_INDIRECT_BUFFER,
                (NUM_DRAWS * std::mem::size_of::<DrawArraysIndirectCommand>()) as _,
                std::ptr::null(),
                gl::STATIC_DRAW,
            );

            let indirect_draw_buffer = gl::MapBufferRange(
                gl::DRAW_INDIRECT_BUFFER,
                0,
                (NUM_DRAWS * std::mem::size_of::<DrawArraysIndirectCommand>()) as _,
                gl::MAP_WRITE_BIT | gl::MAP_INVALIDATE_BUFFER_BIT,
            );

            let number_of_subobjects = self.asteroids.sub_objects.len();
            let mut command_buffer = vec![DrawArraysIndirectCommand::default(); NUM_DRAWS];
            for index in 0..NUM_DRAWS {
                let sub_object = &self.asteroids.sub_objects[index % number_of_subobjects];
                command_buffer[index].count = sub_object.count;
                command_buffer[index].prim_count = 1;
                command_buffer[index].first = sub_object.first;
                command_buffer[index].base_instance = index as _;
            }
            std::ptr::copy(
                command_buffer.as_ptr(),
                indirect_draw_buffer as *mut DrawArraysIndirectCommand,
                command_buffer.len(),
            );

            gl::UnmapBuffer(gl::DRAW_INDIRECT_BUFFER);

            gl::BindVertexArray(self.asteroids.vao);

            let mut draw_index_buffer = 0;
            gl::GenBuffers(1, &mut draw_index_buffer);
            gl::BindBuffer(gl::ARRAY_BUFFER, draw_index_buffer);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (NUM_DRAWS * std::mem::size_of::<GLuint>()) as _,
                std::ptr::null(),
                gl::STATIC_DRAW,
            );

            let mut draw_indices = vec![0; NUM_DRAWS];
            let draw_index_buffer = gl::MapBufferRange(
                gl::ARRAY_BUFFER,
                0,
                (NUM_DRAWS * std::mem::size_of::<GLuint>()) as _,
                gl::MAP_WRITE_BIT | gl::MAP_INVALIDATE_BUFFER_BIT,
            );
            for index in 0..NUM_DRAWS {
                draw_indices[index] = index as _;
            }
            std::ptr::copy(
                draw_indices.as_ptr(),
                draw_index_buffer as *mut GLuint,
                draw_indices.len(),
            );

            gl::UnmapBuffer(gl::ARRAY_BUFFER);

            gl::VertexAttribIPointer(10, 1, gl::UNSIGNED_INT, 0, std::ptr::null());
            gl::VertexAttribDivisor(10, 1);
            gl::EnableVertexAttribArray(10);

            gl::Enable(gl::DEPTH_TEST);
            gl::DepthFunc(gl::LEQUAL);

            gl::Enable(gl::CULL_FACE);
        }

        Ok(())
    }

    fn render(&mut self, time: f32) -> Result<()> {
        let black: [GLfloat; 4] = [0.0, 0.0, 0.0, 1.0];
        let one: [GLfloat; 1] = [1.0];

        unsafe {
            gl::ClearBufferfv(gl::COLOR, 0, &black as *const f32);
            gl::ClearBufferfv(gl::DEPTH, 0, &one as *const f32);
        }

        let view_matrix = glm::look_at(
            &glm::vec3(
                100.0 * (time * 0.023).cos(),
                100.0 * (time * 0.023).cos(),
                300.0 * (time * 0.037).sin() - 600.0,
            ),
            &glm::vec3(0.0, 0.0, 260.0),
            &glm::vec3(0.1 - (time * 0.1).cos() * 0.3, 1.0, 0.0).normalize(),
        );

        let projection_matrix = glm::perspective(4.0 / 3.0, 50_f32.to_radians(), 1.0, 2000.0);

        self.shader_program.activate();

        let time_location = self.shader_program.uniform_location("time");
        let view_matrix_location = self.shader_program.uniform_location("view_matrix");
        let projection_matrix_location = self.shader_program.uniform_location("proj_matrix");
        let viewproj_location = self.shader_program.uniform_location("viewproj_matrix");

        unsafe {
            gl::Uniform1f(time_location, time);
            gl::UniformMatrix4fv(
                view_matrix_location,
                1,
                gl::FALSE,
                view_matrix.as_ptr() as *const _,
            );
            gl::UniformMatrix4fv(
                projection_matrix_location,
                1,
                gl::FALSE,
                projection_matrix.as_ptr() as *const _,
            );
            gl::UniformMatrix4fv(
                viewproj_location,
                1,
                gl::FALSE,
                (projection_matrix * view_matrix).as_ptr() as *const _,
            );
            gl::BindVertexArray(self.asteroids.vao);

            let number_of_subobjects = self.asteroids.sub_objects.len();
            if self.multidraw_active {
                gl::MultiDrawArraysIndirect(gl::TRIANGLES, std::ptr::null(), NUM_DRAWS as _, 0);
            } else {
                for index in 0..NUM_DRAWS {
                    let sub_object = &self.asteroids.sub_objects[index % number_of_subobjects];
                    gl::DrawArraysInstancedBaseInstance(
                        gl::TRIANGLES,
                        sub_object.first as _,
                        sub_object.count as _,
                        1,
                        index as _,
                    );
                }
            }
        }

        Ok(())
    }

    fn on_key(&mut self, keycode: &VirtualKeyCode, keystate: &ElementState) -> Result<()> {
        match (keycode, keystate) {
            (VirtualKeyCode::D, ElementState::Pressed) => {
                self.multidraw_active = !self.multidraw_active;
            }
            (VirtualKeyCode::R, ElementState::Pressed) => {
                self.load_shaders();
            }
            _ => (),
        }

        Ok(())
    }

    fn update(&mut self) -> Result<()> {
        Ok(())
    }

    fn cleanup(&mut self) -> Result<()> {
        Ok(())
    }

    fn handle_events(&mut self, _event: glutin::event::Event<()>, _window: &Window) -> Result<()> {
        Ok(())
    }

    fn on_resize(&mut self, _width: u32, _height: u32) -> Result<()> {
        Ok(())
    }
}

fn main() -> Result<()> {
    let app = DemoApp::default();
    run_application(app, "Asteroids")
}
