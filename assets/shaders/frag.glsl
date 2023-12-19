#version 410 core

in vec3 normal;

out vec4 fragColor;

void main() {
    // Apply ambient lighting
    float ambientStrength = 0.1;
    vec3 lightColor = vec3(0.5, 0.8, 1.0);
    vec3 ambient = ambientStrength * lightColor;

    // vec3 result = ambient * vec3(1.0, 0.5, 0.31);
    vec3 result = normal;

    fragColor = vec4(result, 1.0);
}
