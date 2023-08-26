#version 410 core

uniform float u_x_offset;

in vec3 i_pos;
in vec3 i_color;

out vec3 v_color;

void main()
{
    gl_Position = vec4(i_pos.x + u_x_offset, i_pos.y, i_pos.z, 1.0);
    v_color = i_color;
}