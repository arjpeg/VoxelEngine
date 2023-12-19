#version 410 core

uniform mat4 view;
uniform mat4 projection;

layout (location = 0) in vec3 i_pos;
layout (location = 1) in vec3 i_normal;

out vec3 normal;

void main()
{
    gl_Position = projection * view * vec4(i_pos, 1.0);
    normal = i_normal;
}