use crate::engine::graphics::Graphics;

pub struct Player {
    x: f32,
    y: f32,
}

impl Player {
    pub fn new(x: f32, y: f32) -> Self {
        Player { x, y }
    }

    pub fn draw(&self, graphics: &mut Graphics) {
        graphics.load_shader("../../../shaders/triangle.wgsl");
        graphics.load_shader("../../../shaders/tri.wgsl");
    }
}
