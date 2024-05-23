use crate::engine::graphics::{texture::Texture, vertex_buffers::Vertex, Graphics};

pub struct Player {
    x: f32,
    y: f32,
}

const VERTICES: &[Vertex] = &[
    Vertex {
        position: [-0.5, 0.0, 0.0],
        tex_coord: [0.0, 0.5],
    }, // A
    Vertex {
        position: [-0.3, -0.5, 0.0],
        tex_coord: [0.0, 0.75],
    }, // B
    Vertex {
        position: [0.3, -0.5, 0.0],
        tex_coord: [1.0, 0.75],
    }, // C
    Vertex {
        position: [0.5, 0.0, 0.0],
        tex_coord: [1.0, 0.5],
    }, // D
    Vertex {
        position: [0.0, 0.5, 0.0],
        tex_coord: [0.5, 0.0],
    }, // E
];

const INDICES: &[u16] = &[0, 1, 4, 1, 2, 4, 2, 3, 4];

impl Player {
    pub fn new(x: f32, y: f32) -> Self {
        Player { x, y }
    }

    pub fn init(&self, graphics: &mut Graphics) {
        let tex = graphics.load_texture("../../../../../assets/profile.png");
        let tex2 = graphics.load_texture("../../../../../assets/hand-drawn-robot.png");

        graphics.load_shader(
            "../../../shaders/dyn_polygon.wgsl",
            Some(VERTICES),
            Some(INDICES),
            Some(tex),
        );
        graphics.load_shader(
            "../../../shaders/dyn_polygon-b.wgsl",
            Some(VERTICES),
            Some(INDICES),
            None,
        );
        //TODO: Fix error with bind_groups (Texture.bind_grops)
        graphics.load_shader(
            "../../../shaders/dyn_polygon-c.wgsl",
            Some(VERTICES),
            Some(INDICES),
            Some(tex2),
        );
    }

    pub fn draw(&self, graphics: &mut Graphics) {}
}
