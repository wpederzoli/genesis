use engine::{graphics::Graphics, scene::Scene, Engine};
use genesis::scenes::gameplay::GamePlay;

mod engine;
mod genesis;

//TODO: clear resources in event loop run (verify)
//TODO: windowed and full-screen feature
//TODO: Draw player to screen and movement
//TODO: Abstarct drawing defaults
//TODO: Turn engine into a lib
//TODO: Scene Manager

fn main() {
    Engine::new("Genesis")
        .add_active_scene(GamePlay::new())
        .run();
}
