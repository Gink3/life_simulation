
use std::collections::HashMap;
use std::fs;

use rand::Rng;
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
use crate::sim::world::tile::TileType;

mod plant_controller;
use plant_controller::PlantController;

#[derive(Serialize, Deserialize, Debug)]
pub struct Sim {
    years: usize,
    days: usize,
    score: i64,
    sim_world: World,
    init_people: usize,
    people: Vec<Person>,
    init_plants: usize,
    plants: PlantController,
    init_animals: usize,
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
                // TODO inline if?
                sim_world: World::load_world(c.get_world_filename()).unwrap(),
                // init plants
                init_plants: c.get_init_pl(),
                plants: PlantController::new(),
                // init animals
                init_animals: c.get_init_an(),
                animals: Vec::<Animal>::new(),
                // init people
                init_people: c.get_init_pe(),
                people: Vec::<Person>::new(),
            };
        // Generate new world sim
        } else {
            s = Sim {
                years: 0,
                days: 0,
                score: 0,
                sim_world: World::new(c.get_xdim(),c.get_ydim(),c.get_world_filename().to_string()),
                // init plants
                init_plants: c.get_init_pl(),
                plants: PlantController::new(),
                // init animals
                init_animals: c.get_init_an(),
                animals: Vec::<Animal>::new(),
                // init people
                init_people: c.get_init_pe(),
                people: Vec::<Person>::new(),
            };
        }
        // Generates plants
        s.generate_init_plants(s.init_plants, c.get_xdim(), c.get_ydim());
        // Generates people into person vector
        // TODO animal generation goes here
        // TODO move people generation to its own function
        for _count in 0..c.get_init_pe() {
            s.people.push(Person::new(s.people.len()));
        }

        s
    }
    // Generates the initial plant life for the world
    // np - number of plants
    // xdim - x dimension
    // ydim - y dimension
    fn generate_init_plants(&mut self,np: usize, xdim: usize,ydim: usize) {
        let mut rng = rand::thread_rng();

        // Loop to create X number of plants
        // where x is defined in init_plants
        for _i in 0..np {
            let mut on_land: bool = false;
            while !on_land {
                let rand_x = rng.gen_range(0..=xdim);
                let rand_y = rng.gen_range(0..=ydim);
                // TODO
                // checks if already occupied by a plant

                    match self.sim_world.check_ttype(rand_x,rand_y) {

                        TileType::Grass => {
                            // Debug print statement
                            // println!("Generating berry bush on grass");

                            self.plants.push(Plant::berry_bush());
                            on_land = true;
                        }
                        TileType::Mountain => {
                            // Debug print statement
                            // println!("Generating berry bush on mountain");

                            self.plants.push(Plant::berry_bush());
                            on_land = true;
                        }
                        _ => (),
                    }
            }
        }
    }
    // Print entity stats
    pub fn print_entity_stats(&self) {
        println!("Plant count: {:?}",self.plants.len());
        println!("Animal count: {:?}",self.animals.len());
        println!("Person count: {:?}",self.people.len());
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