mod scene;
mod engine;
mod text;
mod app;

use scene::Scene;
use clap::Parser;
use std::path::PathBuf;

/// A lightweight engine that runs interactive stories written in TOML.
#[derive(Parser, Debug)]
#[command(name = "StoryEngine")]
#[command(author = "LunaSolDev")]
#[command(version = "Alpha 0.1")]
#[command(about = "A lightweight loader for TOML based text-adventures", long_about=None)]
struct Args {
    #[arg(short, long, value_name = "DIR", default_value = "./stories")]
    stories_dir: PathBuf
}

fn main() {
    let args = Args::parse();

    app::run(args.stories_dir)
}
