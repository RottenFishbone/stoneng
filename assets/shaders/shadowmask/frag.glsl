#version 410 core

out vec4 fragColor;

in vec2 uv_pos;

uniform sampler2D lightmap;
uniform float lightmap_scale; 

void main() {
    int dither[64] = int[64](
         0, 32,  8, 40,  2, 34, 10, 42, 
        48, 16, 56, 24, 50, 18, 58, 26, 
        12, 44,  4, 36, 14, 46,  6, 38, 
        60, 28, 52, 20, 62, 30, 54, 22, 
         3, 35, 11, 43,  1, 33,  9, 41, 
        51, 19, 59, 27, 49, 17, 57, 25,
        15, 47,  7, 39, 13, 45,  5, 37,
        63, 31, 55, 23, 61, 29, 53, 21 
    );

    vec2 xy = floor(gl_FragCoord.xy) / lightmap_scale;

    int x = int(mod(xy.x, 8.0)); 
    int y = int(mod(xy.y, 8.0));
    
    // Calculate the id as x + y * row_width (8)
    int id = x + (y << 3);
    // Grab the R channel from the lightmap
    float light = texture(lightmap, uv_pos).r;
    
    float limit = float(dither[id]) / 64.0;
    if (light > limit){
        discard;
    }
    fragColor = vec4(0.1, 0.1, 0.15, 0.975);
}
