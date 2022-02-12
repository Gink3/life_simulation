
// external module
use serde::{Serialize, Deserialize};

// internal modules
//mod person;
//use crate::person::Person;

#[derive(Serialize, Deserialize, Debug)]
pub struct Tile {
    ttype: String,
    food_output: usize,
}

impl Tile {
    pub fn new(fo:usize) -> Tile {
        Tile {
            ttype: "Empty".to_string(),
            food_output: fo,
        }
    }    
}