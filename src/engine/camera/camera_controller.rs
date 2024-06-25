use winit::{
    event::{ElementState, KeyEvent},
    keyboard::{KeyCode, PhysicalKey},
};

use super::Camera;

pub struct CameraController {
    forward: bool,
    backwards: bool,
    left: bool,
    right: bool,
    speed: f32,
}

impl CameraController {
    pub fn new(speed: f32) -> Self {
        Self {
            forward: false,
            backwards: false,
            left: false,
            right: false,
            speed,
        }
    }

    pub fn process_events(&mut self, event: &KeyEvent) -> bool {
        match event {
            KeyEvent {
                physical_key: PhysicalKey::Code(keycode),
                state,
                ..
            } => {
                let is_pressed = *state == ElementState::Pressed;

                match keycode {
                    KeyCode::ArrowUp => {
                        self.forward = is_pressed;
                        true
                    }
                    KeyCode::ArrowDown => {
                        self.backwards = is_pressed;
                        true
                    }
                    KeyCode::ArrowLeft => {
                        self.left = is_pressed;
                        true
                    }
                    KeyCode::ArrowRight => {
                        self.right = is_pressed;
                        true
                    }
                    _ => false,
                }
            }
            _ => false,
        }
    }

    pub fn update_camera(&self, camera: &mut Camera) {
        use cgmath::InnerSpace;

        let forward = camera.target - camera.eye;
        let forward_norm = forward.normalize();
        let forward_mag = forward.magnitude();

        if self.forward && forward_mag > self.speed {
            camera.eye += forward_norm * self.speed;
        }

        if self.backwards {
            camera.eye -= forward_norm * self.speed;
        }

        let right = forward_norm.cross(camera.up);

        let forward = camera.target - camera.eye;
        let forward_mag = forward.magnitude();

        if self.right {
            camera.eye = camera.target - (forward + right * self.speed).normalize() * forward_mag;
        }

        if self.left {
            camera.eye = camera.target - (forward - right * self.speed).normalize() * forward_mag;
        }
    }
}
