extern crate glfw;
use glfw::{Key, Action};

use cgmath::{Vector3, vec3};

pub struct Camera {
    position: Vector3<f32>,
    pitch: f32,
    yaw: f32,
    roll: f32,
}

impl Camera {
    pub fn new() -> Camera {
        Camera {
            position: vec3(0.0, 0.0, 0.0),
            pitch: 0.0,
            yaw: 0.0,
            roll: 0.0,
        }
    }

    pub fn move_camera(&mut self, window: &glfw::Window) {
        if window.get_key(Key::W) == Action::Press {
            self.position.z -= 0.02;
        }

        if window.get_key(Key::D) == Action::Press {
            self.position.x += 0.02;
        }

        if window.get_key(Key::A) == Action::Press {
            self.position.x -= 0.02;
        }
    }

    pub fn get_position(&self) -> &Vector3<f32> {
        &self.position
    }

    pub fn get_pitch(&self) -> &f32 {
        &self.pitch
    }

    pub fn get_yaw(&self) -> &f32 {
        &self.yaw
    }

    pub fn get_roll(&self) -> &f32 {
        &self.roll
    }
}