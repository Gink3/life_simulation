use std::collections::HashMap;

use serde::{Serialize, Deserialize};

use super::plant::Plant;



#[derive(Serialize, Deserialize, Debug)]
pub struct PlantController {
    plants: HashMap<(usize,usize),Plant>,
}

impl PlantController {
    pub fn new() -> PlantController {
        PlantController {
            plants: HashMap::new(),
        }
    }
}