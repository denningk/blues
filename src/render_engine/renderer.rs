extern crate gl;

use super::raw_model::RawModel;

pub fn prepare() {
    unsafe {
        gl::ClearColor(1.0, 0.0, 0.0, 1.0);
        gl::Clear(gl::COLOR_BUFFER_BIT);
    }
}

pub fn render(model: &RawModel) {
    unsafe { 
        gl::BindVertexArray(*model.vao_id());
        gl::EnableVertexAttribArray(0);
        gl::DrawArrays(gl::TRIANGLES, 0, *model.vertex_count());
        gl::DisableVertexAttribArray(0);
        gl::BindVertexArray(0);
    }
}
