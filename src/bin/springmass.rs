use anyhow::Result;
use gl::types::*;
use glutin::{
    event::{ElementState, VirtualKeyCode},
    window::Window,
};
use rand::Rng;
use std::{mem, ptr};
use support::{
    app::run_application, app::App, ktx::prepare_texture, load_ktx, shader::ShaderProgram,
};

// const BLACK: &[GLfloat; 4] = &[0.0, 0.0, 0.0, 0.0];

enum BufferType {
    PositionA,
    PositionB,
    VelocityA,
    VelocityB,
    Connection,
}

#[derive(Default)]
struct DemoApp {
    update_program: ShaderProgram,
    render_program: ShaderProgram,
    draw_points: bool,
    draw_lines: bool,
    iterations_per_frame: u32,
}

impl DemoApp {
    fn load_shaders(&mut self) {
        self.update_program = ShaderProgram::new();
        self.update_program
            .vertex_shader("assets/shaders/spring-mass/update.vs.glsl")
            .fragment_shader("assets/shaders/spring-mass/update.fs.glsl")
            .link();

        //  static const char * tf_varyings[] =
        //         {
        //             "tf_position_mass",
        //             "tf_velocity"
        //         };
        // glTransformFeedbackVaryings(m_update_program, 2, tf_varyings, GL_SEPARATE_ATTRIBS);

        self.render_program = ShaderProgram::new();
        self.render_program
            .vertex_shader("assets/shaders/spring-mass/render.vs.glsl")
            .fragment_shader("assets/shaders/spring-mass/render.fs.glsl")
            .link();
    }
}

impl App for DemoApp {
    fn on_key(&mut self, keycode: &VirtualKeyCode, keystate: &ElementState) -> Result<()> {
        // match (keycode, keystate) {
        //     (VirtualKeyCode::T, ElementState::Pressed) => {}
        //     (VirtualKeyCode::R, ElementState::Pressed) => {}
        //     _ => (),
        // }

        //  switch (key)
        //  {
        //      case 'R': load_shaders();
        //          break;
        //      case 'L': draw_lines = !draw_lines;
        //          break;
        //      case 'P': draw_points = !draw_points;
        //          break;
        //      case GLFW_KEY_KP_ADD: iterations_per_frame++;
        //          break;
        //      case GLFW_KEY_KP_SUBTRACT: iterations_per_frame--;
        //          break;
        //  }

        Ok(())
    }

    fn initialize(&mut self, _window: &Window) -> Result<()> {
        self.draw_points = true;
        self.draw_lines = true;
        self.iterations_per_frame = 16;

        self.load_shaders();

        //   vmath::vec4 * initial_positions = new vmath::vec4 [POINTS_TOTAL];
        //         vmath::vec3 * initial_velocities = new vmath::vec3 [POINTS_TOTAL];
        //         vmath::ivec4 * connection_vectors = new vmath::ivec4 [POINTS_TOTAL];

        //         int n = 0;

        //         for (j = 0; j < POINTS_Y; j++) {
        //             float fj = (float)j / (float)POINTS_Y;
        //             for (i = 0; i < POINTS_X; i++) {
        //                 float fi = (float)i / (float)POINTS_X;

        //                 initial_positions[n] = vmath::vec4((fi - 0.5f) * (float)POINTS_X,
        //                                                    (fj - 0.5f) * (float)POINTS_Y,
        //                                                    0.6f * sinf(fi) * cosf(fj),
        //                                                    1.0f);
        //                 initial_velocities[n] = vmath::vec3(0.0f);

        //                 connection_vectors[n] = vmath::ivec4(-1);

        //                 if (j != (POINTS_Y - 1))
        //                 {
        //                     if (i != 0)
        //                         connection_vectors[n][0] = n - 1;

        //                     if (j != 0)
        //                         connection_vectors[n][1] = n - POINTS_X;

        //                     if (i != (POINTS_X - 1))
        //                         connection_vectors[n][2] = n + 1;

        //                     if (j != (POINTS_Y - 1))
        //                         connection_vectors[n][3] = n + POINTS_X;
        //                 }
        //                 n++;
        //             }
        //         }

        //         glGenVertexArrays(2, m_vao);
        //         glGenBuffers(5, m_vbo);

        //         for (i = 0; i < 2; i++) {
        //             glBindVertexArray(m_vao[i]);

        //             glBindBuffer(GL_ARRAY_BUFFER, m_vbo[POSITION_A + i]);
        //             glBufferData(GL_ARRAY_BUFFER, POINTS_TOTAL * sizeof(vmath::vec4), initial_positions, GL_DYNAMIC_COPY);
        //             glVertexAttribPointer(0, 4, GL_FLOAT, GL_FALSE, 0, NULL);
        //             glEnableVertexAttribArray(0);

        //             glBindBuffer(GL_ARRAY_BUFFER, m_vbo[VELOCITY_A + i]);
        //             glBufferData(GL_ARRAY_BUFFER, POINTS_TOTAL * sizeof(vmath::vec3), initial_velocities, GL_DYNAMIC_COPY);
        //             glVertexAttribPointer(1, 3, GL_FLOAT, GL_FALSE, 0, NULL);
        //             glEnableVertexAttribArray(1);

        //             glBindBuffer(GL_ARRAY_BUFFER, m_vbo[CONNECTION]);
        //             glBufferData(GL_ARRAY_BUFFER, POINTS_TOTAL * sizeof(vmath::ivec4), connection_vectors, GL_STATIC_DRAW);
        //             glVertexAttribIPointer(2, 4, GL_INT, 0, NULL);
        //             glEnableVertexAttribArray(2);
        //         }

        //         delete [] connection_vectors;
        //         delete [] initial_velocities;
        //         delete [] initial_positions;

        //         glGenTextures(2, m_pos_tbo);
        //         glBindTexture(GL_TEXTURE_BUFFER, m_pos_tbo[0]);
        //         glTexBuffer(GL_TEXTURE_BUFFER, GL_RGBA32F, m_vbo[POSITION_A]);
        //         glBindTexture(GL_TEXTURE_BUFFER, m_pos_tbo[1]);
        //         glTexBuffer(GL_TEXTURE_BUFFER, GL_RGBA32F, m_vbo[POSITION_B]);

        //         int lines = (POINTS_X - 1) * POINTS_Y + (POINTS_Y - 1) * POINTS_X;

        //         glGenBuffers(1, &m_index_buffer);
        //         glBindBuffer(GL_ELEMENT_ARRAY_BUFFER, m_index_buffer);
        //         glBufferData(GL_ELEMENT_ARRAY_BUFFER, lines * 2 * sizeof(int), NULL, GL_STATIC_DRAW);

        //         int * e = (int *)glMapBufferRange(GL_ELEMENT_ARRAY_BUFFER, 0, lines * 2 * sizeof(int), GL_MAP_WRITE_BIT | GL_MAP_INVALIDATE_BUFFER_BIT);

        //         for (j = 0; j < POINTS_Y; j++)
        //         {
        //             for (i = 0; i < POINTS_X - 1; i++)
        //             {
        //                 *e++ = i + j * POINTS_X;
        //                 *e++ = 1 + i + j * POINTS_X;
        //             }
        //         }

        //         for (i = 0; i < POINTS_X; i++)
        //         {
        //             for (j = 0; j < POINTS_Y - 1; j++)
        //             {
        //                 *e++ = i + j * POINTS_X;
        //                 *e++ = POINTS_X + i + j * POINTS_X;
        //             }
        //         }

        //         glUnmapBuffer(GL_ELEMENT_ARRAY_BUFFER);

        Ok(())
    }

    fn render(&mut self, time: f32) -> Result<()> {
        //  int i;
        //         glUseProgram(m_update_program);

        //         glEnable(GL_RASTERIZER_DISCARD);

        //         for (i = iterations_per_frame; i != 0; --i)
        //         {
        //             glBindVertexArray(m_vao[m_iteration_index & 1]);
        //             glBindTexture(GL_TEXTURE_BUFFER, m_pos_tbo[m_iteration_index & 1]);
        //             m_iteration_index++;
        //             glBindBufferBase(GL_TRANSFORM_FEEDBACK_BUFFER, 0, m_vbo[POSITION_A + (m_iteration_index & 1)]);
        //             glBindBufferBase(GL_TRANSFORM_FEEDBACK_BUFFER, 1, m_vbo[VELOCITY_A + (m_iteration_index & 1)]);
        //             glBeginTransformFeedback(GL_POINTS);
        //             glDrawArrays(GL_POINTS, 0, POINTS_TOTAL);
        //             glEndTransformFeedback();
        //         }

        //         glDisable(GL_RASTERIZER_DISCARD);

        //         static const GLfloat black[] = { 0.0f, 0.0f, 0.0f, 0.0f };

        //         glViewport(0, 0, info.windowWidth, info.windowHeight);
        //         glClearBufferfv(GL_COLOR, 0, black);

        //         glUseProgram(m_render_program);

        //         if (draw_points)
        //         {
        //             glPointSize(4.0f);
        //             glDrawArrays(GL_POINTS, 0, POINTS_TOTAL);
        //         }

        //         if (draw_lines)
        //         {
        //             glBindBuffer(GL_ELEMENT_ARRAY_BUFFER, m_index_buffer);
        //             glDrawElements(GL_LINES, CONNECTIONS_TOTAL * 2, GL_UNSIGNED_INT, NULL);
        //         }

        Ok(())
    }
}

fn main() -> Result<()> {
    let app = DemoApp::default();
    run_application(app, "Spring-Mass Simulator")
}
