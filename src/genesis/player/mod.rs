use crate::engine::{
    ecs::component::Component,
    graphics::{self, Graphics},
};

#[derive(Component)]
pub struct Player {
    x: f32,
    y: f32,
}

impl Player {
    pub fn new(x: f32, y: f32) -> Self {
        Player { x, y }
    }

    pub fn draw(&self, graphics: &Graphics) {
        todo!()
    }
}
