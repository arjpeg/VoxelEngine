#version 410 core

uniform mat4 view;
uniform mat4 projection;
uniform mat4 model;

layout (location = 0) in vec3 i_pos;
layout (location = 1) in vec3 i_normal;

out vec3 normal;
out vec3 fragPos;

void main()
{
    gl_Position = projection * view * model * vec4(i_pos, 1.0);

    // give the normal to the fragment shader
    normal = i_normal;
    // because there is no model matrix, the fragPos is the same as i_pos
    fragPos = i_pos;
}