#version 410 core

uniform float u_x_offset;


layout (location = 0)in vec3 i_pos;
layout (location = 1) in vec3 i_color;
layout (location = 2) in vec2 i_texCoord;

out vec3 v_color;
out vec2 v_texCoord;

void main()
{
    gl_Position = vec4(i_pos.xy, 0.0, 1.0);

    v_color = i_color;
    v_texCoord = i_texCoord;
}