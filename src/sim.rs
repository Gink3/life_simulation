
use std::fs;

use serde::{Serialize, Deserialize};
//use ron::ser::to_string_pretty;

// internal modules-
pub mod world;
use crate::sim::world::World;

mod person;
use person::Person;

mod plant;
use plant::Plant;

mod animal;
use animal::Animal;

use crate::config::Config;

#[derive(Serialize, Deserialize, Debug)]
pub struct Sim {
    years: usize,
    days: usize,
    score: i64,
    sim_world: World,
    people: Vec<Person>,
    plants: Vec<Plant>,
    animals: Vec<Animal>,
}

impl Sim {
    pub fn new(c:Config) -> Sim {
        Sim {
            years: 0,
            days: 0,
            score: 0,
            sim_world: World::new(c.get_xdim(),c.get_ydim(),c.get_fs()),
            people: Vec::<Person>::new(),
            plants: Vec::<Plant>::new(),
            animals: Vec::<Animal>::new(),
        }
    }
    pub fn sim_debug_ron(&self,filename: String) {
        let serialized = ron::to_string(&self).unwrap();
        fs::write(filename,serialized).expect("Unable to write file")
    }
}