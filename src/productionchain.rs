pub struct ProductionChain {
    pub id: String,
    pub owner: String,
    pub blocks: Vec<ProductionBlock>,
}

pub struct ProductionBlock {
    pub id: String,
    pub name: String,
    pub power: i32,
    pub ingredients: Vec<(String, i32)>,
    pub recipes: Vec<()>,
}

impl ProductionChain {}
