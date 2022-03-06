

use std::fs;

use rand::Rng;
use serde::{Serialize, Deserialize};
//use ron::ser::to_string_pretty;

// Image libraries
use image::{RgbImage, Rgb};

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

#[derive(Serialize, Deserialize, Debug)]
pub struct Sim {
    years: usize,
    days: usize,
    score: i64,
    sim_world: World,
    init_people: usize,
    people: Vec<Person>,
    init_plants: usize,
    plants: Vec<Plant>,
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
                plants: Vec::<Plant>::new(),
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
                plants: Vec::<Plant>::new(),
                // init animals
                init_animals: c.get_init_an(),
                animals: Vec::<Animal>::new(),
                // init people
                init_people: c.get_init_pe(),
                people: Vec::<Person>::new(),
            };
        }
        // Generates entities
        s.generate_init_plants(s.init_plants, c.get_xdim(), c.get_ydim());
        s.generate_init_animals(s.init_animals, c.get_xdim(), c.get_ydim());
        s.generate_init_people(s.init_people, c.get_xdim(), c.get_ydim());

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
                let rand_x = rng.gen_range(0..xdim);
                let rand_y = rng.gen_range(0..ydim);
                // TODO
                // checks if already occupied by a plant

                    match self.sim_world.check_ttype(rand_x,rand_y) {

                        TileType::Grass => {
                            // Debug print statement
                            // println!("Generating berry bush on grass");

                            self.plants.push(Plant::berry_bush(rand_x, rand_y));
                            on_land = true;
                        }
                        TileType::Mountain => {
                            // Debug print statement
                            // println!("Generating berry bush on mountain");

                            self.plants.push(Plant::berry_bush(rand_x, rand_y));
                            on_land = true;
                        }
                        _ => (),
                    }
            }
        }
    }
    // Generates initial animals to simplify simulation initalization
    // na - number of inital animals
    // xdim - x dimension of world
    // ydim - y dimension of world
    fn generate_init_animals(&mut self,na: usize, xdim: usize,ydim: usize) {
        let mut rng = rand::thread_rng();

        // Loop to create X number of plants
        // where x is defined in init_plants
        for _i in 0..na 
        {
            let mut on_land: bool = false;
            while !on_land 
            {
                let rand_x = rng.gen_range(0..xdim);
                let rand_y = rng.gen_range(0..ydim);
                // TODO
                // checks if already occupied by a plant

                    match self.sim_world.check_ttype(rand_x,rand_y) 
                    {

                        TileType::Grass => 
                        {
                            self.animals.push(Animal::rabbit(rand_x, rand_y));
                            on_land = true;
                        }
                        TileType::Mountain => 
                        {
                            self.animals.push(Animal::wolf(rand_x, rand_y));
                            on_land = true;
                        }
                        _ => (),
                    }
            }
        }
    }

    // Generates initial people to simplify simulation initalization
    // np - number of inital people
    // xdim - x dimension of world
    // ydim - y dimension of world
    fn generate_init_people(&mut self,np: usize, xdim: usize,ydim: usize)
    {
        let mut rng = rand::thread_rng();

        // Loop to create X number of plants
        // where x is defined in init_plants
        for _i in 0..np 
        {
            let mut on_land: bool = false;
            while !on_land 
            {
                let rand_x = rng.gen_range(0..xdim);
                let rand_y = rng.gen_range(0..ydim);
                // TODO
                // checks if already occupied by a plant

                    match self.sim_world.check_ttype(rand_x,rand_y) 
                    {

                        TileType::Grass => 
                        {
                            self.people.push(Person::new(self.people.len(),rand_x, rand_y));
                            on_land = true;
                        }
                        TileType::Mountain => 
                        {
                            self.people.push(Person::new(self.people.len(),rand_x, rand_y));
                            on_land = true;
                        }
                        _ => (),
                    }
            }
        }
    }

    pub fn run(&self, days: usize)
    {
        for d in 0..days
        {
            if d % 100 == 0
            {
                // TODO get snapshot of world
                self.snapshot(d);
            }
        }
    }

    fn snapshot(&self, day_count: usize)
    {
        let mut filename = "sim_out\\snapshot-".to_string();
        filename += &day_count.to_string();
        let mut img = RgbImage::new(
            self.sim_world.get_xdim() as u32,
            self.sim_world.get_ydim() as u32);

        // Draw the terrain
        for y in 0..self.sim_world.get_ydim()
        {
            for x in 0..self.sim_world.get_xdim()
            {
                match self.sim_world.check_ttype(x,y)
                {
                    TileType::Beach => img.put_pixel(x as u32,y as u32,Rgb([252,225,149])),
                    TileType::Grass => img.put_pixel(x as u32,y as u32,Rgb([64, 133, 52])),
                    TileType::Mountain => img.put_pixel(x as u32,y as u32,Rgb([127,141,163])),
                    TileType::TallMountain => img.put_pixel(x as u32,y as u32,Rgb([46,45,44])),
                    TileType::Water => img.put_pixel(x as u32, y as u32, Rgb([0,0,255])),
                    _ => (),
                }
            }
        }

        // Overlay entites
        // RGB codes
        // Animals - 79, 6, 21
        // Plants - 17, 77, 7
        // People - 25, 225, 247
        for a in &self.animals
        {
            let x = a.get_x();
            let y = a.get_y();

            img.put_pixel(x as u32, y as u32, Rgb([79,6,21]));
        }
        for p in &self.plants
        {
            let x = p.get_x();
            let y = p.get_y();

            img.put_pixel(x as u32, y as u32, Rgb([17,77,7]));
        }

        for p in &self.people
        {
            let x = p.get_x();
            let y = p.get_y();

            img.put_pixel(x as u32, y as u32, Rgb([25,225,247]));
        }

        match img.save(filename + ".png") {
            Ok(_v) => (),
            Err(e) => println!("{:?}",e),
        };
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
        let serialized = match serde_json::to_string_pretty(&self)
        {
            Ok(s) => s,
            Err(error) => panic!("Problem serializing sim: {:?}",error),
        };
            fs::write("sim_debug.json".to_string(),serialized).expect("Unable to write file")
    }
}