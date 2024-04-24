use engine::{scene::Scene, Engine};

mod engine;
mod genesis;

//TODO: clear resources in event loop run (verify)
//TODO: windowed and full-screen feature
//TODO: Draw player to screen and movement
//TODO: Abstarct drawing defaults
//TODO: Turn engine into a lib
//TODO: Scene Manager

fn main() {
    //Engine::new("Genesis").add_scene(some_scene).add_scenes([some_scene, some_other_scene]).run()
    let test_scene = Scene::new("TestScene");
    Engine::new("Genesis").add_scene(test_scene).run();
}
