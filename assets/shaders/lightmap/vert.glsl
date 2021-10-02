#version 410 core
layout (location = 0) in vec2 pos;
layout (location = 1) in float intensity;

uniform mat4 view_projection;
uniform float px_scale;

out VS_OUT {
    float intensity;
} vs_out;

void main() {
    vs_out.intensity = intensity/px_scale;
    gl_Position = vec4(pos/px_scale, 0.1, 1.0);
}
