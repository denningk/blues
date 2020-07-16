use cgmath::{Vector3};

use crate::models::textured_model::TexturedModel;

pub struct Entity {
    model: TexturedModel,
    position: Vector3<f32>,
    rot_x: f32,
    rot_y: f32,
    rot_z: f32,
    scale: f32,
}

impl Entity {
    pub fn new(model: TexturedModel, position: Vector3<f32>, rot_x: f32,
        rot_y: f32, rot_z: f32, scale: f32) -> Entity {
            Entity { model, position, rot_x, rot_y, rot_z, scale }
    }

    pub fn increase_position(&mut self, dx: f32, dy: f32, dz: f32) {
        self.position.x += dx;
        self.position.y += dy;
        self.position.z += dz;
    }

    pub fn increase_rotation(&mut self, dx: f32, dy: f32, dz: f32) {
        self.rot_x += dx;
        self.rot_y += dy;
        self.rot_z += dz;
    }

    pub fn get_model(&self) -> &TexturedModel {
        &self.model
    }

    pub fn get_position(&self) -> &Vector3<f32> {
        &self.position
    }

    pub fn get_rot_x(&self) -> &f32 {
        &self.rot_x
    }

    pub fn get_rot_y(&self) -> &f32 {
        &self.rot_y
    }

    pub fn get_rot_z(&self) -> &f32 {
        &self.rot_z
    }

    pub fn get_scale(&self) -> &f32 {
        &self.scale
    }
    
    pub fn set_model(&mut self, model: TexturedModel) {
        self.model = model;
    }

    pub fn set_position(&mut self, position: Vector3<f32>) {
        self.position = position;
    }

    pub fn set_rot_x(&mut self, rot_x: f32) {
        self.rot_x = rot_x;
    }

    pub fn set_rot_y(&mut self, rot_y: f32) {
        self.rot_y = rot_y;
    }

    pub fn set_rot_z(&mut self, rot_z: f32) {
        self.rot_z = rot_z;
    }

    pub fn set_scale(&mut self, scale: f32) {
        self.scale = scale;
    }
}
