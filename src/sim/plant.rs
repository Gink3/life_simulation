

use serde::{Serialize, Deserialize};

#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct Plant 
{
    species: String,
    edible: bool,
    servings: usize,
    can_move_through: bool,
    x: usize,
    y: usize,
}

impl Plant 
{
    pub fn berry_bush(xc: usize,yc: usize) -> Plant 
    {
        Plant 
        {
            species: "Berry Bush".to_string(),
            edible: true,
            servings: 4,
            can_move_through: false,
            x: xc,
            y: yc,
        }
    }
    #[allow(dead_code)]
    pub fn get_species(self) -> String 
    {
        self.species
    }

    pub fn get_x(&self) -> usize {
        self.x
    }
    pub fn get_y(&self) -> usize {
        self.y
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_berry_bush() 
    {
        let bb = Plant::berry_bush(0,0);

        assert_eq!(bb.get_species(), "Berry Bush");
    }
    #[test]
    fn has_correct_coords()
    {
        let bb = Plant::berry_bush(0, 0);

        assert_eq!(bb.get_x(), 0);
    }
}