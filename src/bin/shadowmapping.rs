use anyhow::Result;
use gl::types::*;
use glutin::{event::ElementState, event::VirtualKeyCode, window::Window};
use nalgebra_glm as glm;
use std::cmp;
use support::{
    app::{run_application, App},
    load_object,
    object::{render_all, Object},
    shader::ShaderProgram,
};

const GRAY: &[GLfloat; 4] = &[0.2, 0.2, 0.2, 1.0];
const ONES: &[GLfloat; 1] = &[1.0];
const ZERO: &[GLfloat; 1] = &[0.0];

const DEPTH_TEXTURE_SIZE: GLsizei = 4096;

#[derive(Default)]
struct Uniforms {
    pub light: LightUniform,
    pub view: ViewUniform,
}

#[derive(Default)]
struct LightUniform {
    mvp: GLint,
}

#[derive(Default)]
struct ViewUniform {
    pub mv_matrix: GLint,
    pub proj_matrix: GLint,
    pub shadow_matrix: GLint,
    pub full_shading: GLint,
}

#[derive(Default)]
struct Model {
    pub object: Object,
    pub model_matrix: glm::Mat4,
}

#[derive(Default, PartialEq)]
enum RenderMode {
    #[default]
    Full,
    Light,
    Depth,
}

#[derive(Default)]
struct DemoApp {
    mode: RenderMode,

    light_program: ShaderProgram,
    view_program: ShaderProgram,
    show_light_depth_program: ShaderProgram,

    uniforms: Uniforms,

    depth_fbo: GLuint,
    depth_texture: GLuint,
    depth_debug_texture: GLuint,

    dragon: Model,
    cube: Model,
    torus: Model,
    sphere: Model,

    light_view_matrix: glm::Mat4,
    light_proj_matrix: glm::Mat4,

    camera_view_matrix: glm::Mat4,
    camera_proj_matrix: glm::Mat4,

    quad_vao: GLuint,

    window_width: u32,
    window_height: u32,
}

impl DemoApp {
    fn load_shaders(&mut self) {
        self.light_program = ShaderProgram::new();
        self.light_program
            .vertex_shader("assets/shaders/shadowmapping/shadowmapping-light.vs.glsl")
            .fragment_shader("assets/shaders/shadowmapping/shadowmapping-light.fs.glsl")
            .link();

        self.view_program = ShaderProgram::new();
        self.view_program
            .vertex_shader("assets/shaders/shadowmapping/shadowmapping-camera.vs.glsl")
            .fragment_shader("assets/shaders/shadowmapping/shadowmapping-camera.fs.glsl")
            .link();

        self.show_light_depth_program = ShaderProgram::new();
        self.show_light_depth_program
            .vertex_shader("assets/shaders/shadowmapping/shadowmapping-light-view.vs.glsl")
            .fragment_shader("assets/shaders/shadowmapping/shadowmapping-light-view.fs.glsl")
            .link();

        unsafe {
            gl::Enable(gl::DEPTH_TEST);
            gl::DepthFunc(gl::LEQUAL);
        }
    }

    fn update_dimensions(&mut self, width: u32, height: u32) {
        self.window_width = width;
        self.window_height = height;
    }

    fn aspect_ratio(&mut self) -> f32 {
        self.window_width as f32 / cmp::max(self.window_height, 1) as f32
    }

    fn load_objects(&mut self) {
        let (_, object) = load_object!("../../assets/objects/dragon.sbm").unwrap();
        self.dragon = Model {
            object,
            model_matrix: glm::Mat4::identity(),
        };

        let (_, object) = load_object!("../../assets/objects/torus.sbm").unwrap();
        self.torus = Model {
            object,
            model_matrix: glm::Mat4::identity(),
        };

        let (_, object) = load_object!("../../assets/objects/sphere.sbm").unwrap();
        self.sphere = Model {
            object,
            model_matrix: glm::Mat4::identity(),
        };

        let (_, object) = load_object!("../../assets/objects/cube.sbm").unwrap();
        self.cube = Model {
            object,
            model_matrix: glm::Mat4::identity(),
        };
    }

    fn render_scene(&mut self, from_light: bool) {
        let scale_bias_matrix = glm::Mat4::from_columns(&[
            glm::vec4(0.5, 0.0, 0.0, 0.0),
            glm::vec4(0.0, 0.5, 0.0, 0.0),
            glm::vec4(0.0, 0.0, 0.5, 0.0),
            glm::vec4(0.5, 0.5, 0.5, 1.0),
        ]);

        let light_vp_matrix = self.light_proj_matrix * self.light_view_matrix;
        let shadow_sbpv_matrix =
            scale_bias_matrix * self.light_proj_matrix * self.light_view_matrix;

        if from_light {
            unsafe {
                gl::BindFramebuffer(gl::FRAMEBUFFER, self.depth_fbo);
                gl::Viewport(0, 0, DEPTH_TEXTURE_SIZE, DEPTH_TEXTURE_SIZE);
                gl::Enable(gl::POLYGON_OFFSET_FILL);
                gl::PolygonOffset(4.0, 4.0);
                self.light_program.activate();
                gl::DrawBuffers(1, &[gl::COLOR_ATTACHMENT0] as _);
                gl::ClearBufferfv(gl::COLOR, 0, ZERO as *const f32);
            }
        } else {
            unsafe {
                gl::Viewport(0, 0, self.window_width as _, self.window_height as _);
                gl::ClearBufferfv(gl::COLOR, 0, GRAY as *const f32);
                self.view_program.activate();
                gl::ActiveTexture(gl::TEXTURE0);
                gl::BindTexture(gl::TEXTURE_2D, self.depth_texture);
                gl::UniformMatrix4fv(
                    self.uniforms.view.proj_matrix,
                    1,
                    gl::FALSE,
                    self.camera_proj_matrix.as_ptr() as *const _,
                );
                gl::DrawBuffer(gl::BACK);
            }
        }

        unsafe {
            gl::ClearBufferfv(gl::DEPTH, 0, ONES as *const f32);
        }

        [&self.dragon, &self.torus, &self.cube, &self.sphere]
            .iter()
            .for_each(|model| {
                if from_light {
                    unsafe {
                        gl::UniformMatrix4fv(
                            self.uniforms.light.mvp,
                            1,
                            gl::FALSE,
                            (light_vp_matrix * (*model).model_matrix).as_ptr() as *const _,
                        );
                    }
                } else {
                    unsafe {
                        let shadow_matrix = shadow_sbpv_matrix * (*model).model_matrix;
                        gl::UniformMatrix4fv(
                            self.uniforms.view.shadow_matrix,
                            1,
                            gl::FALSE,
                            shadow_matrix.as_ptr() as *const _,
                        );
                        gl::UniformMatrix4fv(
                            self.uniforms.view.mv_matrix,
                            1,
                            gl::FALSE,
                            (self.camera_view_matrix * (*model).model_matrix).as_ptr() as *const _,
                        );
                        gl::Uniform1i(
                            self.uniforms.view.full_shading,
                            if self.mode == RenderMode::Full { 1 } else { 0 },
                        );
                    }
                }
                render_all(&(*model).object);
            });

        if from_light {
            unsafe {
                gl::Disable(gl::POLYGON_OFFSET_FILL);
                gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
            }
        } else {
            unsafe {
                gl::BindTexture(gl::TEXTURE_2D, 0);
            }
        }
    }
}

impl App for DemoApp {
    fn on_resize(&mut self, width: u32, height: u32) -> Result<()> {
        self.update_dimensions(width, height);
        Ok(())
    }

    fn on_key(&mut self, keycode: &VirtualKeyCode, keystate: &ElementState) -> Result<()> {
        match (keycode, keystate) {
            (VirtualKeyCode::F, ElementState::Pressed) => {
                self.mode = RenderMode::Full;
            }
            (VirtualKeyCode::L, ElementState::Pressed) => {
                self.mode = RenderMode::Light;
            }
            (VirtualKeyCode::D, ElementState::Pressed) => {
                self.mode = RenderMode::Depth;
            }
            (VirtualKeyCode::R, ElementState::Pressed) => {
                self.load_shaders();
            }
            _ => (),
        }

        Ok(())
    }

    fn initialize(&mut self, window: &Window) -> Result<()> {
        let inner_size = window.inner_size();
        let (width, height) = (inner_size.width, inner_size.height);
        self.update_dimensions(width, height);
        self.load_shaders();
        self.load_objects();

        unsafe {
            gl::GenFramebuffers(1, &mut self.depth_fbo);
            gl::BindFramebuffer(gl::FRAMEBUFFER, self.depth_fbo);

            gl::GenTextures(1, &mut self.depth_texture);
            gl::BindTexture(gl::TEXTURE_2D, self.depth_texture);
            gl::TexStorage2D(
                gl::TEXTURE_2D,
                11,
                gl::DEPTH_COMPONENT32F,
                DEPTH_TEXTURE_SIZE,
                DEPTH_TEXTURE_SIZE,
            );
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as _);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as _);
            gl::TexParameteri(
                gl::TEXTURE_2D,
                gl::TEXTURE_COMPARE_MODE,
                gl::COMPARE_REF_TO_TEXTURE as _,
            );
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_COMPARE_FUNC, gl::LEQUAL as _);

            gl::FramebufferTexture(gl::FRAMEBUFFER, gl::DEPTH_ATTACHMENT, self.depth_texture, 0);

            gl::GenTextures(1, &mut self.depth_debug_texture);
            gl::BindTexture(gl::TEXTURE_2D, self.depth_debug_texture);
            gl::TexStorage2D(
                gl::TEXTURE_2D,
                1,
                gl::R32F,
                DEPTH_TEXTURE_SIZE,
                DEPTH_TEXTURE_SIZE,
            );
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as _);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as _);

            gl::FramebufferTexture(
                gl::FRAMEBUFFER,
                gl::COLOR_ATTACHMENT0,
                self.depth_debug_texture,
                0,
            );

            gl::BindTexture(gl::TEXTURE_2D, 0);
            gl::BindFramebuffer(gl::FRAMEBUFFER, 0);

            gl::Enable(gl::DEPTH_TEST);

            gl::GenVertexArrays(1, &mut self.quad_vao);
            gl::BindVertexArray(self.quad_vao);
        }

        Ok(())
    }

    fn render(&mut self, time: f32) -> Result<()> {
        unsafe {
            gl::ClearBufferfv(gl::COLOR, 0, GRAY as *const f32);
            gl::ClearBufferfv(gl::DEPTH, 0, ONES as *const f32);
        }

        let time = time * 0.05;

        let light_position = glm::vec3(20.0, 20.0, 20.0);
        let view_position = glm::vec3(0.0, 0.0, 40.0);

        self.light_proj_matrix = glm::perspective(1.0, 90_f32.to_radians(), 1.0_f32, 200_f32);
        self.light_view_matrix =
            glm::look_at(&light_position, &glm::Vec3::zeros(), &glm::Vec3::y());

        self.camera_proj_matrix = glm::perspective(self.aspect_ratio(), 50.0, 1.0, 200.0);
        self.camera_view_matrix =
            glm::look_at(&view_position, &glm::Vec3::zeros(), &glm::Vec3::y());

        self.dragon.model_matrix = glm::rotation(time * 14.5, &glm::Vec3::y())
            * glm::rotation(20.0, &glm::Vec3::x())
            * glm::translation(&(glm::Vec3::y() * -4.0));

        self.sphere.model_matrix = glm::rotation(time * 3.7, &glm::Vec3::y())
            * glm::translation(&glm::vec3(
                (time * 0.37).sin() * 12.0,
                (time * 0.37).cos() * 12.0,
                0.0,
            ))
            * glm::scaling(&glm::vec3(2.0, 2.0, 2.0));

        self.cube.model_matrix = glm::rotation(time * 6.45, &glm::Vec3::y())
            * glm::translation(&glm::vec3(
                (time * 0.25).sin() * 10.0,
                (time * 0.25).cos() * 10.0,
                0.0,
            ))
            * glm::rotation(time * 99.0, &glm::Vec3::z())
            * glm::scaling(&glm::vec3(2.0, 2.0, 2.0));

        self.torus.model_matrix = glm::rotation(time * 5.25, &glm::Vec3::y())
            * glm::translation(&glm::vec3(
                (time * 0.51).sin() * 14.0,
                (time * 0.51).cos() * 14.0,
                0.0,
            ))
            * glm::rotation(time * 120.3, &glm::vec3(0.707106, 0.0, 0.707106))
            * glm::scaling(&glm::vec3(2.0, 2.0, 2.0));

        unsafe { gl::Enable(gl::DEPTH_TEST) };

        self.render_scene(true);

        if let RenderMode::Depth = self.mode {
            unsafe {
                gl::Disable(gl::DEPTH_TEST);
                gl::BindVertexArray(self.quad_vao);
                self.show_light_depth_program.activate();
                gl::BindTexture(gl::TEXTURE_2D, self.depth_debug_texture);
                gl::DrawArrays(gl::TRIANGLE_STRIP, 0, 4);
            }
        } else {
            self.render_scene(false);
        }

        Ok(())
    }
}

fn main() -> Result<()> {
    let app = DemoApp::default();
    run_application(app, "Shadow Mapping")
}
