#![allow(dead_code)]

use crate::EngineError;
use crate::shader;
use crate::ecs::component;

use stb::image::LoadResult;
use std::{
    path::Path,
    mem::size_of,
};
use glm::{Vec2, Vec3, Vec4, Mat4};
use gl::types::*;

/// An individual sprite model directly used for rendering. 
#[repr(C)]
#[derive(Debug, Clone)]
pub struct RenderSprite {
    pub translation:    (f32, f32, f32),
    pub scale:          (f32, f32),
    pub rotation:        f32,   // TODO implement
    pub color:          (f32, f32, f32, f32),
    pub sprite_id:      u32,
    pub sprite_dims:    u8,
    pub sprite_flags:   u8,
    pub reserved:       u16,
}
impl Default for RenderSprite {
    fn default() -> Self {
        Self {
            translation: (0.0, 0.0, 0.0),
            scale:       (1.0, 1.0),
            rotation:    0.0,
            
            color:       (1.0, 1.0, 1.0, 1.0),
            
            sprite_id:    0,
            sprite_dims:  0,
            sprite_flags: 0,
            reserved:     0,
        }
    }
}

/// The SpriteRenderer is used to draw RenderSprites to the screen.
///
/// It operates by loading an atlas image into a texture on the GPU. It later 
/// references the atlas' sprites using the data in a RenderSprite.
///
/// As the renderer naturally relies on OpenGL to operate, it must only be used
/// _after_ the OpenGL bindings have been loaded and only on the main thread.
#[derive(Default, Clone, Copy)]
pub struct SpriteRenderer {
    initialized: bool,

    shader:     GLuint,
    vao:        GLuint,
    abo:        GLuint,
    tex:        GLuint,
    uniform_locations:   [GLint; 4],
}

impl SpriteRenderer {
    /// Creates an empty SpriteRenderer. `init` must be called before use.
    pub fn new() -> Self {
        Self::default()
    }

    /// Loads the atlas and initializes the SpriteRenderer's OpenGL objects.
    /// 
    /// This can _only_ be called after the OpenGL bindings have been loaded.
    pub fn init(&mut self, atlas: &[u8]) -> Result<(), EngineError> {
        if self.initialized { return Ok(()) }

        // Prevent running this function too early.
        if !gl::Viewport::is_loaded() {
            let msg = format!("{}\n{}",
                "SpriteRenderer::init called before gl bindings were loaded.",
                "init() should only be called by the engine."
            );
            return Err(EngineError::RendererInit(msg));
        }

        // Load the image file into memory and format using stb_image
        let atlas_img = match stb::image::load_from_memory(atlas){
            // Accept unsigned byte formatted image
            LoadResult::ImageU8(img) => img,
            // Error on any other result
            _ => {
                let msg = format!("{}\n{}",
                        "Failed to load texture atlas.",
                        "Ensure the atlas is an RGBA PNG."
                    );  
                return Err(EngineError::RendererInit(msg));
            },
        };
 
        // Build shader programs
        self.shader = shader::program_from_sources(
            include_str!("../../../assets/shaders/sprite/vert.glsl").into(),
            include_str!("../../../assets/shaders/sprite/frag.glsl").into(),
            Some(include_str!("../../../assets/shaders/sprite/geom.glsl").into())
        ).unwrap();
 
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
                atlas_img.width as i32, atlas_img.height as i32, 0,
                gl::RGBA, gl::UNSIGNED_BYTE,
                atlas_img.data.as_ptr() as *const GLvoid
            );
            

            // Set up the attribute pointers
            let stride = size_of::<RenderSprite>() as i32;
            // Transform
                // Translation
            gl::BindBuffer(gl::ARRAY_BUFFER, self.abo);
            gl::EnableVertexAttribArray(0);
            gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, stride, 0 as *const GLvoid); 

                // Scale
            let scale_offset = size_of::<f32>() as i32 * 3;
            gl::EnableVertexAttribArray(1);
            gl::VertexAttribPointer(1, 2, gl::FLOAT, gl::FALSE, stride, 
                                    scale_offset as *const GLvoid); 
                // Rotation    
            let rotation_offset = scale_offset + (size_of::<f32>() as i32) * 2;
            gl::EnableVertexAttribArray(2);
            gl::VertexAttribPointer(2, 2, gl::FLOAT, gl::FALSE, stride, 
                                    rotation_offset as *const GLvoid); 
            
            // Color 
            let color_offset = rotation_offset + (size_of::<f32>() as i32);
            gl::EnableVertexAttribArray(3);
            gl::VertexAttribPointer(3, 4, gl::FLOAT, gl::FALSE, stride, 
                                    color_offset as *const GLvoid);  

            // Sprite ID
            let id_offset = color_offset + (size_of::<f32>() as i32) * 4;
            gl::EnableVertexAttribArray(4);
            gl::VertexAttribIPointer(4, 1, gl::UNSIGNED_INT, stride, 
                                     id_offset as *const GLvoid); 
            // Sprite Data
            let data_offset = id_offset + (size_of::<u32>() as i32);
            gl::EnableVertexAttribArray(5);
            gl::VertexAttribIPointer(5, 1, gl::UNSIGNED_INT, stride, 
                                     data_offset as *const GLvoid); 

            // Find and store the uniform locations
            self.uniform_locations[0] = shader::get_uniform_location(
                self.shader, "view_projection"); 
            self.uniform_locations[1] = shader::get_uniform_location(
                self.shader, "sheet_width");                       
            self.uniform_locations[2] = shader::get_uniform_location(
                self.shader, "sheet_tile_w");           

            // Unbinding
            gl::BindTexture(gl::TEXTURE_2D, 0);
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindVertexArray(0);
            gl::UseProgram(0);
        }

        self.initialized = true;
        Ok(())
    }
    
    /// Loads a passed set of RenderSprites to the screen. 
    pub fn render(&self, sprites: &[RenderSprite], window_size: (f32, f32), cam: (f32, f32, f32)){

        if !self.initialized { return; }
        unsafe {
            gl::Enable(gl::BLEND);
            gl::Enable(gl::DEPTH_TEST);

            gl::UseProgram(self.shader);
            gl::BindVertexArray(self.vao);
            gl::BindTexture(gl::TEXTURE_2D, self.tex);
            
            let (winx, winy) = window_size;
            gl::Viewport(0, 0, winx as i32, winy as i32);
            
            // Set uniforms
                // view_projection
            let projection = glm::ortho(0.0, winx, 0.0, winy, -25.0, 25.0);
            let view: Mat4 = glm::translation(&Vec3::new(-cam.0, -cam.1, -cam.2));
            let view_projection = projection * view;
            gl::UniformMatrix4fv(self.uniform_locations[0], 1, gl::FALSE, 
                                 view_projection.as_ptr());
                
                // sheet_width
            gl::Uniform1i(self.uniform_locations[1], 250);
                // sheet_tile_w
            gl::Uniform1i(self.uniform_locations[2], 10);

            // Transfer sprite data
            gl::BindBuffer(gl::ARRAY_BUFFER, self.abo);
            gl::BufferData(
                gl::ARRAY_BUFFER, 
                (size_of::<RenderSprite>() * sprites.len()) as GLsizeiptr,
                sprites.as_ptr() as *const GLvoid,
                gl::DYNAMIC_DRAW
            );
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);

            // Render to main framebuffer
            gl::DrawArrays(gl::POINTS, 0, sprites.len() as i32);
         
            // Unbind
            gl::BindTexture(gl::TEXTURE_2D, 0);
            gl::BindVertexArray(0);
            gl::UseProgram(0);
        }
 
    }
}

