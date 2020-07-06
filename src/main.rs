mod render_engine;

use render_engine::display_manager;

fn main() {
    println!("Hello, world!");
    let (mut window, events, mut glfw) = display_manager::create_display();

    // render loop
    while !window.should_close() {
        display_manager::update_display(&mut window, &events, &mut glfw);
    }

    display_manager::close_display(window);
}
