
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
        let mut s: Sim;
        // Load world sim
        if c.will_load_world() {
            s = Sim {
                years: 0,
                days: 0,
                score: 0,
                sim_world: World::load_world(c.get_world_filename()).unwrap(),
                people: Vec::<Person>::new(),
                plants: Vec::<Plant>::new(),
                animals: Vec::<Animal>::new(),
            };
        // Generate new world sim
        } else {
            s = Sim {
                years: 0,
                days: 0,
                score: 0,
                sim_world: World::new(c.get_xdim(),c.get_ydim(),c.get_world_filename().to_string()),
                people: Vec::<Person>::new(),
                plants: Vec::<Plant>::new(),
                animals: Vec::<Animal>::new(),
            };
        }
        // Generates people into person vector
        for _count in 0..c.get_sp() {
            s.people.push(Person::new(s.people.len()));
        }

        s
    }
    // writes the simulation as a ron file
    #[allow(dead_code)]
    pub fn sim_debug_ron(&self) {
        let serialized = ron::to_string(&self).unwrap();
        fs::write("sim_debug.ron",serialized).expect("Unable to write file")
    }
    // writes the simulation as a json file
    #[allow(dead_code)]
    pub fn sim_debug_json(&self) {
        let serialized = serde_json::to_string_pretty(&self).unwrap();
        fs::write("sim_debug.json".to_string(),serialized).expect("Unable to write file")
    }
}