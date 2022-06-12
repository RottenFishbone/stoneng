/**
 * bitmap font vertex shader
 * Jayden Dumouchel 2022
 *
 * Passes along vertex attribute info to the geometry shader 
 * to build a usable quad.
 *
 * Ascii values passed here are modified to exclude the 
 * first 32 (non-renderable) characters.
 */
#version 420 core
layout (location = 0) in vec3 translation;
layout (location = 1) in float size_in;
layout (location = 2) in vec4 color_in;
layout (location = 3) in int ascii_val_in;

out vec4 color;
out float size;
out int ascii_val;

void main() {
    gl_Position = vec4(translation, 1.0);

    ascii_val = ascii_val_in - 32;
    color = color_in;
    size = size_in; 
}
