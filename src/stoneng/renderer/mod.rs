#![allow(dead_code)]
use crate::EngineError;
use crate::shader;
use crate::ecs::component;

use stb::image::LoadResult;
use std::{
    path::Path,
    mem::size_of,
};
use glm::{Vec2, Vec3, Vec4};
use gl::types::*;

#[repr(C)]
#[derive(Debug)]
pub struct RenderSprite {
    pub translation:    (f32, f32, f32),
    pub scale:          (f32, f32),
    pub rotation:        f32,
    pub color:          (f32, f32, f32, f32),
    pub sprite_id:      u16,
    pub sprite_dims:    u8,
    pub sprite_flags:   u8,
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
        }
    }
}

#[derive(Default, Clone, Copy)]
pub struct SpriteRenderer {
    initialized: bool,

    shader:     GLuint,
    vao:        GLuint,
    abo:        GLuint,
    tex:        GLuint,
    uniform_locations:   [GLint; 3],
}

impl SpriteRenderer {
    pub fn new() -> Self {
        Self::default()
    }

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
                        "Ensure the atlas is a RGBA PNG."
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

            // Sprite Data
            let data_offset = color_offset + (size_of::<f32>() as i32) * 4;
            gl::EnableVertexAttribArray(4);
            gl::VertexAttribIPointer(4, 1, gl::UNSIGNED_INT, stride, 
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

    pub fn render(&self, sprites: &[RenderSprite], window_size: (f32, f32)){

        if !self.initialized { return; }
        unsafe {
            gl::Enable(gl::DEPTH_TEST);
            gl::Enable(gl::BLEND);
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
            
            gl::UseProgram(self.shader);
            gl::BindVertexArray(self.vao);
            gl::BindTexture(gl::TEXTURE_2D, self.tex);
            
            // Set uniforms
            let (winx, winy) = window_size;
            gl::Viewport(0, 0, winx as i32, winy as i32);
            // view_projection
            let view_projection = glm::ortho(0.0, winx, 0.0, winy, -1.0, 1.0);
            gl::UniformMatrix4fv(self.uniform_locations[0], 1, gl::FALSE, 
                                 view_projection.as_ptr());
            
            // sheet_width
            gl::Uniform1i(self.uniform_locations[1], 256);
            // sheet_tile_w
            gl::Uniform1i(self.uniform_locations[2], 32);

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
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindVertexArray(0);
        }
 
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct RenderLight {
    pub pos: (f32, f32),
    pub intensity: f32,
}

#[derive(Default, Debug)]
pub struct LightRenderer {
    initialized:    bool,
    pub dithered:       bool, // TODO implement disabling dithering
    pub dither_scale:   f32,
    
    fbo:        GLuint,
    shaders:    [GLuint; 2],
    vaos:       [GLuint; 2],
    ebo:        GLuint,
    abos:       [GLuint; 2],
    tex:        GLuint,
    uniform_locations:   [GLint; 3],
}
impl LightRenderer {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn init(&mut self) -> Result<(), EngineError> {
        if self.initialized { return Ok(()) }

        // Prevent running this function too early.
        if !gl::Viewport::is_loaded() {
            let msg = format!("{}\n{}",
                "LightRenderer::init called before gl bindings were loaded.",
                "init() should only be called by the engine."
            );
            return Err(EngineError::RendererInit(msg));
        }
 
        // Build shader programs
        self.shaders[0] = shader::program_from_sources(
            include_str!("../../../assets/shaders/lightmap/vert.glsl").into(),
            include_str!("../../../assets/shaders/lightmap/frag.glsl").into(),
            Some(include_str!("../../../assets/shaders/lightmap/geom.glsl").into())
        ).unwrap();
        self.shaders[1] = shader::program_from_sources(
            include_str!("../../../assets/shaders/shadowmask/vert.glsl").into(),
            include_str!("../../../assets/shaders/shadowmask/frag.glsl").into(),
            None,
        ).unwrap();  
        
        unsafe {
            // Generate OpenGL objects/buffers
            gl::GenFramebuffers(1, &mut self.fbo as *mut GLuint);
            gl::GenVertexArrays(2, &mut self.vaos as *mut GLuint);
            gl::GenBuffers(2, &mut self.abos as *mut GLuint);
            gl::GenTextures(1, &mut self.tex as *mut GLuint);
            

            // ===================== Lightmap =======================
            gl::UseProgram(self.shaders[0]);
            gl::BindVertexArray(self.vaos[0]);
            gl::BindFramebuffer(gl::FRAMEBUFFER, self.fbo); 
            gl::BindTexture(gl::TEXTURE_2D, self.tex);

            // Framebuffer
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
            gl::TexImage2D(
                gl::TEXTURE_2D, 0, gl::RGB as i32, 
                800, 600, 0, 
                gl::RGB, gl::UNSIGNED_BYTE, 
                std::ptr::null()
            );
            gl::FramebufferTexture(gl::FRAMEBUFFER, gl::COLOR_ATTACHMENT0, self.tex, 0); 
            gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
            gl::BindTexture(gl::TEXTURE_2D, 0);
    
            // Attribute Pointers
            // 2d Pos
            gl::BindBuffer(gl::ARRAY_BUFFER, self.abos[0]);
            let stride = (size_of::<GLfloat>() * 3) as i32;
            gl::EnableVertexAttribArray(0);
            gl::VertexAttribPointer(0, 2, gl::FLOAT, gl::FALSE, stride, 0 as *const GLvoid);
            
            // Intensity
            let intensity_offset = (size_of::<GLfloat>() * 2) as i32;
            gl::EnableVertexAttribArray(1);
            gl::VertexAttribPointer(1, 1, gl::FLOAT, gl::FALSE, 
                                    stride, intensity_offset as *const GLvoid); 
            
            // Unbinding
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindVertexArray(0);
            gl::UseProgram(0);

            
            // ====================== Shadowmask =======================
            gl::GenBuffers(1, &mut self.ebo as *mut GLuint);
            
            gl::UseProgram(self.shaders[1]);
            
            gl::BindVertexArray(self.vaos[1]);
            gl::BindBuffer(gl::ARRAY_BUFFER, self.abos[1]);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.ebo);
          
            // Pos
            gl::EnableVertexAttribArray(0);
            gl::VertexAttribPointer(0, 2, gl::FLOAT, gl::FALSE, 16, 0 as *const GLvoid);
            // UV
            gl::EnableVertexAttribArray(1);
            gl::VertexAttribPointer(1, 2, gl::FLOAT, gl::FALSE, 16, 8 as *const GLvoid);

            let screen_quad: [f32; 16] = [
                 1.0,  1.0, 1.0, 1.0,
                 1.0, -1.0, 1.0, 0.0,
                -1.0, -1.0, 0.0, 0.0,
                -1.0,  1.0, 0.0, 1.0,
            ];
            let scr_quad_ind: [GLuint; 6] = [
                0, 1, 3,
                1, 2, 3,
            ];

            gl::BufferData(
                gl::ARRAY_BUFFER,  
                (size_of::<f32>() * screen_quad.len()) as GLsizeiptr,
                screen_quad.as_ptr() as *const GLvoid,
                gl::STATIC_DRAW
            );
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                (size_of::<GLuint>() * scr_quad_ind.len()) as GLsizeiptr,
                scr_quad_ind.as_ptr() as *const GLvoid,
                gl::STATIC_DRAW
            );

            self.uniform_locations[0] = shader::get_uniform_location(
                self.shaders[0], "view_projection");
            self.uniform_locations[1] = shader::get_uniform_location(
                self.shaders[0], "px_scale");
            self.uniform_locations[2] = shader::get_uniform_location(
                self.shaders[1], "lightmap_scale");

            gl::BindVertexArray(0);
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);
            gl::UseProgram(0);

        }

        self.initialized = true;
        Ok(())

    }

    pub fn render(&self, lights: &[RenderLight], window_size: (f32, f32)) {
        let (winx, winy) = window_size;
        let (s_winx, s_winy) = (window_size.0 / self.dither_scale, 
                                window_size.1 / self.dither_scale);
        let scaled_vp = glm::ortho(0.0, s_winx, 0.0, s_winy, -1.0, 1.0);
        
        // ============== Render lightmap to framebuffer =============
        unsafe {
            gl::Enable(gl::BLEND);
            gl::Disable(gl::DEPTH_TEST);
            
            gl::UseProgram(self.shaders[0]);
            gl::BindVertexArray(self.vaos[0]);
            gl::BindFramebuffer(gl::FRAMEBUFFER, self.fbo);
            gl::BindTexture(gl::TEXTURE_2D, self.tex);
            gl::TexImage2D(
                gl::TEXTURE_2D, 0, gl::RGB as i32, 
                s_winx as i32, s_winy as i32, 0, 
                gl::RGB, gl::UNSIGNED_BYTE, 
                std::ptr::null()
            );
            gl::FramebufferTexture(gl::FRAMEBUFFER, gl::COLOR_ATTACHMENT0, self.tex, 0);  
            
            // Black the framebuffer
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            // Enable additive blending ( for light blending )
            gl::BlendFunc(gl::ONE, gl::ONE);
            
            gl::Viewport(0, 0, s_winx as i32, s_winy as i32);
            // view_projection
            gl::UniformMatrix4fv(self.uniform_locations[0], 1, gl::FALSE, 
                                 scaled_vp.as_ptr());
    
            // px_scale
            gl::Uniform1f(self.uniform_locations[1], self.dither_scale);

            // Load point light data
            gl::BindBuffer(gl::ARRAY_BUFFER, self.abos[0]);
            gl::BufferData(
                gl::ARRAY_BUFFER, 
                (size_of::<RenderLight>() * lights.len()) as GLsizeiptr,
                lights.as_ptr() as *const GLvoid,
                gl::DYNAMIC_DRAW
            );
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            
            // Render lightmap to texture
            gl::DrawArrays(gl::POINTS, 0, lights.len() as i32);
            gl::BindFramebuffer(gl::FRAMEBUFFER, self.fbo);
            gl::DrawArrays(gl::POINTS, 0, lights.len() as i32);
            // Reset the blend function to normal
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
            gl::BlendEquation(gl::FUNC_ADD); 
            
            // Revert to using the screen's framebuffer
            gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
            
            // ========= Render the shadow mask =============
            // This renders the lightmap onto a quad, run through
            // a frag shader that discards specific fragments.
            // The fragments to discard are selected using bayesian
            // dithering.
            // ----------------------------------------------
            gl::UseProgram(self.shaders[1]);
            gl::BindVertexArray(self.vaos[1]);
            gl::Viewport(0, 0, winx as i32, winy as i32);
            
            // lightmap_scale
            gl::Uniform1f(self.uniform_locations[2], self.dither_scale);
            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, 0 as *const GLvoid);

            gl::BindTexture(gl::TEXTURE_2D, 0);
            gl::BindVertexArray(0);
            gl::UseProgram(0);
        }
    }
}

