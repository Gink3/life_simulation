
#[derive(Debug,Clone)]
enum Size {
    XL,
    L,
    M,
    S,
    XS,
}

use crate::sim::animal::Size::*;

#[derive(Debug,Clone)]
pub struct Animal {
    species: String,
    health: i32,
    hunger: i32,
    size: Size
    
}

impl Animal {
    pub fn rabbit() -> Animal {
        Animal {
            species: "Rabbit".to_string(),
            health: 5,
            hunger: 100,
            size: S,
        }
    }
    pub fn wolf() -> Animal {
        Animal {
            species: "Wolf".to_string(),
            health: 20,
            hunger: 100,
            size: M,
        }
    }

}