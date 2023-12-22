#version 410 core

uniform vec3 cameraPosition;
uniform vec3 lightPosition;

uniform float time;

in vec3 normal;
in vec3 fragPos;

out vec4 fragColor;

void main() {
    vec3 lightColor = vec3(0.5, 0.8, 1.0);

    // Apply ambient lighting
    float ambientStrength = 0.1;
    vec3 ambient = ambientStrength * lightColor;

    // Apply diffuse lighting
    vec3 norm = normalize(normal);
    vec3 lightDir = normalize(lightPosition - fragPos);
    float diff = max(dot(norm, lightDir), 0.0);
    vec3 diffuse = diff * lightColor;

    // Apply specular lighting
    float specularStrength = 0.5;
    vec3 viewDir = normalize(cameraPosition - fragPos);
    vec3 reflectDir = reflect(-lightDir, norm);

    float spec = pow(max(dot(viewDir, reflectDir), 0.0), 1);
    vec3 specular = specularStrength * spec * lightColor;

    // Apply all lighting techniques
    vec3 result = ambient + diffuse; // + specular;

    fragColor = vec4(result, 1.0);
}
