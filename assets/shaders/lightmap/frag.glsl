#version 410 core

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

    float lum = max(0.0, 1.0-(dist_sq/gs_out.intensity_sq));
    lum *= lum*0.75;

    fragColor = vec4(clamp(lum * 1.1, 0.0, 1.0), 0.0, 0.0, 1.0);
}
