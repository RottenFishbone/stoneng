#![allow(unused_variables, dead_code, unused_imports)]
use crate::error::EngineError;
use gl::types::*;
use std::{ fs, ffi::{CString, CStr} };

#[derive(Debug, Clone)]
pub enum ShaderType {
    VertexShader = gl::VERTEX_SHADER as isize,
    FragmentShader = gl::FRAGMENT_SHADER as isize,
    GeometryShader = gl::GEOMETRY_SHADER as isize,
}

/// Returns the position of a shader program's uniform.
///
/// The name must contain no null bytes or will result in an error
/// on conversion to CString.
pub fn get_uniform_location(program: GLuint, name: &str) -> GLint {
    unsafe {
        let cname = CString::new(name).expect("Null byte contained within uniform name.");
        gl::GetUniformLocation(program, cname.as_ptr() as *const GLchar)
    }
}

/// Builds an OpenGL shader program using GLSL sources.
///
/// The shader program is created, compiled and linked. If no geometry shader
/// is provided it will be omitted from the linking. 
/// Each shader's source code must be free of null bytes or there will errors
/// upon conversion to CStrings (a requirement of OpenGL C bindings).
///
/// Shaders are detached and deleted after program is linked.
pub fn program_from_sources(vert_source: String, 
                            frag_source: String, 
                            geom_source: Option<String>)
                            -> Result<GLuint, EngineError> {
    // Compile mandatory shaders
    let vert_shader = compile_source(vert_source, ShaderType::VertexShader)?;
    let frag_shader = compile_source(frag_source, ShaderType::FragmentShader)?;
    // Optionally compile the geometry shader
    let geom_shader = match geom_source {
        Some(source) => {
            // Wrap the result in Some, otherwise return the error
            match compile_source(source, ShaderType::GeometryShader) {
                Ok(shader) => Some(shader),
                Err(err) => return Err(err),
            }
        },
        // Forward the None value to linking
        None => None,
    };
    
    // Link the program and save the Result
    let program = link_program(vert_shader, frag_shader, geom_shader);
    
    // Cleanup OpenGL objects
    unsafe {
        gl::DeleteShader(vert_shader);
        gl::DeleteShader(frag_shader);
        if let Some(shader) = geom_shader {
            gl::DeleteShader(shader);
        }
    }

    // Forward the linking Result
    program
}

/// Creates a shader program and links shaders to it.
///
/// Shaders are detached after successful linking.
fn link_program(vert_shader: GLuint, 
                frag_shader: GLuint, 
                geom_shader: Option<GLuint>) 
                -> Result<GLuint, EngineError> {
    
    let program: GLuint;
    unsafe {
        // Create and link the program
        program = gl::CreateProgram();
        gl::AttachShader(program, vert_shader);
        gl::AttachShader(program, frag_shader);
        if let Some(shader) = geom_shader {
            gl::AttachShader(program, shader);
        }
        gl::LinkProgram(program);

        // Check for linking errors
        let mut link_success: GLint = 1;
        gl::GetProgramiv(program, gl::LINK_STATUS, &mut link_success);
        
        // If failed, get the log
        if link_success == 0 {
        
            let mut log_len: GLint = 0;
            gl::GetProgramiv(program, gl::INFO_LOG_LENGTH, &mut log_len);
            let mut log_buffer: Vec<u8> = Vec::with_capacity(log_len as usize + 1);
            // Fill the buffer with spaces
            log_buffer.extend([b' '].iter().cycle().take(log_len as usize));
            let error_log: CString = CString::from_vec_unchecked(log_buffer);
            
            // Load the log into CString
            gl::GetProgramInfoLog(program, log_len, 
                                  std::ptr::null_mut(),
                                  error_log.as_ptr() as *mut GLchar);

            // Convert CString to String
            let error = error_log.to_string_lossy().into_owned();
            // Return as linking error
            println!("=====Linking=====\n{}", error);
            return Err(EngineError::ShaderLink(error));
        }
        
        // Detach shader files from the program
        gl::DetachShader(program, vert_shader);
        gl::DetachShader(program, frag_shader);
        if let Some(shader) = geom_shader {
            gl::DetachShader(program, shader);
        }
    }
    Ok(program)
}


/// Compiles an OpenGL glsl shader from source code and returns the shader id.
///
/// This is mostly used as a helper for 'program_from_*' which can be used
/// to build an OpenGL shader program.
/// The source should not contain any null bytes or it will result in an
/// error while converting the source into a CString.
pub fn compile_source(source: String, shader_type: ShaderType)
        -> Result<GLuint, EngineError> {
    
    //println!("================={:?}================\n{}", shader_type, source);
    
    // Virtually all this code is unsafe as it deals with CStrings and
    // OpenGL calls
    let shader: GLuint;
    unsafe {
        // Convert source to CString
        let csource = match CString::new(source) {
            Ok(cstring) => cstring,
            Err(err) => {
                return Err(EngineError::ShaderCompile("Failed to build CString".into()));
            }
        };

        shader = gl::CreateShader(shader_type.clone() as u32);
        gl::ShaderSource(shader, 1, &csource.as_ptr(), std::ptr::null());
        gl::CompileShader(shader);
        
        // Check for compilation success
        let mut compile_success: GLint = 1;
        gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut compile_success);
        
        // On fail, retrieve the error
        if compile_success == 0 {
            let mut log_len: GLint = 0;
            gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut log_len);
            let mut log_buffer: Vec<u8> = Vec::with_capacity(log_len as usize + 1);
            // Fill the buffer with spaces
            log_buffer.extend([b' '].iter().cycle().take(log_len as usize));
            let error_log: CString = CString::from_vec_unchecked(log_buffer);
            
            // Load the log into CString
            gl::GetShaderInfoLog(shader, 
                                log_len, 
                                std::ptr::null_mut(),
                                error_log.as_ptr() as *mut GLchar);

            // Convert CString to String
            let error = error_log.to_string_lossy().into_owned();
            // Return as compile error
            println!("========= {:?} Compile Error =========\n{}\n==========================", 
                     &shader_type, error);
            return Err(EngineError::ShaderCompile(error));
        }
    }
    
    // Return the shader id that OpenGL provided
    Ok(shader)
}

/// Compiles a file into an OpenGL shader and returns the shader id.
///
/// This is mostly used as a helper for `program_from_*` which can be used
/// to build an OpenGL shader program.
/// The source should not contain any null bytes or it will result in an
/// error while converting the source into a CString.
pub fn compile_file(path: String, shader_type: ShaderType) 
        -> Result<GLuint, EngineError> {
    
    // Load the data and provide to compile_from_source
    let shader_data = fs::read_to_string(path).expect("Failed to load shader file.");
    compile_source(shader_data, shader_type)
}
