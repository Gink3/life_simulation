
// external module
use serde::{Serialize, Deserialize};

// internal modules
//mod person;
//use crate::person::Person;

#[derive(Serialize, Deserialize, Debug)]
enum Tile_type {
    WATER,
    GRASS,
    MOUNTAIN,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Tile {
    ttype: Tile_type,
    food_output: usize,
}

impl Tile {
    pub fn water(fo:usize) -> Tile {
        Tile {
            ttype: Tile_type::WATER,
            food_output: fo,
        }
    }
    pub fn grass(fo:usize) -> Tile {
        Tile {
            ttype: Tile_type::GRASS,
            food_output: fo,
        }
    }    
    pub fn mountain(fo:usize) -> Tile {
        Tile {
            ttype: Tile_type::MOUNTAIN,
            food_output: fo,
        }
    }    
}