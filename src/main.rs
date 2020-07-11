mod render_engine;
mod shaders;

use render_engine::display_manager::DisplayManager;
use render_engine::loader::Loader;
use render_engine::renderer;
use shaders::static_shader;

fn main() {
    // create window
    let mut display_manager = DisplayManager::create_display("Blues Game Engine", 800, 600);
    let mut loader = Loader::new();
    let shader = static_shader::new_static_shader();

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

    let model = loader.load_to_vao(&vertices, &indices);

    // render loop
    while !display_manager.should_window_close() {
        renderer::prepare();
        shader.start();
        // game logic
        renderer::render(&model);
        shader.stop();
        display_manager.update_display();
    }

    shader.clean_up();
    loader.clean_up();
    display_manager.close_display();
}
