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

fn main() {
    let t_h = Block::new(6042, None, 910);
    println!("{:#?}", t_h);

    let c_new = Chunk::new_with_block(t_h);
    println!("{:#?}", c_new);
}

