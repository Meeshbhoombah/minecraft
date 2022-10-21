/// Minecraft
/// src/chunk.rs
use crate::{
    Block,
    block::SEA_LEVEL,
};

pub const iCHUNK_SIDE_LENGTH: i32 = 16;
pub const fCHUNK_SIDE_LENGTH: f32 = 16.0;

fn nw(b: &Block) -> Block {
    let x = b.x as f32 / fCHUNK_SIDE_LENGTH;
    let x = x.floor();
    let x = x as i32;
    let x = x * iCHUNK_SIDE_LENGTH;

    let y = SEA_LEVEL;

    let z = b.z as f32 / fCHUNK_SIDE_LENGTH;
    let z = z.floor();
    let z = z as i32;
    let z = z * iCHUNK_SIDE_LENGTH;

    Block {
        x,
        y,
        z
    }
}

fn ne(b: &Block) -> Block {
    let x = b.x as f32 / fCHUNK_SIDE_LENGTH;
    let x = x.ceil();
    let x = x as i32;
    let x = x * iCHUNK_SIDE_LENGTH;

    let y = SEA_LEVEL;

    let z = b.z as f32 / fCHUNK_SIDE_LENGTH;
    let z = z.floor();
    let z = z as i32;
    let z = z * iCHUNK_SIDE_LENGTH;

    Block {
        x,
        y,
        z
    }
}

fn se(b: &Block) -> Block {
    let x = b.x as f32 / fCHUNK_SIDE_LENGTH;
    let x = x.ceil();
    let x = x as i32;
    let x = x * iCHUNK_SIDE_LENGTH;

    let y = SEA_LEVEL;

    let z = b.z as f32 / fCHUNK_SIDE_LENGTH;
    let z = z.ceil();
    let z = z as i32;
    let z = z * iCHUNK_SIDE_LENGTH;

    Block {
        x,
        y,
        z
    }
}

fn sw(b: &Block) -> Block {
    let x = b.x as f32 / fCHUNK_SIDE_LENGTH;
    let x = x.floor();
    let x = x as i32;
    let x = x * iCHUNK_SIDE_LENGTH;

    let y = SEA_LEVEL;

    let z = b.z as f32 / fCHUNK_SIDE_LENGTH;
    let z = z.ceil();
    let z = z as i32;
    let z = z * iCHUNK_SIDE_LENGTH;

    Block {
        x,
        y,
        z
    }
}

#[derive(Debug)]
pub struct Chunk {
    pub nw: Block,
    pub ne: Block,
    pub se: Block,
    pub sw: Block,
}

impl Chunk {
    pub fn new_with_block(b: Block) -> Self {
        let nw = nw(&b);
        let ne = ne(&b);
        let se = se(&b);
        let sw = sw(&b);

        Chunk {
            nw,
            ne,
            se,
            sw,
        }
    }
}

