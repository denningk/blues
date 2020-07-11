extern crate gl;

use super::shader_program;

const VERTEX_FILE: &str = "src/shaders/vertexShader.glsl";
const FRAGMENT_FILE: &str = "src/shaders/fragmentShader.glsl";

pub fn new_static_shader() -> shader_program::ShaderProgram {
    return shader_program::ShaderProgram::new(VERTEX_FILE, FRAGMENT_FILE, 0, "position");
}
