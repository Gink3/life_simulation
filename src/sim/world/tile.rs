
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
}

impl Tile {
    pub fn water() -> Tile {
        Tile {
            ttype: Tile_type::WATER,
            
        }
    }
    pub fn grass() -> Tile {
        Tile {
            ttype: Tile_type::GRASS,
            
        }
    }    
    pub fn mountain() -> Tile {
        Tile {
            ttype: Tile_type::MOUNTAIN,
            
        }
    }    
}