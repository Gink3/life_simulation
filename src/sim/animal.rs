
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
            size: Size::S,
        }
    }
    pub fn wolf() -> Animal {
        Animal {
            species: "Wolf".to_string(),
            health: 20,
            hunger: 100,
            size: Size::M,
        }
    }
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
        let r = Animal::rabbit();

        assert_eq!(r.get_species(),"Rabbit".to_string());
    }

    #[test]
    fn is_wolf() {
        let w = Animal::wolf();

        assert_eq!(w.get_species(),"Wolf".to_string());
    }

}