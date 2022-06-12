/**
 * bitmap font fragment shader
 * Jayden Dumouchel 2022
 * 
 * A simple fragment shader to read the UV coords from the geometry shader
 * and output the (tinted) texture.
 */
#version 410 core
out vec4 FragColor;

uniform sampler2D tex_bitmap_font;

in GS_OUT {
    vec2 tex_coord;
    vec4 tint;
} gs_out;

void main() {
    // Build the frag color as the (texture + tint) * alpha
    FragColor = (texture(tex_bitmap_font, gs_out.tex_coord)
        + vec4(gs_out.tint.rgb, 0.0))
        * vec4(1.0, 1.0, 1.0, gs_out.tint.a);
}
