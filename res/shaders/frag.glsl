#version 410 core

out vec4 fragColor;

in vec3 v_color;
in vec2 v_texCoord;

uniform sampler2D u_texture;
uniform sampler2D u_texture2;


void main() {
    vec4 texColor = texture(u_texture, v_texCoord);
    vec4 texColor2 = texture(u_texture2, v_texCoord);

    fragColor = mix(texColor, texColor2, 0.2) * vec4(v_color, 1.0);
}
