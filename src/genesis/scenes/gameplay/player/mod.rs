use crate::engine::graphics::{vertex_buffers::Vertex, Graphics};

pub struct Player {
    x: f32,
    y: f32,
}

const VERTICES: &[Vertex] = &[
    Vertex {
        position: [-0.0868241, 0.49240386, 0.0],
        color: [0.5, 0.0, 0.5, 1.0],
    }, // A
    Vertex {
        position: [-0.49513406, 0.06958647, 0.0],
        color: [0.5, 0.0, 0.5, 1.0],
    }, // B
    Vertex {
        position: [-0.21918549, -0.44939706, 0.0],
        color: [0.5, 0.0, 0.5, 1.0],
    }, // C
    Vertex {
        position: [0.35966998, -0.3473291, 0.0],
        color: [0.5, 0.0, 0.5, 1.0],
    }, // D
    Vertex {
        position: [0.44147372, 0.2347359, 0.0],
        color: [0.5, 0.0, 0.5, 1.0],
    }, // E
];

const INDICES: &[u16] = &[0, 1, 4, 1, 2, 4, 2, 3, 4];

impl Player {
    pub fn new(x: f32, y: f32) -> Self {
        Player { x, y }
    }

    pub fn init(&self, graphics: &mut Graphics) {
        graphics.load_shader(
            "../../../shaders/dyn_polygon.wgsl",
            Some(VERTICES),
            Some(INDICES),
        );
        // graphics.load_shader("../../../shaders/tri.wgsl");
    }

    pub fn draw(&self, graphics: &mut Graphics) {}
}
