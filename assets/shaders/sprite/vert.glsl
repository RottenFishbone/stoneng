#version 430 core
layout (location = 0) in vec3 pos;
layout (location = 1) in vec4 color;
layout (location = 2) in vec2 scale;
layout (location = 3) in float rotation;
layout (location = 4) in uint sprite_data;
// Sprite data must be packed as:
// [ 0x00 0x00 0x00 0x00 ]
//   |--| |--| |-------|
//  flags dims sprite_id
// Where:
//  id     - The location of the idle sprite. This is determined from 
//           right to left, from the top-left corner = 0:
//              0 1 2 3
//              4 5 6 7
//              ...
//
//  dims  - How many tiles wide and tall the sprite is with the right half
//          of the byte being x and the left being y. Zero defaults to 1x1.
//
//  flags  - Flags applying directly to this sprite

uniform mat4 view_projection;
uniform int sheet_width;
uniform int sheet_tile_w;


out VS_OUT {
    vec4 color;
    vec2 scale;
    uint id;
    vec2 dims;
} vs_out;

void main() {
    // Unpack sprite data
    vs_out.id   = sprite_data & 0xFF;
    vs_out.dims = vec2(float((sprite_data >> 24) & 0xF) + 1.0,
                       float((sprite_data >> 28) & 0xF) + 1.0);
    
    // Forward attributes to geometry shader
    vs_out.scale = scale; 
    vs_out.color = color;

    gl_Position = vec4(pos, 1.0);
    // TODO rotation
}
