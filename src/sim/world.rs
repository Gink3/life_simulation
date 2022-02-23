
// external crates
use std::fs;
//use std::io::Write;
//use std::io::prelude::*;
use rand::Rng;
use serde::{Serialize, Deserialize};

// Perlin noise libraries
use noise::Fbm;
// use noise::Perlin;
use noise::utils::PlaneMapBuilder;
use noise::utils::NoiseMapBuilder;
use noise::Seedable;

// Image libraries
use image::{RgbImage, Rgb};

// File io
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

// internal modules
mod tile;
use crate::sim::world::tile::Tile;

#[derive(Serialize, Deserialize, Debug)]
pub struct World {    
    map: Vec<Vec<Tile>>,
}

impl World {
    pub fn new(xdim: usize, ydim: usize,file: String) -> World {
        let mut w = World {
            map: Vec::<Vec::<Tile>>::new(),
        };
        w.initalize(xdim,ydim,file);
        w
    }
    // initalizes the map with tiles
    // uses the food scarcity percentage to determine
    // which tiles will randomly get food allocated to them
    // xdim - x dimension of map
    // ydim - y dimension of map
    // food_output - food scarcity percentage
    pub fn initalize(&mut self,xdim: usize,ydim: usize,filename:String)  {
        // initialize rng
        let mut rng = rand::thread_rng();
        
        // initialize perlin noise generators
        // let perlin = Perlin::new().set_seed(rng.gen());

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

        // various cutoff values of the noise to define the levels at which each
        // type of tile is initially generated
        let water_cutoff = 0.008;
        let beach_cutoff = 0.015;
        let grass_cutoff = 0.35;
        let mountain_cutoff = 0.5;

        // Creates tiles by row then appends row to 
        for y in 0..ydim {
            let mut tmp_row = Vec::new();
            for x in 0..xdim {
                let v = nmap.get_value(x,y);

                // Iterates over noise map
                // set water
                if v <= water_cutoff {
                    img.put_pixel(x as u32,y as u32,Rgb([0,0,255]));
                    tmp_row.push(Tile::water());
                // set beach
                } else if v > water_cutoff && v <= beach_cutoff  {
                    img.put_pixel(x as u32,y as u32,Rgb([252,225,149]));
                    tmp_row.push(Tile::beach());
                // set grass
                } else if v > beach_cutoff && v <= grass_cutoff {
                    img.put_pixel(x as u32,y as u32,Rgb([64, 133, 52]));
                    tmp_row.push(Tile::grass());
                // set mountain
                } else if v > grass_cutoff && v <= mountain_cutoff {
                    img.put_pixel(x as u32,y as u32,Rgb([127,141,163]));
                    tmp_row.push(Tile::mountain());
                // set tall mountain
                } else {
                    img.put_pixel(x as u32,y as u32,Rgb([46,45,44]));
                    tmp_row.push(Tile::tall_mountain());
                }

            }
            // Appends row to map vector
            self.map.push(tmp_row);
        }
        match img.save(filename + ".png") {
            Ok(_v) => (),
            Err(e) => println!("{:?}",e),
        };
    }
    // Loads a serialized json world for repeating
    // simulations on the same world 
    // From https://docs.serde.rs/serde_json/de/fn.from_reader.html
    pub fn load_world<P: AsRef<Path>>(path: P) -> Result<World,Box<dyn Error>>{
        let file = File::open(path)?;
        let reader = BufReader::new(file);

        let u = serde_json::from_reader(reader)?;

        Ok(u)
    }
    // Debug function to output the world in json format
    #[allow(dead_code)]
    pub fn serialize_world(&self,filename: String) {
        // append json file extension
        let json_fn = filename + ".json";
        // let mut file = File::open(filename);
        let serialized = serde_json::to_string_pretty(&self.map).unwrap();
        fs::write(json_fn, serialized).expect("Unable to write file");
    }
    
}