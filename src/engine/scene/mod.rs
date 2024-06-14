use winit::{event::WindowEvent, event_loop::EventLoopWindowTarget};

use super::graphics::Graphics;

///The Scene trait holds the functionalities related to each scene:
///- `init`: Gets called once and is ment to allocate the required resources and run initialization
///logic
///- `input`: Gets called every loop with an accessible `WindowEvent`
///- `update`: Gets called every loop with an accessible `delta_time`
///- `draw`: Gets called every loop with access to the `graphics`
///- `cleanup`: Gets called once when switching to a diffent scene.
///- `is_initialized`: Is a flag that inidicates if the `init` function has been called or not
pub trait Scene {
    fn init(&mut self, graphics: &mut Graphics);
    fn input(&mut self, event: &WindowEvent, target: &EventLoopWindowTarget<()>);
    fn update(&mut self, delta_time: f32);
    fn draw(&self, graphics: &mut Graphics);
    fn cleanup(&mut self);
    fn is_initialized(&self) -> bool;
}
