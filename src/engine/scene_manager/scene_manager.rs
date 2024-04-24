use crate::engine::scene::Scene;

pub struct SceneManager {
    scenes: Vec<Box<Scene>>,
    active_scene: Option<usize>,
}

impl SceneManager {
    pub fn new() -> Self {
        SceneManager {
            scenes: Vec::new(),
            active_scene: None,
        }
    }

    pub fn add_scene(&mut self, scene: Scene) {
        self.scenes.push(Box::new(scene));
    }

    pub fn set_active_scene(&mut self, label: &str) {
        let index = self.get_scene(label);
        self.active_scene = index;
    }

    fn get_scene(&self, label: &str) -> Option<usize> {
        for (index, scene) in self.scenes.iter().enumerate() {
            if scene.label == label {
                return Some(index);
            }
        }

        None
    }
}
