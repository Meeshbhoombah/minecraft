/// Minecraft
/// 
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


mod item;
mod block;
mod chunk;
mod world;
mod build;

mod castle;

pub use item::{
    Count,
    Item
};

pub use block::{ 
    Block
};

pub use chunk::{
    Chunk
};

pub use world::{
    World
};


#[derive(Debug, Deserialize)]
struct Marker {
    name: String,
    x: i64,
    y: Option<i32>,
    z: i64
}

impl Marker {
    pub fn to_block(&self) -> (&String, Block) { 
        let x = self.x as i32;

        let y = if let Some(y) = self.y {
            Some(y as i32)
        } else {
            None
        };

        let z = self.z as i32;

        (&self.name, Block::new(x, y, z))
    }
}

#[derive(Debug, Deserialize)]
struct RawWorldData {
    seed: Option<i64>,
    markers: Vec<Marker>
}

impl RawWorldData {
    pub fn make_workable(&self) -> World {
        World {
            seed: self.seed,
            structures: None,
        }
    }
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

    let raw: RawWorldData = match toml::from_str(&contents) {
        Ok(d) => d,
        Err(e) => {
            eprintln!("Could not load data from World.toml: {:?}", e);
            process::exit(1);
        }
    };

    for marker in &raw.markers {
        let b = marker.to_block();
        println!("\n {:#?} \n", marker);
        println!("\n {:#?} \n", b); 
    }

    let w = raw.make_workable();
    println!("\n {:#?} \n", w); 
}

