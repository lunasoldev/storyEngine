use crate::scene::{Action, Story, ActionType};
use crate::Scene;

use std::fs;
use std::collections::HashMap;
use std::path::Path;

/// Loads a .story file
pub fn load_story<P: AsRef<Path>>(path: P) -> HashMap<String, Scene> {
    let content = fs::read_to_string(path).expect("Failed to load file!");

    let story: Story = toml::from_str::<Story>(&content).expect("Failed to parse story!");
    
    let mut scene_map = HashMap::new();

    for scene in story.scene {
        scene_map.insert(scene.id.clone(), scene);
    }

    scene_map
}

pub fn execute_action(action: &Option<Action>, mut vars: HashMap<String, usize>) -> HashMap<String, usize> {
    let act = action.clone().unwrap();
    let name = act.variable.to_string();
    let action_type = &act.action_type;
    let mut value = act.value;

    match action_type {
        ActionType::Set => {
            vars.insert(name, value);
        },
        ActionType::Add => {
            value = vars.get(&name).unwrap().saturating_add(value);
            vars.insert(name, value);
        },
        ActionType::Subtract => {
            value = vars.get(&name).unwrap().saturating_sub(value);
            vars.insert(name, value);
        }
    }
    vars
}
