pub type Count = i64;

pub struct Item {
    name: String,
    recipie: Option<Vec<Item>>,
    count: Count
}

