use std::collections::HashMap;

use serde::{Serialize, Deserialize};

use super::plant::Plant;

// PlantController handles dealing with the creation, deletion, reproduction,
// and movement of the plants
// QUESTION could I use a typedef instead?
#[derive(Serialize, Deserialize, Debug)]
pub struct PlantController {
    plants: HashMap<(usize,usize),Plant>,
}

impl PlantController {
    pub fn new() -> PlantController 
    {
        PlantController 
        {
            plants: HashMap::new(),
        }
    }
    // Inserts a new plant with a coordinate pair (x, y)
    // x - coordinate
    // 
    //
    //
    pub fn insert_new_plant(&mut self, x: usize, y: usize, p: Plant) 
    {
            // create tuple
            let coords = (x,y);
            // insert into Hashmap
            self.plants.insert(coords, p);
    }
    pub fn has_plant_at(&self, x: usize, y: usize) -> bool 
    {
        let coords = (x,y);
        self.plants.contains_key(&coords)
    }
    pub fn update_coord() 
    {
        todo!()
    }
    // Removes plant with coordinates x,y
    //
    //
    pub fn remove_plant_at(&mut self, x: usize, y: usize) 
    {
        if self.has_plant_at(x, y) 
        {
            let coords = (x,y);
            self.plants.remove(&coords);
        }
    }
    pub fn get_plant_count(&self) -> usize 
    {
        self.plants.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    //
    #[test]
    fn pc_add_1_to_hashmap() 
    {
        let mut pc = PlantController::new();
        let test_plant = Plant::berry_bush();
        
        pc.insert_new_plant(0, 0, test_plant);

        assert!(pc.has_plant_at(0, 0));
    }
    //
    #[test]
    fn pc_add_10_to_hashmap() 
    {
        let mut pc = PlantController::new();

        for i in 0..10 
        {
            let test_plant = Plant::berry_bush();
            pc.insert_new_plant(i, 0, test_plant);
        }

        assert_eq!(pc.get_plant_count(), 10);
    }
    //
    #[test]
    fn pc_init_to_0() 
    {
        let pc = PlantController::new();
        assert_eq!(pc.get_plant_count(), 0);
    }
    //
    #[test]
    fn pc_can_remove_plant() 
    {
        let mut pc = PlantController::new();
        let test_plant = Plant::berry_bush();
        
        pc.insert_new_plant(0, 0, test_plant);
        pc.remove_plant_at(0, 0);

        assert_eq!(pc.get_plant_count(), 0);
    }
    /*
    // ret
    #[test]
    fn pc_remove_nonexistant_plant() {

    }
    */
}