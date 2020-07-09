extern crate gl;
use self::gl::types::*;

use std::mem;
use std::os::raw::c_void;
use std::ptr;

use super::raw_model::RawModel;

pub struct Loader {
    vaos: Vec<u32>,
    vbos: Vec<u32>,
}

impl Loader {
    pub fn new() -> Loader {
        Loader { vaos: Vec::new(), vbos: Vec::new() }
    }

    pub fn load_to_vao(&mut self, positions: Vec<f64>) -> RawModel {
        let vao_id = self.create_vao();
        self.store_data_in_attribute_list(0, &positions);
        self.unbind_vao();

        RawModel::new(vao_id, positions.len() as i32 / 3)
    }

    pub fn clean_up(&self) {
        for i in &self.vaos {
            unsafe { gl::DeleteVertexArrays(1, i); }
        }

        for i in &self.vbos {
            unsafe { gl::DeleteBuffers(1, i); }
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

    fn store_data_in_attribute_list(&mut self, attribute_number: u32, data: &Vec<f64>) {
        let mut vbo_id = 0;
        unsafe {
            gl::GenBuffers(1, &mut vbo_id);
            self.vbos.push(vbo_id);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo_id);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (data.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                &data[0] as *const f64 as *const c_void,
                gl::STATIC_DRAW
            );
            gl::VertexAttribPointer(attribute_number, 3, gl::FLOAT, gl::FALSE, 0, ptr::null());
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        }
    }

    fn unbind_vao(&self) {
        unsafe {
            gl::BindVertexArray(0);
        }
    }
}
