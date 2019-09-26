use rand::Rng;
use std::{mem, ptr};
use support::app::*;
use support::ktx::prepare_texture;
use support::load_ktx;
use support::shader::*;

const BLACK: &[GLfloat; 4] = &[0.0, 0.0, 0.0, 0.0];

#[derive(Default)]
struct Droplet {
    x_offset: f32,
    rotation_speed: f32,
    fall_speed: f32,
}

#[derive(Default)]
struct DemoApp {
    shader_program: ShaderProgram,
    alien_textures: u32,
    rain_buffer: u32,
    droplets: Vec<Droplet>,
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
            .vertex_shader("assets/shaders/alien-rain/alien-rain.vs.glsl")
            .fragment_shader("assets/shaders/alien-rain/alien-rain.fs.glsl")
            .link();
    }
}

impl App for DemoApp {
    fn initialize(&mut self, _: &mut glfw::Window) {
        self.load_shaders();

        let (_, data) = load_ktx!("../assets/textures/aliens.ktx").unwrap();
        self.alien_textures = prepare_texture(&data);

        let mut vao = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut vao);
            gl::BindVertexArray(vao);
            gl::BindTexture(gl::TEXTURE_2D_ARRAY, self.alien_textures);
            gl::TexParameteri(
                gl::TEXTURE_2D_ARRAY,
                gl::TEXTURE_MIN_FILTER,
                gl::LINEAR_MIPMAP_LINEAR as i32,
            );
            gl::GenBuffers(1, &mut self.rain_buffer);
            gl::BindBuffer(gl::UNIFORM_BUFFER, self.rain_buffer);
            gl::BufferData(
                gl::UNIFORM_BUFFER,
                256 * (mem::size_of::<GLfloat>() * 4) as GLsizeiptr,
                ptr::null(),
                gl::DYNAMIC_DRAW,
            );
        }

        let mut rng = rand::thread_rng();

        for index in 0..256 {
            self.droplets.push(Droplet {
                x_offset: rng.gen::<f32>() * 2.0 - 1.0,
                rotation_speed: (rng.gen::<f32>() + 0.5) * if index % 2 != 0 { -3.0 } else { 3.0 },
                fall_speed: rng.gen::<f32>() + 0.2,
            });
        }

        unsafe {
            gl::Enable(gl::BLEND);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
        }
    }

    fn render(&mut self, current_time: f32) {
        self.shader_program.activate();
        let mut droplet_buffer: [GLfloat; 256 * 4] = [0.0; 256 * 4];
        unsafe {
            gl::ClearBufferfv(gl::COLOR, 0, BLACK as *const f32);
            gl::DrawArrays(gl::TRIANGLE_STRIP, 0, 4);
            gl::BindBufferBase(gl::UNIFORM_BUFFER, 0, self.rain_buffer);
            let data = gl::MapBufferRange(
                gl::UNIFORM_BUFFER,
                0,
                256 * (mem::size_of::<GLfloat>() * 4) as GLsizeiptr,
                gl::MAP_WRITE_BIT | gl::MAP_INVALIDATE_BUFFER_BIT,
            );

            for (index, droplet) in self.droplets.iter().enumerate() {
                let fall_speed = 2.0 - ((current_time + index as f32) * droplet.fall_speed) % 4.31;
                let offset = index * 4;
                droplet_buffer[offset] = droplet.x_offset;
                droplet_buffer[offset + 1] = fall_speed;
                droplet_buffer[offset + 2] = current_time * droplet.rotation_speed;
                droplet_buffer[offset + 3] = 0.0;
            }

            ptr::copy(
                droplet_buffer.as_ptr(),
                data as *mut GLfloat,
                droplet_buffer.len(),
            );

            gl::UnmapBuffer(gl::UNIFORM_BUFFER);

            for alien_index in 0..256 {
                gl::VertexAttribI1i(0, alien_index);
                gl::DrawArrays(gl::TRIANGLE_STRIP, 0, 4);
            }
        }
    }
}

fn main() {
    DemoApp::new().run("Alien Rain");
}
