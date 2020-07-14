extern crate gl;

use std::ptr;

use crate::models::textured_model::TexturedModel;

pub fn prepare() {
    unsafe {
        gl::Clear(gl::COLOR_BUFFER_BIT);
        gl::ClearColor(1.0, 0.0, 0.0, 1.0);
    }
}

pub fn render(textured_model: &TexturedModel) {
    let model = textured_model.get_raw_model();

    unsafe { 
        gl::BindVertexArray(*model.vao_id());
        gl::EnableVertexAttribArray(0);
        gl::EnableVertexAttribArray(1);
        gl::ActiveTexture(gl::TEXTURE0);
        gl::BindTexture(gl::TEXTURE_2D, *textured_model.get_texture().get_texture_id());
        gl::DrawElements(gl::TRIANGLES, *model.vertex_count(), gl::UNSIGNED_INT, ptr::null());
        gl::DisableVertexAttribArray(0);
        gl::DisableVertexAttribArray(1);
        gl::BindVertexArray(0);
    }
}
