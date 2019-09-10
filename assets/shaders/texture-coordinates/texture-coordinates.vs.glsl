#version 420 core

uniform mat4 modelview_matrix;
uniform mat4 projection_matrix;

layout (location = 0) in vec4 position;
layout (location = 4) in vec2 tc;

out VS_OUT
{
  vec2 tc;
} vs_out;

void main(void)
{
  vec4 pos_vs = modelview_matrix * position;

  vs_out.tc = tc;

  gl_Position = projection_matrix * pos_vs;
}
