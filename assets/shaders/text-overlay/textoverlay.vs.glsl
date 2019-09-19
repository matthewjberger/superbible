#version 440 core

void main(void)
{
  gl_Position = vec4(float((gl_VertexID >> 1) & 1) * 2.0 - 1.0, float((gl_VertexID & 1)) * 2.0 - 1.0, 0.0, 1.0);
}

