/// The maximum altitude for the Overworld is 320
pub const MAX_Y_OVERWORLD: i32 = 320;
/// For both the Nether and the End, the maximum altitude is 256
pub const MAX_Y_OTHERWORLD: i32 = 256;

pub const SEA_LEVEL: i32 = 62;

pub const MIN_Y: i32 = 0;


#[derive(Debug)]
pub struct Block {
    /// We use `i32` to eclipse the largest coordinate possible for a Minecraft
    /// block -- the World Border is located at X/Z ±29,999,984
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl Block {
    pub fn new(x: i32, y: Option<i32>, z: i32) -> Self {
        let y = if let None = y {
            SEA_LEVEL 
        } else {
            y.unwrap()
        };

        Self {
            x,
            y,
            z
        }
    }
}

