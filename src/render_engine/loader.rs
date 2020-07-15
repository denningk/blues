extern crate gl;
use self::gl::types::*;

extern crate image;
use image::GenericImageView;

use std::mem;
use std::os::raw::c_void;
use std::ptr;
use std::path::Path;

use crate::models::raw_model::RawModel;

pub struct Loader {
    vaos: Vec<u32>,
    vbos: Vec<u32>,
    textures: Vec<u32>,
}

impl Loader {
    pub fn new() -> Loader {
        Loader { vaos: Vec::new(), vbos: Vec::new(), textures: Vec::new() }
    }

    pub fn load_to_vao(&mut self, positions: &Vec<f32>, texture_coords: &Vec<f32>, indices: &Vec<u32>) -> RawModel {
        let vao_id = self.create_vao();
        self.bind_indices_buffer(indices);
        self.store_data_in_attribute_list(0, 3, &positions);
        self.store_data_in_attribute_list(1, 2, &texture_coords);
        self.unbind_vao();

        RawModel::new(vao_id, indices.len() as i32)
    }

    pub fn load_texture(&mut self, file_name: &str) -> u32 {
        let texture = unsafe {
            let mut texture = 0;
            
            // Generate and bind an OpenGL texture
            gl::GenTextures(1, &mut texture);
            gl::BindTexture(gl::TEXTURE_2D, texture);

            gl::TexParameterf(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as f32);
            gl::TexParameterf(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as f32);
            
            texture
        };
        
        unsafe {
            let img = image::open(&Path::new(&format!("res/{}", file_name))).expect("Failed to load texture");
            let data = img.to_rgb().into_raw();
            let (width, height) = img.dimensions();
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGB as i32,
                width as i32,
                height as i32,
                0,
                gl::RGB,
                gl::UNSIGNED_BYTE,
                &data[0] as *const u8 as *const c_void
            );
            gl::GenerateMipmap(gl::TEXTURE_2D);
        }

        self.textures.push(texture);
        return texture;
    }

    pub fn clean_up(&self) {
        for i in &self.vaos {
            unsafe { gl::DeleteVertexArrays(1, i); }
        }

        for i in &self.vbos {
            unsafe { gl::DeleteBuffers(1, i); }
        }

        for i in &self.textures {
            unsafe { gl::DeleteTextures(1, i); }
        }

    }

    fn create_vao(&mut self) -> u32 {
        let mut vao_id = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut vao_id);
            self.vaos.push(vao_id);
            gl::BindVertexArray(vao_id);
        }

        vao_id
    }

    fn store_data_in_attribute_list(&mut self, attribute_number: u32, coordinate_size: i32, data: &Vec<f32>) {
        let mut vbo_id = 0;
        unsafe {
            gl::GenBuffers(1, &mut vbo_id);
            self.vbos.push(vbo_id);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo_id);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (data.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                &data[0] as *const f32 as *const c_void,
                gl::STATIC_DRAW
            );
            gl::VertexAttribPointer(attribute_number, coordinate_size, gl::FLOAT, gl::FALSE, 0, ptr::null());
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        }
    }

    fn bind_indices_buffer(&mut self, indices: &Vec<u32>) {
        let mut vbo_id = 0;
        unsafe {
            gl::GenBuffers(1, &mut vbo_id);
            self.vbos.push(vbo_id);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, vbo_id);
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                (indices.len() * mem::size_of::<GLint>()) as GLsizeiptr,
                &indices[0] as *const u32 as *const c_void,
                gl::STATIC_DRAW
            );
        }
    }

    fn unbind_vao(&self) {
        unsafe {
            gl::BindVertexArray(0);
        }
    }
}
