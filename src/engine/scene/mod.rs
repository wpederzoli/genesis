use std::sync::Arc;

use winit::{event_loop::EventLoopWindowTarget, keyboard::PhysicalKey};

use super::graphics::Graphics;

pub trait BaseScene {
    fn update(&mut self);
    fn input(&mut self);
    fn draw(&mut self);
}

pub struct Scene {
    pub label: String,
    input_handler: Arc<dyn Fn(&PhysicalKey, &EventLoopWindowTarget<()>)>,
    draw_handler: Arc<dyn Fn(&Graphics)>,
}

impl BaseScene for Scene {
    fn update(&mut self) {
        todo!()
    }

    fn input(&mut self) {
        todo!()
    }

    fn draw(&mut self) {
        todo!()
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
        F: Fn(&Graphics) + 'static,
    {
        Scene {
            draw_handler: Arc::new(draw_handler),
            ..self
        }
    }

    pub fn run(&mut self) {
        self.input();
        self.update();
        self.draw();
    }
}
