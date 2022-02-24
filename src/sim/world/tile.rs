
// external module
use serde::{Serialize, Deserialize};

// internal modules
//mod person;
//use crate::person::Person;

// tile types
#[derive(Serialize, Deserialize, Debug)]
enum TileType {
    Water,
    Beach,
    Grass,
    Mountain,
    TallMountain,
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
            ttype: TileType::Water,
            solid: false,
        }
    }
    pub fn beach() -> Tile {
        Tile {
            ttype: TileType::Beach,
            solid: false,
        }
    }
    pub fn grass() -> Tile {
        Tile {
            ttype: TileType::Grass,
            solid: false,
        }
    }    
    pub fn mountain() -> Tile {
        Tile {
            ttype: TileType::Mountain,
            solid: true,
        }
    }
    pub fn tall_mountain() -> Tile {
        Tile {
            ttype: TileType::TallMountain,
            solid: true,
        }
    }
}