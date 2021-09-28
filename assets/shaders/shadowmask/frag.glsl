#version 460 core

out vec4 fragColor;

in vec2 uv_pos;

uniform sampler2D lightmap;

void main() {

    int dither[8][8] = {
        { 0, 32,  8, 40,  2, 34, 10, 42}, 
        {48, 16, 56, 24, 50, 18, 58, 26}, 
        {12, 44,  4, 36, 14, 46,  6, 38}, 
        {60, 28, 52, 20, 62, 30, 54, 22}, 
        { 3, 35, 11, 43,  1, 33,  9, 41}, 
        {51, 19, 59, 27, 49, 17, 57, 25},
        {15, 47,  7, 39, 13, 45,  5, 37},
        {63, 31, 55, 23, 61, 29, 53, 21} 
    };
    float scale = 2.0;
    vec2 xy = gl_FragCoord.xy / scale;
    vec2 xy_adj = mod(gl_FragCoord.xy, scale);
    int x = int(mod(xy.x, 4.0)); 
    int y = int(mod(xy.y, 4.0)); 
    // Grab the R channel from the lightmap
    float light = texture(lightmap, uv_pos).r;
    ivec2 offs = ivec2(-int(xy_adj.x), -int(xy_adj.y));
    light = textureOffset(lightmap, uv_pos, offs).r;
    
    float limit = float(dither[x][y]+1) / 64.0;
    if (light > limit)
        discard;

    fragColor = vec4(0.1, 0.1, 0.15, 0.975);
}
