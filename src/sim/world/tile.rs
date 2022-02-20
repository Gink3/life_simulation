
// external module
use serde::{Serialize, Deserialize};

// internal modules
//mod person;
//use crate::person::Person;

// tile types
#[derive(Serialize, Deserialize, Debug)]
enum TileType {
    WATER,
    GRASS,
    MOUNTAIN,
}


// Describes the basic unit of the world
// ttype --> tile type
// solid --> whether something can move through that tile
#[derive(Serialize, Deserialize, Debug)]
pub struct Tile {
    ttype: TileType,
    solid: bool,
}

impl Tile {
    pub fn water() -> Tile {
        Tile {
            ttype: TileType::WATER,
            solid: false,
        }
    }
    pub fn grass() -> Tile {
        Tile {
            ttype: TileType::GRASS,
            solid: false,
        }
    }    
    pub fn mountain() -> Tile {
        Tile {
            ttype: TileType::MOUNTAIN,
            solid: true,
        }
    }    
}