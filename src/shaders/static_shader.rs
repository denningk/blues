extern crate gl;

use cgmath::{Matrix4};

use super::shader_program;
use crate::entities::camera::Camera;
use crate::toolbox::math;

const VERTEX_FILE: &str = "src/shaders/vertexShader.glsl";
const FRAGMENT_FILE: &str = "src/shaders/fragmentShader.glsl";

pub struct StaticShader<'a> {
    pub program: shader_program::ShaderProgram<'a>,
    location_transformation_matrix: i32,
    location_projection_matrix: i32,
    location_view_matrix: i32,
}

impl<'a> StaticShader<'a> {
    pub fn new() -> StaticShader<'a> {
        let program = shader_program::ShaderProgram::new(
            VERTEX_FILE,
            FRAGMENT_FILE,
            vec!["position", "textureCoords"],
            vec!["transformationMatrix"]
        );
        
        let location_transformation_matrix = program.get_uniform_location("transformationMatrix");
        let location_projection_matrix = program.get_uniform_location("projectionMatrix");
        let location_view_matrix = program.get_uniform_location("viewMatrix");

        StaticShader { program, location_transformation_matrix, location_projection_matrix, location_view_matrix }
    }

    pub fn load_transformation_matrix(&self, matrix: &Matrix4<f32>) {
        shader_program::ShaderProgram::load_matrix(self.location_transformation_matrix, matrix);
    }

    pub fn load_projection_matrix(&self, projection: &Matrix4<f32>) {
        shader_program::ShaderProgram::load_matrix(self.location_projection_matrix, projection);
    }

    pub fn load_view_matrix(&self, camera: &Camera) {
        let view_matrix = math::create_view_matrix(camera);
        shader_program::ShaderProgram::load_matrix(self.location_view_matrix, &view_matrix);
    }
}
