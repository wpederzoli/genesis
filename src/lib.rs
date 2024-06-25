pub mod engine;
pub use bytemuck;

pub mod prelude {
    pub use crate::engine;
    pub use crate::engine::camera;
    pub use crate::engine::graphics::Graphics;
    pub use crate::engine::scene::Scene;
    pub use bytemuck::{self};
    pub use cgmath::{Matrix4, Point3, Vector3};
    pub use genesis_macros::GenesisUniform;
    pub use wgpu::Color;
    pub use wgpu::*;
    pub use winit::event::WindowEvent;
    pub use winit::event_loop::EventLoopWindowTarget;
    pub use winit::keyboard::*;
}
