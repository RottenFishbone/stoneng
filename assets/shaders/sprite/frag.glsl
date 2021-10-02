#version 410 core
out vec4 FragColor;

uniform sampler2D spritesheet_tex;

in GS_OUT {
    vec2 tex_coord;
    vec4 color_adj;
} gs_out;

void main() {
    vec4 tex_data = texture(spritesheet_tex, gs_out.tex_coord);
    FragColor = tex_data * gs_out.color_adj;
}

