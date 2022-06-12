/**
 * bitmap font geometry shader
 * Jayden Dumouchel 2022
 * 
 * Builds a quad and relevant texture locations for a glyph using an ascii value
 * as an offset identifier. This is to simplify the construction and communication
 * of characters for the (device) host.
 */
#version 410 core
layout (points) in;
layout (triangle_strip, max_vertices = 4) out;

uniform mat4 projection;
uniform mat4 view;
uniform float glyph_size;
uniform float atlas_width;

out GS_OUT {
    vec2 tex_coord;
    vec4 tint;
} gs_out;

in vec4 color[]; 
in float size[];
in int ascii_val[];

void main() {
    // Offsets to draw a unit quad around a point
    vec4 quad_verts[4] = vec4[4](
        vec4( -1.0, -1.0, 0.0, 0.0 ), // bottom-left
        vec4( -1.0,  1.0, 0.0, 0.0 ), // top-left
        vec4(  1.0, -1.0, 0.0, 0.0 ), // bottom-right
        vec4(  1.0,  1.0, 0.0, 0.0 )  // top-right
    );


    // Calculate uv_coords using (0, 1) as top left.
    // The glyph's sub-texture is 'scale' units wide, thus, the bottom vert is 
    // calculated as (1.0 - scale) for the top-left tile's bottom
    //
    // atlas_scale = glyphs size as ratio of atlas width
    // given a 2x2 16px atlas with 8px glyphs, 
    // the width would be 1/2 or glyph_size/atlas_width.
    //  ___
    // |_|_| ]-> atlas_scale  
    // |_|_|
    float atlas_scale = glyph_size / atlas_width;
    // Inverted unit UV square
    vec2 unit_uv_verts[4] = vec2[4](
        vec2( 0.0,          0.0 + atlas_scale ),// bottom-left
        vec2( 0.0,          0.0               ),// top-left
        vec2( atlas_scale,  0.0 + atlas_scale ),// bottom-right
        vec2( atlas_scale,  0.0               ) // top-right
    );
 
    // Calculate the 2d index of the character in the atlas
    int glyphs_per_row = int(atlas_width / glyph_size);
    vec2 glyph_id_coord = vec2( 
             float(ascii_val[0] % glyphs_per_row),
             float(ascii_val[0] / glyphs_per_row)
        );
    vec2 uv_offset = glyph_id_coord * atlas_scale;
    for (int i = 0; i < 4; ++i){
        // Scale the unit quad to size and offset the point by the corresponding
        // unit vertex position
        vec4 scaled_vert_offset = quad_verts[i] * (glyph_size/2.0) * size[0];
        gl_Position = projection * (gl_in[0].gl_Position + scaled_vert_offset);
        
        gs_out.tex_coord = unit_uv_verts[i] + uv_offset;
        gs_out.tint = color[0];
        EmitVertex();
    }

    EndPrimitive();
}
