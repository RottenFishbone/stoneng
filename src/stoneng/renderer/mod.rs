#![allow(dead_code)]
use crate::sprite::{self, Sprite};
use crate::EngineError;
use crate::shader;

use std::{
    path::Path,
    mem::size_of,
};
use glm::{Vec2, Vec3, Vec4};
use gl::types::*;

pub struct Renderer {
    initialized:    bool,

    pub sprites:    Vec<Sprite>,

    // OpenGL members
    program:    GLuint,
    vert_arr:   GLuint,
    array_buf:  GLuint,
    texture:    GLuint,
    uniform_locs: [GLint; 3], 

    frame_count: usize,
}

impl Renderer {
    pub fn new() -> Self {
        Self {
            initialized:    false,
            sprites:    Vec::with_capacity(1024),

            program:    0,
            vert_arr:   0,
            array_buf:  0,
            texture:    0,
            uniform_locs: [0, 0, 0], 

            frame_count: 0, 
        }
    }
    
    /// Initializes the Renderer's OpenGL objects.
    ///
    /// This is called by the engine before the game is initialized.
    /// Do not call this function before the engine is initialized or the gl
    /// bindings will not yet be loaded.
    pub fn init(&mut self) -> Result<(), EngineError>{
        use stb::image::LoadResult;
        
        // Just no op on a second call.
        if self.initialized { return Ok(()); }

        // Prevent running this function too early.
        if !gl::Viewport::is_loaded() {
            let msg = format!("{}\n{}",
                "Renderer::init called before gl bindings were loaded.",
                "init() should only be called by the engine."
            );
            return Err(EngineError::RendererInit(msg));
        }

        // Build shader program
        self.program = shader::program_from_sources(
            include_str!("../../../assets/shaders/sprite_vert.glsl").into(),
            include_str!("../../../assets/shaders/sprite_frag.glsl").into(),
            //None,
            Some(include_str!("../../../assets/shaders/sprite_geom.glsl").into())
        ).unwrap();
        
        // Load the image file into memory and format using stb_image
        let atlas_bytes = include_bytes!("../../../assets/textures/atlas.png");
        let atlas_img = match stb::image::load_from_memory(atlas_bytes){
            // Accept unsigned byte formatted image
            LoadResult::ImageU8(img) => img,
            // Error on any other result
            _ => {
                let msg = format!("{}\n{}",
                        "Failed to load texture atlas.",
                        "Ensure the file is located at assets/texutes/atlas.png"
                    );  
                return Err(EngineError::RendererInit(msg));
            },
        };
        
        unsafe {
            gl::UseProgram(self.program);

            // Generate OpenGL objects/buffers
            gl::GenVertexArrays(1, &mut self.vert_arr as *mut GLuint);
            gl::GenBuffers(1, &mut self.array_buf as *mut GLuint);
            gl::GenTextures(1, &mut self.texture as *mut GLuint);
            
            // Binding
            gl::BindVertexArray(self.vert_arr);
            gl::BindBuffer(gl::ARRAY_BUFFER, self.array_buf);
            gl::BindTexture(gl::TEXTURE_2D, self.texture);

            // Setup texture parameters ( Nearest for pixel-perfect scaling ) 
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);

            // Load the texture to the GPU
            gl::TexImage2D(
                gl::TEXTURE_2D, 0, gl::RGBA as i32, 
                atlas_img.width as i32, atlas_img.height as i32, 0,
                gl::RGBA, gl::UNSIGNED_BYTE,
                atlas_img.data.as_ptr() as *const GLvoid
            );
            
            // Set up the attribute pointers
            let stride = size_of::<Sprite>() as i32;
                // Position
            gl::EnableVertexAttribArray(0);
            gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, stride, 0 as *const GLvoid); 

                // Color 
            let color_offset = size_of::<Vec3>() as i32;
            gl::EnableVertexAttribArray(1);
            gl::VertexAttribPointer(1, 4, gl::FLOAT, gl::FALSE, stride, 
                                    color_offset as *const GLvoid); 
                // Scale
            let scale_offset = color_offset + (size_of::<Vec4>() as i32);
            gl::EnableVertexAttribArray(2);
            gl::VertexAttribPointer(2, 2, gl::FLOAT, gl::FALSE, stride, 
                                    scale_offset as *const GLvoid); 
                // Rotation    
            let rotation_offset = scale_offset + (size_of::<Vec2>() as i32);
            gl::EnableVertexAttribArray(3);
            gl::VertexAttribPointer(3, 2, gl::FLOAT, gl::FALSE, stride, 
                                    rotation_offset as *const GLvoid); 
                // Sprite Data
            let sdata_offset = rotation_offset + (size_of::<f32>() as i32);
            gl::EnableVertexAttribArray(4);
            gl::VertexAttribIPointer(4, 1, gl::UNSIGNED_INT, stride, 
                                    sdata_offset as *const GLvoid); 

            // Find and store the uniform locations
            self.uniform_locs[0] = shader::get_uniform_location(
                self.program, "view_projection"); 
            self.uniform_locs[1] = shader::get_uniform_location(
                self.program, "sheet_width");           
            self.uniform_locs[2] = shader::get_uniform_location(
                self.program, "sheet_tile_w");           
 
            // Unbind
            gl::BindTexture(gl::TEXTURE_2D, 0);
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindVertexArray(0);
            gl::UseProgram(0);
        }


        self.initialized = true; 
        Ok(())
    }

    pub fn render(&mut self) {
        if !self.initialized { return; }

        unsafe {
            // Bind
            gl::UseProgram(self.program);
            gl::BindVertexArray(self.vert_arr);
            gl::BindBuffer(gl::ARRAY_BUFFER, self.array_buf);
            gl::BindTexture(gl::TEXTURE_2D, self.texture);
            
            // Set uniforms
            let view_projection = glm::ortho(0.0, 800.0, 0.0, 600.0, -1.0, 1.0);
            gl::UniformMatrix4fv(self.uniform_locs[0], 1, gl::FALSE, view_projection.as_ptr());
            gl::Uniform1i(self.uniform_locs[1], 256);
            gl::Uniform1i(self.uniform_locs[2], 32);

            // Transfer sprite data
            gl::BufferData(
                gl::ARRAY_BUFFER, 
                (size_of::<Sprite>() * self.sprites.len()) as GLsizeiptr,
                self.sprites.as_ptr() as *const GLvoid,
                gl::DYNAMIC_DRAW
            );

            gl::DrawArrays(gl::POINTS, 0, self.sprites.len() as i32);
            
            // Unbind
            gl::BindTexture(gl::TEXTURE_2D, 0);
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindVertexArray(0);
            gl::UseProgram(0);
        }
        // Order sprites so that they are sent front to back
        //self.sprites.sort_by(|a,b|  b.partial_cmp(a).unwrap() );
    
        // Advance the frame_count
        if self.frame_count < usize::MAX { self.frame_count += 1; }
        else { self.frame_count = 0; }
    }
}
