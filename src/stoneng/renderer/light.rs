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

