use winit::{
    event_loop::EventLoopWindowTarget,
    keyboard::{KeyCode, PhysicalKey},
};

use crate::engine::graphics::Graphics;

use self::player::Player;
mod player;

pub struct GamePlay {
    player: Player,
}

impl GamePlay {
    pub fn new() -> Self {
        GamePlay {
            player: Player::new(0., 0.),
        }
    }

    pub fn draw(&self, graphics: &mut Graphics) {
        graphics.set_clear_color(wgpu::Color::BLACK);
        self.player.draw(graphics);
    }

    pub fn input(&self, key: &PhysicalKey, target: &EventLoopWindowTarget<()>) {
        match key {
            PhysicalKey::Code(KeyCode::Escape) => target.exit(),
            _ => (),
        }
    }
}
