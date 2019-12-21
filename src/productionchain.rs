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
    pub count: f64,
    pub ingredients: Vec<(String, i32)>,
    pub recipes: Vec<ProductionRecipe>,
}

#[derive(Debug)]
pub struct ProductionRecipe {
    pub id: String,
    pub name: String,
    pub type_: String,
    pub output_count: f64,
    pub speed: f64,
    // factory stuff
    pub factory_name: String,
    pub factory_count: f64,
    pub factory_speed: f64,
    pub modules: Vec<()>,
    pub beacons: Vec<()>,
}

impl ProductionChain {}
