use engine::Engine;
use winit::{
    event_loop::EventLoopWindowTarget,
    keyboard::{KeyCode, PhysicalKey},
};
mod engine;

//TODO: clear resources in event loop run (verify)
//TODO: windowed and full-screen feature
//TODO: draw to screen

fn my_input_handler(input_key: &PhysicalKey, target: &EventLoopWindowTarget<()>) {
    println!("Handling input {:?}", input_key);
    match input_key {
        PhysicalKey::Code(key_code) => match key_code {
            KeyCode::Escape => {
                target.exit();
            }
            _ => (),
        },
        _ => (),
    }
}

fn main() {
    Engine::new("Genesis")
        .with_clear_color(wgpu::Color::BLACK)
        .with_input(my_input_handler)
        .run();
}
