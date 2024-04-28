use engine::Engine;
use genesis::genesis::GameState;

mod engine;
mod genesis;

//TODO: clear resources in event loop run (verify)
//TODO: windowed and full-screen feature
//TODO: Draw player to screen and movement
//TODO: Abstarct drawing defaults
//TODO: Turn engine into a lib
//TODO: Shaders, feature

fn main() {
    Engine::new("Genesis")
        .add_active_scene(GameState::GamePlay.init())
        .run();
}
