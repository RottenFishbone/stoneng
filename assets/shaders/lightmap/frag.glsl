#version 460 core

out vec4 fragColor;

in GS_OUT {
    float intensity_sq;
    vec2 center;
} gs_out;



void main() {
    vec2 xy = gl_FragCoord.xy; 

    vec2 delta_pos = xy-gs_out.center;
    float dist_sq = dot(delta_pos, delta_pos);
    // Round the corners from the quad
    if (dist_sq >= gs_out.intensity_sq)
        discard;

    float lum = 0.0;
    float dist_ratio = dist_sq / gs_out.intensity_sq * 1.45;
    // Distance from inner glow
    if (dist_ratio < 0.1) {
        lum = 1.0 - dist_ratio/3.0;
    }
    else{
        lum = 1.0-(dist_ratio);
    }

    fragColor = vec4(lum, 0.0, 0.0, 0.0);
}
