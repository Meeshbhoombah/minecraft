use std::{
    collections::{
        HashMap
    }
};

#[derive(Debug)]
pub enum Structure {
    Biome,

    Generated,

    Base,
    Farm,
    Quarry,
    Mine
}

#[derive(Debug)]
pub struct World {
    pub seed: Option<i64>,
    pub structures: Option<HashMap<String, Structure>>
}


