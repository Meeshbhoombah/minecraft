use crate::{
    Item
};

struct Redstone {
    dust: Item,
    torches: Item,
    piston: Item, 
    sticky_piston: Item,
}

pub struct Build {
    primary: Option<Item>,
    accents: Option<Vec<Item>>,
    redstone: Option<Redstone>
}


