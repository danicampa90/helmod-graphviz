mod productionchainparser;
pub use productionchainparser::ConversionError;

type ItemQtyList = Vec<(String, f64)>;

#[derive(Debug)]
pub struct ProductionChain {
    pub products: Vec<(String, f64)>,
    pub ingredients: Vec<(String, f64)>,
    pub blocks: Vec<ProductionBlock>,
}

#[derive(Debug)]
pub struct ProductionBlock {
    pub id: String,
    pub name: String,
    pub power: i32,
    pub ingredients: Vec<(String, i32)>,
    pub recipes: Vec<()>,
}

impl ProductionChain {}
