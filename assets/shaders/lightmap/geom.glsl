#version 410 core
layout (points) in;
layout (triangle_strip, max_vertices = 4) out;

uniform mat4 view;
uniform mat4 projection;
uniform float px_scale;
in VS_OUT {
    float intensity;
} vs_out[];

out GS_OUT {
    float intensity_sq;
    vec2 center;
} gs_out;

void main() {
    float intensity = vs_out[0].intensity;
    
    // Calculate vertex offsets of a square to be drawn with triangle strip.
    // Note this goes from corner to corner, so the origin is centered.
    vec4 unit_quad_verts[4] = vec4[4](
        vec4( -1.0, -1.0, 0.0, 0.0 ), // bottom-left
        vec4( -1.0,  1.0, 0.0, 0.0 ), // top-left
        vec4(  1.0, -1.0, 0.0, 0.0 ), // bottom-right
        vec4(  1.0,  1.0, 0.0, 0.0 )  // top-right
    );

    float int_sq = vs_out[0].intensity * vs_out[0].intensity;
    vec4 view_offset = vec4(view[3].xy, 0.0, 0.0) / px_scale;
    for (int i = 0; i < 4; ++i) {
        gl_Position = projection * (gl_in[0].gl_Position + view_offset + unit_quad_verts[i] * intensity);
        gl_Position.w = 1.0;
        gs_out.intensity_sq = int_sq;
        gs_out.center = view_offset.xy + gl_in[0].gl_Position.xy;
        EmitVertex();
    }
    EndPrimitive();
}
