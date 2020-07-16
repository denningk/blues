mod render_engine;
mod shaders;
mod models;
mod textures;
mod toolbox;
mod entities;

use cgmath::{vec3};

use render_engine::display_manager::DisplayManager;
use render_engine::loader::Loader;
use render_engine::renderer;
use shaders::static_shader;
use textures::model_texture::ModelTexture;
use models::textured_model::TexturedModel;
use toolbox::math;
use entities::entity::Entity;

fn main() {
    // create window
    let mut display_manager = DisplayManager::create_display("Blues Game Engine", 800, 600);
    let mut loader = Loader::new();
    let shader = static_shader::StaticShader::new();

    let vertices: Vec<f32> = vec![
        -0.5,  0.5, 0.0,
		-0.5, -0.5, 0.0,
		 0.5, -0.5, 0.0,
		 0.5,  0.5, 0.0,
    ];

    let indices: Vec<u32> = vec![
        0, 1, 3,
        3, 1, 2
    ];

    let texture_coords: Vec<f32> = vec![
        0.0, 0.0,
        0.0, 1.0,
        1.0, 1.0,
        1.0, 0.0
    ];

    let model = loader.load_to_vao(&vertices, &texture_coords, &indices);
    let texture = ModelTexture::new(loader.load_texture("funny.png"));
    let static_model = TexturedModel::new(model, texture);

    let mut entity = Entity::new(static_model, vec3(-1.0,0.0,0.0), 0.0, 0.0, 0.0, 1.0);

    // render loop
    while !display_manager.should_window_close() {
        entity.increase_position(0.002, 0.0, 0.0);
        entity.increase_rotation(0.0, 1.0, 0.0);
        renderer::prepare();
        shader.program.start();
        // game logic
        renderer::render(&entity, &shader);
        shader.program.stop();
        display_manager.update_display();
    }

    shader.program.clean_up();
    loader.clean_up();
    display_manager.close_display();
}
