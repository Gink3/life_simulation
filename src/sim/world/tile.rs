
// external module
use serde::{Serialize, Deserialize};

// internal modules
//mod person;
//use crate::person::Person;

#[derive(Serialize, Deserialize, Debug)]
pub struct Tile {
    Ttype: String,
    FoodOutput: usize,
}

impl Tile {
    pub fn new(fo:usize) -> Tile {
        Tile {
            Ttype: "Empty".to_string(),
            FoodOutput: fo,
        }
    }    
}