
// external module
use serde::{Serialize, Deserialize};

// internal modules
//mod person;
//use crate::person::Person;

#[derive(Serialize, Deserialize, Debug)]
pub struct Tile {
    X: usize,
    Y: usize,
    Ttype: String,
    FoodOutput: usize,
}

impl Tile {
    pub fn new(xidx: usize,yidx: usize,fo:usize) -> Tile {
        Tile {
            X: xidx,
            Y: yidx,
            Ttype: "Empty".to_string(),
            FoodOutput: fo,
        }
    }    
}