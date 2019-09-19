#version 440 core

layout (origin_upper_left) in vec4 gl_FragCoord;
layout (location = 0) out vec4 o_color;
layout (binding = 0) uniform isampler2D text_buffer;
layout (binding = 1) uniform isampler2DArray font_texture;

void main(void)
{
    ivec2 frag_coord = ivec2(gl_FragCoord.xy);
    ivec2 char_size = textureSize(font_texture, 0).xy;
    ivec2 char_location = frag_coord / char_size;
    ivec2 texel_coord = frag_coord % char_size;
    int character = texelFetch(text_buffer, char_location, 0).x;
    float val = texelFetch(font_texture, ivec3(texel_coord, character), 0).x;
    if (val == 0.0)
        discard;
    o_color = vec4(1.0);
}

