#version 430 core
layout (points) in;
layout (triangle_strip, max_vertices = 4) out;

uniform mat4 view_projection;
uniform int sheet_width;
uniform int sheet_tile_w;

/// Sprite data from the vertex shader
in VS_OUT {
    vec4 color;
    vec2 scale;
    uint id;
    vec2 dims;
} vs_out[];

/// Fragment data to output
out GS_OUT {
    vec2 tex_coord;
    vec4 color_adj;
} gs_out;

void main() {
    // Aliases
    uint sprite_id = vs_out[0].id;
    float sh_width = float(sheet_width);
    float tile_width = float(sheet_tile_w);
    vec2 dims = vs_out[0].dims;
    vec2 tile_dims = vec2(tile_width, tile_width);

    // Calculate vertex offsets of a square to be drawn with triangle strip.
    // Note this goes from corner to corner, so the origin is centered.
    vec4 unit_quad_verts[4] = {
        vec4( -1.0, -1.0, 0.0, 0.0 ), // bottom-left
        vec4( -1.0,  1.0, 0.0, 0.0 ), // top-left
        vec4(  1.0, -1.0, 0.0, 0.0 ), // bottom-right
        vec4(  1.0,  1.0, 0.0, 0.0 ), // top-right
    };
    
    // Calculate a unit uv square using (0, 1) as top left.
    //
    //  y=1 _ _ _ x,y=1
    //     |_|_|_|
    //     |_|_|_|
    //     |_|_|_|
    // x,y=0      x=1
    //
    // We are looking to find the top-left sub-rect in the map. Each sub-rect's
    // width can be calculated as a ratio of the rect size and width of the sheet. 
    // That is, The width of a sprite, sheet_tile_w, divided by the width of the sheet, sheet_width.
    // This 'sheet_ratio' is used to create a square from the top-left, sheet_ratio wide.
    float sheet_ratio = tile_width / sh_width;
    vec2 unit_uv_verts[4] = {
        vec2( 0.0,         1.0 - sheet_ratio ), // bottom-left
        vec2( 0.0,         1.0               ), // top-left
        vec2( sheet_ratio, 1.0 - sheet_ratio ), // bottom-right
        vec2( sheet_ratio, 1.0               ), // top-right
    };
   
    // Building the quad
    for (int i = 0; i < 4; ++i) {
        // ====== Vertex Calculations ======
        // TODO Rotation
        //
        // Each vertex is calculated used the middle of the quad as origin.
        // This origin is the position of the GL_POINT passed to this shader.
        //
        // The point is using the scale of the sprite tile and a unit quad.
        // This position is then scaled by the size of the sprite in tiles, as 
        // well as the scaling data of the sprite.
        // Finally it's translated so that the anchor of the scaling is along
        // the bottom edge, on the left-most tile.
        // =================================
        // The sprite origin (middle of rect) passed to the shader
        vec4 point_origin = gl_in[0].gl_Position;
        // The offset of this vertex from origin, used to draw the unit quad
        vec4 point_offset = unit_quad_verts[i] * vec4(tile_dims/2.0, 0.0, 0.0);
        // The scale vector for the quad to span the needed sprite dimensions
        //vec4 quad_scale = vec4(vs_out[0].dims, 1.0, 1.0);
        vec4 quad_scale = vec4(dims, 1.0, 1.0) * vec4(vs_out[0].scale, 1.0, 1.0);
        // The translation vector to move it back to a final origin.
        vec4 quad_transl = vec4((dims.x-1.0) * tile_dims.x/2.0,
                                (dims.y-1.0 + vs_out[0].scale.y/2.0) * tile_dims.y/2.0, 
                                0.0, 0.0);
        
        // Finally calculate the vertex position.
        gl_Position = view_projection * (point_origin + point_offset * quad_scale + quad_transl);
        
        // ====== UV calculations ======
        // =============================
        // Calculate the 2d position of the sprite_id 
        uint spr_in_row = sheet_width / sheet_tile_w;
        vec2 uv_id = vec2(float(sprite_id % spr_in_row),
                           float(sprite_id / spr_in_row));
        // Scale that position by the width of the tiles in uv-space
        vec2 uv_offset = uv_id * sheet_ratio;
        // Flip the uv unit square vertically (to flip the texture)
        vec2 flipped_uv = vec2(unit_uv_verts[i].x, 1.0 - unit_uv_verts[i].y);
        gs_out.tex_coord = flipped_uv * dims 
                            + uv_offset        
                            - uv_offset*(dims - vec2(1.0, 1.0));
        gs_out.color_adj = vs_out[0].color;
        
        // Save the vertex
        EmitVertex();
    }

    // Output the quad
    EndPrimitive();
}
