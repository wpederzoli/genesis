use crate::engine::{
    self,
    graphics::{vertex_buffers::Vertex, Graphics},
};

pub struct Player {
    x: f32,
    y: f32,
}

const VERTICES: &[Vertex] = &[
    Vertex {
        position: [0.0, 0.5, 0.0],
        color: [0.2, 0.4, 0.3, 1.0],
    },
    Vertex {
        position: [-0.5, -0.5, 0.0],
        color: [0.2, 0.4, 0.3, 1.0],
    },
    Vertex {
        position: [0.5, -0.5, 0.0],
        color: [0.2, 0.4, 0.3, 1.0],
    },
];

impl Player {
    pub fn new(x: f32, y: f32) -> Self {
        Player { x, y }
    }

    pub fn init(&self, graphics: &mut Graphics) {
        graphics.load_shader("../../../shaders/triangle.wgsl", Some(VERTICES));
        // graphics.load_shader("../../../shaders/tri.wgsl");
    }

    pub fn draw(&self, graphics: &mut Graphics) {}
}
