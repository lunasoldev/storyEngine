# StoryEngine - A simple way to create text adventures

A small, lightweight text-based game engine that lets you define branching stories using the [TOML](https://toml.io/en/) format. No coding required – just write your story in a structured way, and the engine turns it into an interactive narrative!

## Features

- Define stories and choices using TOML files
- Easy-to-write syntax for branching paths
- Minimalist terminal-based game experience
- Fast and portable – ideal for terminal-based text adventures
- Support for variables and ~~conditional choices~~ (W.I.P) to allow for more complex stories

## Getting started

### 1. Clone the project
```bash
git clone https://github.com/lunasoldev/storyEngine.git
cd storyEngine
```

### 2. Build & Run
From inside the storyEngine directory, run:
```bash
cargo run
```
Alternatively you can use
```bash
cargo run -- -stories-dir PATH/TO/STORIES/DIRECTORY/
```
to use a different directory for your story files.

# Story Format
A story is defined using the TOML format and can have the file extensions `.story` or `.toml`
Here is an example of a story file:

```toml
# Every story starts with a scene that has the id "start"
[[scene]]
id = "start"
text = "This is the text shown to the player when the story begins."
choices = [
    { text = "Go to test1", target = "test1" },
    { text = "Go to test2", target = "test2" }
]

# A second scene, reachable from "start"
[[scene]]
id = "test1"
text = "You've reached scene 'test1'."
choices = [
    { text = "End the story", target = "end" }, # Special target: "end" closes the game
    { text = "Go to test2", target = "test2" }
]

# A third scene, looping back or ending
[[scene]]
id = "test2"
text = "This is scene 'test2'."
choices = [
    { text = "End the story", target = "end" },
    { text = "Go to test1", target = "test1" }
]
```

To create your own story files, simply drop the file into the storyEngine/stories folder, or if you are using another folder, in that one.

# Project Ideas & Goals
Some things I would like to add to this project are:
- Full support for custom defined variables and conditional choices
- A save/load system for longer stories
- A better way to create/visualise story files

# Contributing
Feel free to send a PR if you want to support this project!

# License
see [LICENSE](LICENSE)
