mod render_engine;

use render_engine::display_manager::DisplayManager;
use render_engine::loader::Loader;
use render_engine::renderer;

fn main() {
    // create window
    let mut display_manager = DisplayManager::create_display("Blues Game Engine", 800, 600);

    let mut loader = Loader::new();

    let vertices: Vec<f32> = vec![
        // Left bottom triangle
        -0.5,  0.5, 0.0,
		-0.5, -0.5, 0.0,
		 0.5, -0.5, 0.0,
        // Right top triangle
		 0.5, -0.5, 0.0,
		 0.5,  0.5, 0.0,
		-0.5,  0.5, 0.0
    ];

    let model = loader.load_to_vao(&vertices);

    // render loop
    while !display_manager.should_window_close() {
        renderer::prepare();
        // game logic
        renderer::render(&model);
        display_manager.update_display();
    }

    loader.clean_up();
    display_manager.close_display();
}
