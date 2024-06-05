pub mod engine;

pub mod prelude {
    pub use crate::engine;
    pub use crate::engine::camera;
    pub use wgpu::Color;
}
