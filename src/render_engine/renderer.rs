extern crate gl;

use std::ptr;

use super::raw_model::RawModel;

pub fn prepare() {
    unsafe {
        gl::Clear(gl::COLOR_BUFFER_BIT);
        gl::ClearColor(1.0, 0.0, 0.0, 1.0);
    }
}

pub fn render(model: &RawModel) {
    unsafe { 
        gl::BindVertexArray(*model.vao_id());
        gl::EnableVertexAttribArray(0);
        gl::DrawElements(gl::TRIANGLES, *model.vertex_count(), gl::UNSIGNED_INT, ptr::null());
        gl::DisableVertexAttribArray(0);
        gl::BindVertexArray(0);
    }
}
