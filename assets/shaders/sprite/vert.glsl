#version 410 core
layout (location = 0) in vec3 pos;
// pos      - the world position of the sprite to be drawn
//            z-values will not change the scale, only the draw order
layout (location = 1) in vec2 scale;
// scale    - multiplicitively resizes the x,y components of the sprite
layout (location = 2) in float rotation;
// TODO     -- implement
layout (location = 3) in vec4 color;
// color    - applies an additive tint to the sprite (rgb)
//         alpha is applied as the final alpha of the sprite
layout (location = 4) in uint sprite_id;
// sprite_id - The location of the idle sprite. This is determined from 
//           right to left, from the top-left corner = 0:
//              0 1 2 3
//              4 5 6 7
//              ...
//
layout (location = 5) in uint sprite_data;
// Sprite data must be packed as:
// [ 0x00 0x00 0x00 0x00 ]
//   |--| |--| |-------|
//  flags dims reserved
// Where:
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
    vs_out.id = sprite_id;
    // Unpack sprite data
    vs_out.dims = vec2(float((sprite_data >> 24) & 0xF) + 1.0,
                       float((sprite_data >> 28) & 0xF) + 1.0);
    
    // Forward attributes to geometry shader
    vs_out.scale = scale; 
    vs_out.color = color;

    gl_Position = vec4(pos, 1.0);
    // TODO rotation
}
