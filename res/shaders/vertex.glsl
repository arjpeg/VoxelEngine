#version 410 core

uniform mat4 u_matrix;

layout (location = 0)in vec3 i_pos;
layout (location = 1) in vec3 i_color;
layout (location = 2) in vec2 i_texCoord;

out vec3 v_color;
out vec2 v_texCoord;

void main()
{
    gl_Position = u_matrix * vec4(i_pos, 1.0);

    v_color = i_color;
    v_texCoord = i_texCoord;
}