extern crate glfw;

use glfw::{Action, Context, Key};

use std::sync::mpsc::Receiver;

const WIDTH: u32 = 1280;
const HEIGHT: u32 = 720;

pub fn create_display() -> (glfw::Window, Receiver<(f64, glfw::WindowEvent)>, glfw::Glfw) {
    // initialize and configure GLFW
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(
        glfw::OpenGlProfileHint::Core
    ));
    glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));

    // GLFW create window
    let (mut window, events) = glfw.create_window(
        WIDTH, HEIGHT, "Blues Game Engine", glfw::WindowMode::Windowed
    ).expect("Failed to create GLFW window.");

    window.set_key_polling(true);
    window.make_current();
    
    (window, events, glfw)
}

pub fn update_display(mut window: &mut glfw::Window, events: &Receiver<(f64, glfw::WindowEvent)>, glfw: &mut glfw::Glfw) {
    glfw.poll_events();
    for (_, event) in glfw::flush_messages(&events) {
        handle_window_event(&mut window, event);
    }
}

pub fn close_display(window: glfw::Window,) {
    window.close();
}


fn handle_window_event(window: &mut glfw::Window, event: glfw::WindowEvent) {
    match event {
        glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
            window.set_should_close(true);
        }
        _ => {}
    }
}