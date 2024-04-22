use wgpu::{Device, Queue, Surface, SurfaceConfiguration};
use winit::event_loop::EventLoop;

pub struct Graphics {
    event_loop: EventLoop<()>,
    device: Device,
    queue: Queue,
    surface: Surface<'static>,
    config: SurfaceConfiguration,
    clear_color: wgpu::Color,
}

impl Graphics {
    pub fn new(title: &str) -> Self {}
}
