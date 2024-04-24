use engine::Engine;
use genesis::{
    genesis::my_input_handler,
    player::{self, Player},
};

mod engine;
mod genesis;

//TODO: clear resources in event loop run (verify)
//TODO: windowed and full-screen feature
//TODO: Draw player to screen and movement
//TODO: Abstarct drawing defaults
//TODO: Turn engine into a lib

fn main() {
    Engine::new("Genesis")
        .with_clear_color(wgpu::Color::BLACK)
        .with_input_system(my_input_handler)
        .run();
}
