#[derive(Debug)]
pub struct Block {
    /// We use `i32` to eclipse the largest coordinate possible for a Minecraft
    /// block -- the World Border is located at X/Z ±29,999,984
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

