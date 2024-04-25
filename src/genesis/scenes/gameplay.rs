use winit::{
    event_loop::EventLoopWindowTarget,
    keyboard::{KeyCode, PhysicalKey},
};

use crate::engine::{graphics::Graphics, scene::Scene};

pub struct GamePlay;

impl GamePlay {
    pub fn new() -> Scene {
        Scene::new("GamePlay")
            .with_input_system(input_handler)
            .with_draw_system(draw_handler)
    }
}

fn input_handler(key: &PhysicalKey, target: &EventLoopWindowTarget<()>) {
    match key {
        PhysicalKey::Code(KeyCode::Escape) => target.exit(),
        _ => (),
    }
}

fn draw_handler(graphics: &mut Graphics) {
    graphics.set_clear_color(wgpu::Color::BLACK);
}
