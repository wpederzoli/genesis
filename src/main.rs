use engine::Engine;
use genesis::genesis::GameState;

mod engine;
mod genesis;

//TODO: clear resources in event loop run (verify)
//TODO: windowed and full-screen feature
//TODO: Draw player to screen and movement
//TODO: Turn engine into a lib
//TODO: Document
//TODO: Camera controller

fn main() {
    Engine::new("Genesis")
        .add_scene(GameState::GamePlay.init())
        .switch_scene("GamePlay")
        .run();
}
