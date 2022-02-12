

#[derive(Debug,Clone)]
pub struct Plant {
    species: String,
    edible: bool,
    servings: usize,
}

impl Plant {
    pub fn berry_bush() -> Plant {
        Plant {
            species: "Berry Bush".to_string(),
            edible: true,
            servings: 4,
        }
    }

    pub fn get_species(self) -> String {
        self.species
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_berry_bush() {
        let bb = Plant::berry_bush();

        assert_eq!(bb.get_species(), "Berry Bush");
    }
}