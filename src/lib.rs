pub mod engine;

pub mod prelude {
    pub use crate::engine;
    pub use crate::engine::camera;
    pub use crate::engine::graphics::Graphics;
    pub use crate::engine::scene::Scene;
    pub use wgpu::Color;
    pub use winit::event::WindowEvent;
    pub use winit::event_loop::EventLoopWindowTarget;
    pub use winit::keyboard::*;
}
