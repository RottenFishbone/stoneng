#version 460 core
layout (location = 0) in vec2 pos;
layout (location = 1) in float intensity;

uniform mat4 view_projection;

out VS_OUT {
    float intensity;
} vs_out;

void main() {
    vs_out.intensity = intensity;
    gl_Position = vec4(pos, 0.1, 1.0);
}
