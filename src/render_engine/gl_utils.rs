use std::ffi::CString;
use std::ptr;
use crate::gl;
use crate::gl::types::*;

/// Safe wrapper for OpenGL buffer operations
pub struct SafeBuffer {
    id: u32,
    buffer_type: u32,
}

impl SafeBuffer {
    pub fn new(buffer_type: u32) -> Result<Self, String> {
        let mut id = 0;
        unsafe {
            gl::GenBuffers(1, &mut id);
        }
        
        if id == 0 {
            return Err("Failed to generate buffer".to_string());
        }
        
        Ok(SafeBuffer { id, buffer_type })
    }
    
    pub fn bind(&self) {
        unsafe {
            gl::BindBuffer(self.buffer_type, self.id);
        }
    }
    
    pub fn unbind(&self) {
        unsafe {
            gl::BindBuffer(self.buffer_type, 0);
        }
    }
    
    pub fn set_data<T>(&self, data: &[T], usage: u32) {
        self.bind();
        unsafe {
            gl::BufferData(
                self.buffer_type,
                (data.len() * std::mem::size_of::<T>()) as GLsizeiptr,
                data.as_ptr() as *const std::ffi::c_void,
                usage
            );
        }
    }
    
    pub fn id(&self) -> u32 {
        self.id
    }
}

impl Drop for SafeBuffer {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, &self.id);
        }
    }
}

/// Safe wrapper for VAO operations
pub struct SafeVertexArray {
    id: u32,
}

impl SafeVertexArray {
    pub fn new() -> Result<Self, String> {
        let mut id = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut id);
        }
        
        if id == 0 {
            return Err("Failed to generate vertex array".to_string());
        }
        
        Ok(SafeVertexArray { id })
    }
    
    pub fn bind(&self) {
        unsafe {
            gl::BindVertexArray(self.id);
        }
    }
    
    pub fn unbind(&self) {
        unsafe {
            gl::BindVertexArray(0);
        }
    }
    
    pub fn id(&self) -> u32 {
        self.id
    }
}

impl Drop for SafeVertexArray {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteVertexArrays(1, &self.id);
        }
    }
}

/// Safe wrapper for texture operations
pub struct SafeTexture {
    id: u32,
    texture_type: u32,
}

impl SafeTexture {
    pub fn new(texture_type: u32) -> Result<Self, String> {
        let mut id = 0;
        unsafe {
            gl::GenTextures(1, &mut id);
        }
        
        if id == 0 {
            return Err("Failed to generate texture".to_string());
        }
        
        Ok(SafeTexture { id, texture_type })
    }
    
    pub fn bind(&self) {
        unsafe {
            gl::BindTexture(self.texture_type, self.id);
        }
    }
    
    pub fn unbind(&self) {
        unsafe {
            gl::BindTexture(self.texture_type, 0);
        }
    }
    
    pub fn set_parameter_f(&self, param: u32, value: f32) {
        self.bind();
        unsafe {
            gl::TexParameterf(self.texture_type, param, value);
        }
    }
    
    pub fn id(&self) -> u32 {
        self.id
    }
}

impl Drop for SafeTexture {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteTextures(1, &self.id);
        }
    }
}

/// Safe wrapper for shader operations
pub fn create_shader_safe(shader_type: u32, source: &str) -> Result<u32, String> {
    let shader_id = unsafe { gl::CreateShader(shader_type) };
    
    if shader_id == 0 {
        return Err("Failed to create shader".to_string());
    }
    
    let c_source = CString::new(source).map_err(|_| "Invalid shader source")?;
    
    unsafe {
        gl::ShaderSource(shader_id, 1, &c_source.as_ptr(), ptr::null());
        gl::CompileShader(shader_id);
        
        // Check compilation status
        let mut success = gl::FALSE as GLint;
        gl::GetShaderiv(shader_id, gl::COMPILE_STATUS, &mut success);
        
        if success != gl::TRUE as GLint {
            let mut info_log = Vec::with_capacity(512);
            let mut len = 0;
            gl::GetShaderInfoLog(shader_id, 512, &mut len, info_log.as_mut_ptr() as *mut GLchar);
            info_log.set_len(len as usize);
            
            let error_msg = String::from_utf8_lossy(&info_log);
            gl::DeleteShader(shader_id);
            return Err(format!("Shader compilation failed: {}", error_msg));
        }
    }
    
    Ok(shader_id)
}

/// Safe wrapper for program operations
pub fn create_program_safe(vertex_shader: u32, fragment_shader: u32) -> Result<u32, String> {
    let program_id = unsafe { gl::CreateProgram() };
    
    if program_id == 0 {
        return Err("Failed to create program".to_string());
    }
    
    unsafe {
        gl::AttachShader(program_id, vertex_shader);
        gl::AttachShader(program_id, fragment_shader);
        gl::LinkProgram(program_id);
        
        // Check linking status
        let mut success = gl::FALSE as GLint;
        gl::GetProgramiv(program_id, gl::LINK_STATUS, &mut success);
        
        if success != gl::TRUE as GLint {
            let mut info_log = Vec::with_capacity(512);
            let mut len = 0;
            gl::GetProgramInfoLog(program_id, 512, &mut len, info_log.as_mut_ptr() as *mut GLchar);
            info_log.set_len(len as usize);
            
            let error_msg = String::from_utf8_lossy(&info_log);
            gl::DeleteProgram(program_id);
            return Err(format!("Program linking failed: {}", error_msg));
        }
        
        gl::ValidateProgram(program_id);
    }
    
    Ok(program_id)
}