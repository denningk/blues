extern crate gl;
use self::gl::types::*;

use std::fs;
use std::ffi::CString;
use std::ptr;
use std::str;

pub struct ShaderProgram {
    program_id: u32,
    vertex_shader_id: u32,
    fragment_shader_id: u32,
}

impl ShaderProgram {
    pub fn new(vertex_file: &str, fragment_file: &str, attribute: u32, variable_name: &str) -> ShaderProgram {
        let vertex_shader_id = load_shader(vertex_file, gl::VERTEX_SHADER);
        let fragment_shader_id = load_shader(fragment_file, gl::FRAGMENT_SHADER);
        
        let program_id = unsafe {
            let program_id = gl::CreateProgram();
            gl::AttachShader(program_id, vertex_shader_id);
            gl::AttachShader(program_id, fragment_shader_id);
            gl::LinkProgram(program_id);
            gl::ValidateProgram(program_id);
            Self::bind_attribute(program_id, attribute, variable_name);

            program_id
        };

        ShaderProgram { program_id, vertex_shader_id, fragment_shader_id }
    }

    pub fn start(&self) {
        unsafe { gl::UseProgram(self.program_id); }
    }
    
    pub fn stop(&self) {
        unsafe { gl::UseProgram(0); }
    }

    pub fn clean_up(&self) {
        self.stop();
        unsafe {
            gl::DetachShader(self.program_id, self.vertex_shader_id);
            gl::DetachShader(self.program_id, self.fragment_shader_id);
            gl::DeleteShader(self.vertex_shader_id);
            gl::DeleteShader(self.fragment_shader_id);
            gl::DeleteProgram(self.program_id);
        }
    }

    fn bind_attribute(program_id: u32, attribute: u32, variable_name: &str) {
        unsafe { gl::BindAttribLocation(program_id, attribute, CString::new(variable_name).unwrap().as_ptr()); }
    }
}

fn load_shader(file: &str, shader_type: u32) -> u32 {
    let shader_source = fs::read_to_string(file)
        .expect("Unable to read shader file");
        
    //println!(shader_source);
    let shader_id = unsafe {
        let shader_id = gl::CreateShader(shader_type);
        let c_str_shader = CString::new(shader_source.as_bytes()).unwrap();
        gl::ShaderSource(shader_id, 1, &c_str_shader.as_ptr(), ptr::null());
        gl::CompileShader(shader_id);

        // Check if shader compiled successfully
        let mut success = gl::FALSE as GLint;
        let mut info_log = Vec::with_capacity(512);
        let info_log_len_ptr = &mut 0 as *mut GLsizei;
        gl::GetShaderiv(shader_id, gl::COMPILE_STATUS, &mut success);
        if success != gl::TRUE as GLint {
            gl::GetShaderInfoLog(shader_id, 512, info_log_len_ptr, info_log.as_mut_ptr() as *mut GLchar);
            info_log.set_len(*info_log_len_ptr as usize);
            panic!("Shader did not compile! {}", str::from_utf8(&info_log).unwrap());
        }

        shader_id
    };
    
    return shader_id;
}