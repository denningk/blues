extern crate glfw;

use glfw::{Action, Context, Key};

use std::sync::mpsc::Receiver;

pub struct DisplayManager {
    window: glfw::Window,
    events: Receiver<(f64, glfw::WindowEvent)>,
    glfw: glfw::Glfw,
}

impl DisplayManager {
    pub fn create_display(title: &str, width: u32, height: u32) -> DisplayManager {
        // initialize and configure GLFW
        let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
        glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
        glfw.window_hint(glfw::WindowHint::OpenGlProfile(
            glfw::OpenGlProfileHint::Core
        ));
        glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));
    
        // GLFW create window
        let (mut window, events) = glfw.create_window(
            width, height, title, glfw::WindowMode::Windowed
        ).expect("Failed to create GLFW window.");
    
        window.set_key_polling(true);
        window.make_current();
        window.set_framebuffer_size_polling(true);

        // gl: load all OpenGL function pointers
        gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

        DisplayManager { window, events, glfw }
    }
    
    pub fn update_display(&mut self) {
        self.window.swap_buffers();
        self.glfw.poll_events();
        for (_, event) in glfw::flush_messages(&self.events) {
            match event {
                glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                    self.window.set_should_close(true);
                }
                _ => {}
            }
        }
    }
    
    pub fn close_display(self) {
        self.window.close();
    }
    
    pub fn should_window_close(&self) -> bool {
        self.window.should_close()
    }
}
