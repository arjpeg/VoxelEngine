#version 410 core

in vec2 i_pos;


void main()
{
    gl_Position = vec4(i_pos, 0.0, 1.0);
}