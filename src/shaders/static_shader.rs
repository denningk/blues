extern crate gl;

use super::shader_program;

const VERTEX_FILE: &str = "src/shaders/vertexShader.glsl";
const FRAGMENT_FILE: &str = "src/shaders/fragmentShader.glsl";

pub fn new_static_shader<'a>() -> shader_program::ShaderProgram<'a> {
    let program = shader_program::ShaderProgram::new(VERTEX_FILE, FRAGMENT_FILE, vec!["position", "textureCoords"]);

    program
}
