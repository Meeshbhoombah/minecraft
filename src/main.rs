/// Minecraft
/// src/main.rs
///
/// Binary application running random Minecraft utility functions

mod block;
pub use block::{ 
    Block
};

mod chunk;
pub use chunk::{
    Chunk
};

/*
pub fn make_portal_path(points: Vec<Block>) -> Vec<Block> {
}

pub fn make_water_channel(points: Vec<Block>) -> Vec<Block> {
}
*/

fn main() {
    let tree_house = Block {
        x: 6042,
        y: 78,
        z: 910
    };

    let c = chunk::new_from_block(tree_house);
    println!("{:?}", c);
}

