use support::app::*;
use support::shader::*;

static VERTEX_SHADER_SOURCE: &str = "
#version 450 core

void main(void)
{
    const vec4 vertices[] = vec4[](vec4( 0.25, -0.25, 0.5, 1.0),
                                   vec4(-0.25, -0.25, 0.5, 1.0),
                                   vec4( 0.25,  0.25, 0.5, 1.0));

    gl_Position = vertices[gl_VertexID];
}
";

static TESSELLATION_CONTROL_SHADER_SOURCE: &str = "
#version 450 core

layout (vertices = 3) out;

void main(void)
{
    if (gl_InvocationID == 0)
    {
        gl_TessLevelInner[0] = 5.0;
        gl_TessLevelOuter[0] = 5.0;
        gl_TessLevelOuter[1] = 5.0;
        gl_TessLevelOuter[2] = 5.0;
    }
    gl_out[gl_InvocationID].gl_Position = gl_in[gl_InvocationID].gl_Position;
}
";

static TESSELLATION_EVALUATION_SHADER_SOURCE: &str = "
#version 450 core

layout (triangles, equal_spacing, cw) in;

void main(void)
{
    gl_Position = (gl_TessCoord.x * gl_in[0].gl_Position) +
                  (gl_TessCoord.y * gl_in[1].gl_Position) +
                  (gl_TessCoord.z * gl_in[2].gl_Position);
}
";

static GEOMETRY_SHADER_SOURCE: &str = "
#version 450 core

layout (triangles) in;
layout (points, max_vertices = 3) out;

void main(void)
{
    int i;
    for (i = 0; i < gl_in.length(); i++)
    {
        gl_Position = gl_in[i].gl_Position;
        EmitVertex();
    }
}
";

static FRAGMENT_SHADER_SOURCE: &str = "
#version 450 core

out vec4 color;

void main(void)
{
    color = vec4(0.0, 0.8, 1.0, 1.0);
}
";

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
                title: "Geometry Shader".to_string(),
                ..Default::default()
            },
            ..Default::default()
        }
    }

    fn load_shaders(&mut self) {
        let mut vertex_shader = Shader::new(ShaderType::Vertex);
        vertex_shader.load(VERTEX_SHADER_SOURCE);

        let mut tessellation_control_shader = Shader::new(ShaderType::TessellationControl);
        tessellation_control_shader.load(TESSELLATION_CONTROL_SHADER_SOURCE);

        let mut tessellation_evaluation_shader = Shader::new(ShaderType::TessellationEvaluation);
        tessellation_evaluation_shader.load(TESSELLATION_EVALUATION_SHADER_SOURCE);

        let mut geometry_shader = Shader::new(ShaderType::Geometry);
        geometry_shader.load(GEOMETRY_SHADER_SOURCE);

        let mut fragment_shader = Shader::new(ShaderType::Fragment);
        fragment_shader.load(FRAGMENT_SHADER_SOURCE);

        self.shader_program = ShaderProgram::new();
        self.shader_program
            .attach(vertex_shader)
            .attach(tessellation_control_shader)
            .attach(tessellation_evaluation_shader)
            .attach(geometry_shader)
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
        unsafe {
            gl::CreateVertexArrays(1, &mut self.vao);
            gl::BindVertexArray(self.vao);
            gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);
            gl::PointSize(5.0);
        }
    }

    fn render(&mut self, _: f32) {
        let background_color: [GLfloat; 4] = [0.0, 0.0, 0.0, 1.0];
        self.shader_program.activate();
        unsafe {
            gl::ClearBufferfv(gl::COLOR, 0, &background_color as *const f32);
            gl::DrawArrays(gl::PATCHES, 0, 3);
        }
    }
}

fn main() {
    run(&mut DemoApp::new());
}
