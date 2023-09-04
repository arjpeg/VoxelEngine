#version 410 core

uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;

layout (location = 0)in vec3 i_pos;
layout (location = 1) in vec3 i_color;

out vec3 v_color;

void main()
{
    // gl_Position = vec4(i_pos, 1.0);
    gl_Position = projection * view * model * vec4(i_pos, 1.0);

    v_color = i_color;
}