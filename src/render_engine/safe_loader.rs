extern crate image;
use image::GenericImageView;

use std::path::Path;

use crate::models::raw_model::RawModel;
use crate::render_engine::gl_utils::{SafeBuffer, SafeVertexArray, SafeTexture};
use crate::gl;

pub struct SafeLoader {
    vaos: Vec<SafeVertexArray>,
    vbos: Vec<SafeBuffer>,
    textures: Vec<SafeTexture>,
}

impl SafeLoader {
    pub fn new() -> SafeLoader {
        SafeLoader { 
            vaos: Vec::new(), 
            vbos: Vec::new(), 
            textures: Vec::new() 
        }
    }

    pub fn load_to_vao(&mut self, positions: &[f32], texture_coords: &[f32], indices: &[u32]) -> Result<RawModel, String> {
        let vao = SafeVertexArray::new()?;
        vao.bind();
        
        let indices_buffer = SafeBuffer::new(gl::ELEMENT_ARRAY_BUFFER)?;
        indices_buffer.set_data(indices, gl::STATIC_DRAW);
        
        self.store_data_in_attribute_list(0, 3, positions)?;
        self.store_data_in_attribute_list(1, 2, texture_coords)?;
        
        vao.unbind();

        let vao_id = vao.id();
        self.vaos.push(vao);
        self.vbos.push(indices_buffer);

        Ok(RawModel::new(vao_id, indices.len() as i32))
    }

    pub fn load_texture(&mut self, file_name: &str) -> Result<u32, String> {
        let texture = SafeTexture::new(gl::TEXTURE_2D)?;
        texture.bind();

        texture.set_parameter_f(gl::TEXTURE_MIN_FILTER, gl::LINEAR as f32);
        texture.set_parameter_f(gl::TEXTURE_MAG_FILTER, gl::LINEAR as f32);
        
        let img = image::open(&Path::new(&format!("res/{}", file_name)))
            .map_err(|e| format!("Failed to load texture: {}", e))?;
        
        let data = img.to_rgb8().into_raw();
        let (width, height) = img.dimensions();
        
        unsafe {
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGB as i32,
                width as i32,
                height as i32,
                0,
                gl::RGB,
                gl::UNSIGNED_BYTE,
                data.as_ptr() as *const std::ffi::c_void
            );
            gl::GenerateMipmap(gl::TEXTURE_2D);
        }

        let texture_id = texture.id();
        self.textures.push(texture);
        Ok(texture_id)
    }

    fn store_data_in_attribute_list(&mut self, attribute_number: u32, coordinate_size: i32, data: &[f32]) -> Result<(), String> {
        let vbo = SafeBuffer::new(gl::ARRAY_BUFFER)?;
        vbo.set_data(data, gl::STATIC_DRAW);
        
        unsafe {
            gl::VertexAttribPointer(
                attribute_number, 
                coordinate_size, 
                gl::FLOAT, 
                gl::FALSE, 
                0, 
                std::ptr::null()
            );
        }
        
        vbo.unbind();
        self.vbos.push(vbo);
        Ok(())
    }
}