use std::collections::HashMap;
use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;

use crate::{engine, Scene};


pub struct App {
    scenes: HashMap<String, Scene>,
    current_id: String,
}

pub fn run(dir: PathBuf) {
    let mut app = App {
        scenes: HashMap::new(),
        current_id: "start".to_string(),
    };
    app.init(dir);
}

impl App {
    /// Main loop function
    fn main_loop(&mut self) {
        let mut variables = HashMap::new();
        loop {
            // Load the current scene
            let current = &self.scenes.get(&self.current_id).unwrap();
            println!("{}", current.text);
            
            // Allow user to make choices
            for (i, choice) in current.choices.iter().enumerate() {
                println!("{}: {}", i + 1, choice.text);
            }

            print!("> ");
            io::stdout().flush().unwrap();

            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            input = input.trim().to_string();


            let index: usize = if input.is_empty() { 1 } else { input.parse().unwrap_or(0) };

            if index == 0 || index > current.choices.len() {
                println!("Invalid choice.");
                continue;
            }

            let choice = &current.choices[index - 1];

            if choice.target == "end" {
                break;
            }

            if choice.action.is_some() {
                variables = engine::execute_action(&choice.action, variables)
            }

            for var in variables.keys() {
                println!("{}", format!("{}: {}", var, variables.get(var).unwrap()));
            }

            self.current_id = choice.target.to_string();
        }
    }

    fn init(&mut self, dir: PathBuf) {
        println!("Welcome to StoryEngine, please select a story!");

        // Let the user choose a story
        let mut stories = Vec::new();

        for (i, entry) in fs::read_dir(dir).expect("Could not read the folder!").enumerate() {
            let entry = entry.expect("");
            let path = entry.path();
            if path.extension().map(|e| e == "story" || e == "toml").unwrap_or(false) {
                println!("{}: {:?}", i + 1, path.file_name().unwrap());
                stories.push(path);
            }
        }

        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input: usize = input.trim().parse().unwrap_or(0);

        if input == 0 || input > stories.len() {
            println!("Invalid option!");
        }

        let selected_story = &stories[input - 1];
        println!("Loading {:?}", selected_story.file_name().unwrap());

        self.scenes = engine::load_story(selected_story);

        self.main_loop();
    }
}
