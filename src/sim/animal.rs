
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug,Clone)]
enum Size {
    XL,
    L,
    M,
    S,
    XS,
}



#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Animal {
    species: String,
    health: i32,
    hunger: i32,
    size: Size,
    x: usize,
    y: usize,
}

impl Animal {
    pub fn rabbit(xc: usize, yc: usize) -> Animal {
        Animal {
            species: "Rabbit".to_string(),
            health: 5,
            hunger: 100,
            size: Size::S,
            x: xc,
            y: yc,
        }
    }
    pub fn wolf(xc: usize, yc: usize) -> Animal {
        Animal {
            species: "Wolf".to_string(),
            health: 20,
            hunger: 100,
            size: Size::M,
            x: xc,
            y: yc,
        }
    }
    pub fn get_x(&self) -> usize
    {
        self.x
    }
    pub fn get_y(&self) -> usize
    {
        self.y
    }
    #[allow(dead_code)]
    pub fn get_species(self) -> String {
        self.species
    }

}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    #[test]
    fn is_rabbit() {
        let r = Animal::rabbit(0,0);

        assert_eq!(r.get_species(),"Rabbit".to_string());
    }

    #[test]
    fn is_wolf() {
        let w = Animal::wolf(0,0);

        assert_eq!(w.get_species(),"Wolf".to_string());
    }

}