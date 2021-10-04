#version 410 core
layout (location = 0) in vec2 pos;
layout (location = 1) in vec2 tex_pos;

uniform float lightmap_scale;

out vec2 uv_pos;

void main() {
    uv_pos = tex_pos;///lightmap_scale;
    gl_Position = vec4(pos, 0.0, 1.0);
}
