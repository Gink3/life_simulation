
// external crates
use std::fs;
//use std::io::Write;
//use std::io::prelude::*;
use rand::Rng;
use serde::{Serialize, Deserialize};

// Perlin noise libraries
use noise::Fbm;
use noise::Perlin;
use noise::utils::PlaneMapBuilder;
use noise::utils::NoiseMapBuilder;
use noise::Seedable;

// Image libraries
use image::{RgbImage, Rgb};

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
        // initialize rng
        let mut rng = rand::thread_rng();
        
        // initialize perlin noise generators
        let perlin = Perlin::new().set_seed(rng.gen());

        // Optional fbm noise generator
        let fbm = Fbm::new().set_seed(rng.gen());

        let nmap = PlaneMapBuilder::new(&fbm)
            .set_size(xdim, ydim)
            .set_x_bounds(-5.0, 5.0)
            .set_y_bounds(-5.0, 5.0)
            .build();

        // Not working currently
        // Save raw noise to image
        // nmap.write_to_file("sim_out\\noisemap.png");

        // Create image buffer
        let mut img = RgbImage::new(xdim as u32, ydim as u32);

        // Creates tiles by row then appends row to 
        for y in 0..ydim {
            let mut tmp_row = Vec::new();
            for x in 0..xdim {
                let v = nmap.get_value(x,y);

                // Iterates over noise map
                // set water
                if v <= 0.005 {
                    img.put_pixel(x as u32,y as u32,Rgb([0,0,255]));
                    tmp_row.push(Tile::water());
                // set grass
                } else if v > 0.005 && v <= 0.4 {
                    img.put_pixel(x as u32,y as u32,Rgb([64, 133, 52]));
                    tmp_row.push(Tile::grass());
                // set mountain
                } else if v > 0.4 {
                    img.put_pixel(x as u32,y as u32,Rgb([127,141,163]));
                    tmp_row.push(Tile::mountain());
                }
            }
            self.map.push(tmp_row);
        }
        img.save("sim_out\\world.png");
    }
    pub fn serialize_world(&self,filename: String) {
        //let mut file = File::open(filename);
        let serialized = serde_json::to_string_pretty(&self.map).unwrap();
        fs::write(filename, serialized).expect("Unable to write file");
    }
    
}