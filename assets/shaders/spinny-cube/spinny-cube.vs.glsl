#version 450 core

in vec4 position;

out VS_OUT
{
  vec4 color;
} vs_out;

uniform mat4 modelview_matrix;
uniform mat4 projection_matrix;

void main(void)
{
  gl_Position = projection_matrix * modelview_matrix * position;
  vs_out.color = position * 2.0 + vec4(0.5, 0.5, 0.5, 0.0);
}
