
// external crates
use std::fs;
//use std::io::Write;
//use std::io::prelude::*;
use rand::Rng;
use serde::{Serialize, Deserialize};

// internal modules
mod tile;
use crate::sim::world::tile::Tile;

#[derive(Serialize, Deserialize, Debug)]
pub struct World {    
    map: Vec<Vec<Tile>>,
}

impl World {
    pub fn new(xdim: usize, ydim: usize, food_output: usize) -> World {
        let mut w = World {
            map: Vec::<Vec::<Tile>>::new(),
        };
        w.initalize(xdim,ydim,food_output);
        w
    }
    // initalizes the map with tiles
    // uses the food scarcity percentage to determine
    // which tiles will randomly get food allocated to them
    // xdim - x dimension of map
    // ydim - y dimension of map
    // food_output - food scarcity percentage
    pub fn initalize(&mut self,xdim: usize,ydim: usize, food_output: usize)  {
        let mut rng = rand::thread_rng();
        
        // Creates tiles by row then appends row to 
        for _y in 0..ydim {
            let mut tmp_row = Vec::new();
            for _x in 0..xdim {
                let mut food = 0_usize;
                if rng.gen_range(0..100) < food_output{
                    food = rng.gen_range(1..6);
                }
                tmp_row.push(Tile::water(food));
            }
            self.map.push(tmp_row);
        }
    }
    pub fn serialize_world(&self,filename: String) {
        //let mut file = File::open(filename);
        let serialized = serde_json::to_string_pretty(&self.map).unwrap();
        fs::write(filename, serialized).expect("Unable to write file");
    }
    
}