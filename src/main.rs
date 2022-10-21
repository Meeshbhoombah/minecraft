/// Minecraft
/// src/main.rs
///
/// Binary application running random Minecraft utility functions
use std::{
    env,
    fs,
    process
};

use serde::{
    Deserialize
};

mod block;
pub use block::{ 
    Block
};

mod chunk;
pub use chunk::{
    Chunk
};


#[derive(Debug, Deserialize)]
struct World {
    seed: i64,
    markers: Vec<Marker>
}

#[derive(Debug, Deserialize)]
struct Marker {
    name: String,
    x: i64,
    y: Option<i32>,
    z: i64
}


fn main() {
    let mut path = env::current_dir()
        .expect("Could not retrive path to current directory");

    path.push("World.toml");

    let contents = match fs::read_to_string(path) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Could not read World.toml: {:?}", e);
            process::exit(1);
        }
    };

    let world: World = match toml::from_str(&contents) {
        Ok(d) => d,
        Err(e) => {
            eprintln!("Could not load data from World.toml: {:?}", e);
            process::exit(1);
        }
    };

    println!("\n {:#?} \n", world);
}

