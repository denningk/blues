extern crate gl;

use cgmath::{Matrix4};

use super::shader_program;

const VERTEX_FILE: &str = "src/shaders/vertexShader.glsl";
const FRAGMENT_FILE: &str = "src/shaders/fragmentShader.glsl";

pub struct StaticShader<'a> {
    pub program: shader_program::ShaderProgram<'a>,
    location_transformation_matrix: i32,
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

        StaticShader { program, location_transformation_matrix}
    }

    pub fn load_transformation_matrix(&self, matrix: &Matrix4<f32>) {
        shader_program::ShaderProgram::load_matrix(self.location_transformation_matrix, matrix);
    }
}
