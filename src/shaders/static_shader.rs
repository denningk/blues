extern crate gl;

use super::shader_program;

const VERTEX_FILE: &str = "src/shaders/vertexShader.glsl";
const FRAGMENT_FILE: &str = "src/shaders/fragmentShader.glsl";

pub fn new_static_shader() -> shader_program::ShaderProgram {
    let program = shader_program::ShaderProgram::new(VERTEX_FILE, FRAGMENT_FILE);

    program.bind_attribute(0, "position");
    program.bind_attribute(1, "textureCoords");

    program
}
