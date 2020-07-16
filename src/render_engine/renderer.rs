extern crate gl;

use std::ptr;

use crate::entities::entity::Entity;
use crate::shaders::static_shader::StaticShader;
use crate::toolbox::math;

pub fn prepare() {
    unsafe {
        gl::Clear(gl::COLOR_BUFFER_BIT);
        gl::ClearColor(1.0, 0.0, 0.0, 1.0);
    }
}

pub fn render(entity: &Entity, shader: &StaticShader) {
    let model = entity.get_model();
    let raw_model = model.get_raw_model();

    unsafe { 
        gl::BindVertexArray(*raw_model.vao_id());
        gl::EnableVertexAttribArray(0);
        gl::EnableVertexAttribArray(1);

        let transformation_matrix = math::create_transformation_matrix(
            entity.get_position(),
            *entity.get_rot_x(),
            *entity.get_rot_y(),
            *entity.get_rot_z(),
            *entity.get_scale()
        );

        shader.load_transformation_matrix(&transformation_matrix);

        gl::ActiveTexture(gl::TEXTURE0);
        gl::BindTexture(gl::TEXTURE_2D, *model.get_texture().get_texture_id());
        gl::DrawElements(gl::TRIANGLES, *raw_model.vertex_count(), gl::UNSIGNED_INT, ptr::null());
        gl::DisableVertexAttribArray(0);
        gl::DisableVertexAttribArray(1);
        gl::BindVertexArray(0);
    }
}
