extern crate gl;

use cgmath::{Matrix4, perspective, Deg};

use std::ptr;

use crate::entities::entity::Entity;
use crate::shaders::static_shader::StaticShader;
use crate::toolbox::math;

const FOV: f32 = 70.0;
const NEAR_PLANE: f32 = 0.1;
const FAR_PLANE: f32 = 1000.0;


pub fn initialize(shader: &StaticShader) {
    let projection_matrix = create_projection_matrix();
    shader.program.start();
    shader.load_projection_matrix(&projection_matrix);
    shader.program.stop();
}

fn create_projection_matrix() -> Matrix4<f32> {
    let aspect_ratio = crate::WIDTH as f32 / crate::HEIGHT as f32;
    return perspective(Deg(FOV), aspect_ratio, NEAR_PLANE, FAR_PLANE);
}

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
