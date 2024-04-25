use std::sync::Arc;

use winit::{event_loop::EventLoopWindowTarget, keyboard::PhysicalKey};

use super::graphics::Graphics;

pub trait BaseScene {
    fn input(&self, key: &PhysicalKey, target: &EventLoopWindowTarget<()>);
    fn draw(&self, graphics: &mut Graphics);
}

pub struct Scene {
    pub label: String,
    input_handler: Arc<dyn Fn(&PhysicalKey, &EventLoopWindowTarget<()>)>,
    draw_handler: Arc<dyn Fn(&mut Graphics)>,
}

impl BaseScene for Scene {
    fn input(&self, key: &PhysicalKey, target: &EventLoopWindowTarget<()>) {
        (self.input_handler)(key, target);
    }

    fn draw(&self, graphics: &mut Graphics) {
        (self.draw_handler)(graphics);
    }
}

impl Scene {
    pub fn new(label: &str) -> Self {
        Scene {
            label: label.to_string(),
            input_handler: Arc::new(|_, _| {}),
            draw_handler: Arc::new(|_| {}),
        }
    }

    pub fn with_input_system<F>(self, input_handler: F) -> Self
    where
        F: Fn(&PhysicalKey, &EventLoopWindowTarget<()>) + 'static,
    {
        Scene {
            input_handler: Arc::new(input_handler),
            ..self
        }
    }

    pub fn with_draw_system<F>(self, draw_handler: F) -> Self
    where
        F: Fn(&mut Graphics) + 'static,
    {
        Scene {
            draw_handler: Arc::new(draw_handler),
            ..self
        }
    }
}
