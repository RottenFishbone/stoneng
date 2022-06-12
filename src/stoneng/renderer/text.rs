#![allow(dead_code)]

use crate::EngineError;
use crate::shader;

use stb::image::LoadResult;
use std::{
    path::Path,
    mem::size_of,
};

use glm::{Vec2, Vec3, Vec4};
use gl::types::*;

/// Used to differentiate between drawing to the screen and using 
/// ingame positions. This is important for UI vs ingame text.
pub enum VectorSpace {
    World,
    Screen,
}


/// An individual string to be used to build `RenderChar`s.
#[derive(Debug)]
pub struct RenderString {
    pub translation:    (f32, f32, f32),
    pub size:           f32,
    pub color:          (f32, f32, f32, f32),
    pub text:           String,
}
impl From<String> for RenderString {
    fn from(text: String) -> Self {
        Self {
            translation:    (0.0, 0.0, 0.0),
            size:          1.0,
            color:          (1.0, 1.0, 1.0, 1.0),

            text,
        }
    }
}

/// An individual character with rendering info. Used directly for rendering.
///
/// TODO This system may be a cpu bottleneck for large amounts of text.
/// A system that perhaps only transmits the glyphs and character number
/// as attributes would be more performant
#[repr(C)]
#[derive(Debug)]
pub struct RenderChar {
    pub translation:    (f32, f32, f32),
    // pub offset:      (f32, f32) TODO implement
    pub size:           f32,
    pub color:          (f32, f32, f32, f32),
    pub glyph:          GLbyte,
}
impl RenderChar {
    pub fn new( translation: (f32,f32,f32), 
                size:  f32, 
                color: (f32,f32,f32,f32), 
                glyph: char) -> Self {
        Self { translation, size, color, glyph: glyph as GLbyte }
    }
}

/// The text renderer is used to draw RenderStrings to the screen.
///
/// Strings can be rendered in either world or screen space based on a parameter
/// of `render`.
///
/// As the renderer naturally relies on OpenGL to operate, it must only be used
/// _after_ the OpenGL bindings have been loaded and only on the main thread.
#[derive(Clone, Copy)]
pub struct TextRenderer {
    initialized: bool,

    shader:     GLuint,
    vao:        GLuint,
    abo:        GLuint,
    tex:        GLuint,
    uniform_locations:  [GLint; 4],

    glyph_size: u32,
    atlas_width: u32,
    kerning: f32,
}
impl Default for TextRenderer {
    fn default() -> Self {
        Self {
            initialized: false,
            shader: 0, vao: 0, abo: 0, tex: 0,
            uniform_locations: [0; 4],
            glyph_size: 0,
            atlas_width: 0,
            kerning: -3.0,    
        }
    }
}

impl TextRenderer {
    /// Creates an empty TextRenderer. `init` must be called before use.
    pub fn new() -> Self {
        Self::default()
    }

    /// Initializes OpenGL objects and loads the font texture to the GPU
    ///
    /// This can _only_ be called after the OpenGL bindings have been loaded.
    pub fn init(&mut self, font_img_bytes: &[u8], glyph_size: u32) -> Result<(), EngineError> {
        // Prevent double loading
        if self.initialized { return Ok(()) }

        // Prevent calling too early
        if !gl::Viewport::is_loaded() {
            let msg = format!("{}\n{}",
                "TextRenderer::init called before gl bindings were loaded.",
                "init() should only be called by the engine."
            );
            return Err(EngineError::RendererInit(msg));
        }

        // Load the font image into memory and format it using stb_image
        let font_img = match stb::image::load_from_memory(font_img_bytes){
            LoadResult::ImageU8(img) => img,
            _ => {
                let msg = format!("{}\n{}",
                        "Failed to load the font image.",
                        "Ensure the image is an RGBA PNG."
                    );  
                return Err(EngineError::RendererInit(msg));
            },
        };

        self.shader = shader::program_from_sources(
            include_str!("../../../assets/shaders/text/vert.glsl").into(),
            include_str!("../../../assets/shaders/text/frag.glsl").into(),
            Some(include_str!("../../../assets/shaders/text/geom.glsl").into()),
        ).unwrap();
        
        // Record atlas metadata
        self.glyph_size = glyph_size;
        self.atlas_width = font_img.width as u32;

        unsafe {
            gl::UseProgram(self.shader);
            
            // Generate OpenGL objects/buffers
            gl::GenVertexArrays(1, &mut self.vao as *mut GLuint);
            gl::GenBuffers(1, &mut self.abo as *mut GLuint);
            gl::GenTextures(1, &mut self.tex as *mut GLuint);

            // Binding
            gl::BindVertexArray(self.vao);
            gl::BindTexture(gl::TEXTURE_2D, self.tex);

            // Load the texture to the GPU
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
            gl::TexImage2D(
                gl::TEXTURE_2D, 0, gl::RGBA as i32, 
                font_img.width as i32, font_img.height as i32, 0,
                gl::RGBA, gl::UNSIGNED_BYTE,
                font_img.data.as_ptr() as *const GLvoid
            );
            
            gl::BindBuffer(gl::ARRAY_BUFFER, self.abo);
            
            // Set up the attribute pointers
            let stride = size_of::<RenderChar>() as i32;
               // Translation
            gl::EnableVertexAttribArray(0);
            gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, stride, 0 as *const GLvoid); 

                // Size
            let size_offset = (size_of::<f32>() as i32) * 3;
            gl::EnableVertexAttribArray(1);
            gl::VertexAttribPointer(1, 1, gl::FLOAT, gl::FALSE, stride, 
                                    size_offset as *const GLvoid); 
            
                // Color 
            let color_offset = size_offset + (size_of::<f32>() as i32);
            gl::EnableVertexAttribArray(2);
            gl::VertexAttribPointer(2, 4, gl::FLOAT, gl::FALSE, stride, 
                                    color_offset as *const GLvoid); 

                // Char
            let char_offset = color_offset + (size_of::<f32>() as i32) * 4;
            gl::EnableVertexAttribArray(3);
            gl::VertexAttribIPointer(3, 1, gl::BYTE, stride, char_offset as *const GLvoid);
        

            // Uniform locations
            self.uniform_locations[0] = shader::get_uniform_location(
                self.shader, "projection");
            self.uniform_locations[1] = shader::get_uniform_location(
                self.shader, "view");
            self.uniform_locations[2] = shader::get_uniform_location(
                self.shader, "glyph_size");
            self.uniform_locations[3] = shader::get_uniform_location(
                self.shader, "atlas_width");

            // Unbind states
            gl::BindTexture(gl::TEXTURE_2D, 0);
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindVertexArray(0);
            gl::UseProgram(0);
        }
        
        self.initialized = true;
        Ok(())
    }

    /// Draws a set of RenderStrings to the screen.
    pub fn render(&self, strings: &[RenderString], window_size: (f32, f32), view: (f32, f32, f32)){
        if !self.initialized { return; }
        
        // Build a set of render chars using the passed render strings
        let mut chars: Vec<RenderChar> = Vec::new();
        for string in strings {
            for (i, c) in string.text.chars().enumerate() {
                let (x, y, z) = string.translation;
                chars.push(
                    RenderChar::new(
                        (x + ((self.kerning + self.glyph_size as f32) * string.size * i as f32), y, z),
                        string.size, string.color, c)
                    );
            }
        }

        unsafe {
            // Setup OpenGL state
            gl::Enable(gl::DEPTH_TEST);
            gl::Enable(gl::BLEND);
        
            gl::UseProgram(self.shader);
            gl::BindVertexArray(self.vao);
            gl::BindTexture(gl::TEXTURE_2D, self.tex);
            
            let (winx, winy) = window_size;
            gl::Viewport(0, 0, winx as i32, winy as i32);

            // Uniforms
            let projection = glm::ortho(0.0, winx, 0.0, winy, -1.0, 1.0);
            gl::UniformMatrix4fv(self.uniform_locations[0], 1, gl::FALSE, 
                                 projection.as_ptr());
            
            let view = Vec3::new(view.0, view.1, view.2);
            let view_mat = glm::translation(&view);
            gl::UniformMatrix4fv(self.uniform_locations[1], 1, gl::FALSE,
                                 view_mat.as_ptr());

            gl::Uniform1f(self.uniform_locations[2], self.glyph_size as GLfloat);
            gl::Uniform1f(self.uniform_locations[3], self.atlas_width as GLfloat);

            // Vertex data
            gl::BindBuffer(gl::ARRAY_BUFFER, self.abo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (size_of::<RenderChar>() * chars.len()) as GLsizeiptr,
                chars.as_ptr() as *const GLvoid,
                gl::DYNAMIC_DRAW
            );
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);

            // Render to screen
            gl::DrawArrays(gl::POINTS, 0, chars.len() as i32);

            // Unbind
            gl::BindTexture(gl::TEXTURE_2D, 0);
            gl::BindVertexArray(0);
            gl::UseProgram(0);
        }
    }
}
