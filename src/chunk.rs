/// Minecraft
/// src/chunk.rs
use crate::{
    Block
};


/// The maximum altitude for the Overworld is 320
const MAX_Y_OVERWORLD: i32 = 320;
/// For both the Nether and the End, the maximum altitude is 256
const MAX_Y_OTHERWORLD: i32 = 256;

const SEA_LEVEL: i32 = 62;


const iCHUNK_SIDE_LENGTH: i32 = 16;
const fCHUNK_SIDE_LENGTH: f32 = 16.0;

#[derive(Debug)]
pub struct Chunk {
    pub nw: Block,
    pub ne: Block,
    pub se: Block,
    pub sw: Block,
}

pub fn nw_corner(b: &Block) -> Block {
    let n = b.x as f32 / fCHUNK_SIDE_LENGTH;
    let a = b.z as f32 / fCHUNK_SIDE_LENGTH;
    
    let n = n.floor();
    let a = a.floor();

    let n = n * fCHUNK_SIDE_LENGTH;
    let a = a * fCHUNK_SIDE_LENGTH;

    let x = n as i32;
    let z = a as i32;

    Block {
        x,
        y: SEA_LEVEL,
        z
    }
}

pub fn ne_corner(b: &Block) -> Block {
    let n = nw_corner(&b);

    let x = n.x + iCHUNK_SIDE_LENGTH;
    let y = n.y;
    let z = n.z;

    Block {
        x,
        y,
        z
    }
}

pub fn se_corner(b: &Block) -> Block {
    let n = nw_corner(&b);

    let x = n.x + iCHUNK_SIDE_LENGTH;
    let y = n.y;
    let z = n.z + iCHUNK_SIDE_LENGTH;

    Block {
        x,
        y,
        z
    }
}

pub fn sw_corner(b: &Block) -> Block {
    let n = nw_corner(&b);

    let x = n.x;
    let y = n.y;
    let z = n.z + iCHUNK_SIDE_LENGTH;

    Block {
        x,
        y,
        z
    }     
}

pub fn new_from_block(b: Block) -> Chunk {
    let nw = nw_corner(&b);
    let ne = ne_corner(&b);
    let se = se_corner(&b);
    let sw = sw_corner(&b);

    Chunk {
        nw,
        ne,
        se,
        sw
    } 
}


