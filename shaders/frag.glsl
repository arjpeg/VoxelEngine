#version 410 core

out vec4 fragColor;

in vec3 v_color;
in vec2 v_texCoord;

uniform sampler2D u_texture;

void main() {
    fragColor = vec4(v_color, 1.0) * texture(u_texture, v_texCoord);
}
