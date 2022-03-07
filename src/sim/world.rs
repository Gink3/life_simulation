
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

// num formatter
use num_format::{Locale, ToFormattedString};

// internal modules
pub(crate) mod tile;
use crate::sim::world::tile::Tile;
use crate::sim::world::tile::TileType;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct World {    
    x_dim: usize,
    y_dim: usize,
    total_tiles: usize,
    water_count: usize,
    grass_count: usize,
    mountain_count: usize,
    map: Vec<Vec<Tile>>,
}

impl World {
    pub fn new(xdim: usize, ydim: usize) -> World 
    {
        let mut w = World {
            x_dim: xdim,
            y_dim: ydim,
            total_tiles: ydim * xdim,
            water_count: 0,
            grass_count: 0,
            mountain_count: 0,
            map: Vec::<Vec::<Tile>>::new(),
        };
        w.initialize(xdim,ydim);
        w
    }
    // initializes the map with tiles
    // uses the food scarcity percentage to determine
    // which tiles will randomly get food allocated to them
    // xdim - x dimension of map
    // ydim - y dimension of map
    // food_output - food scarcity percentage
    pub fn initialize(&mut self,xdim: usize,ydim: usize)  
    {
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

        // various cutoff values of the noise to define the levels at which each
        // type of tile is initially generated
        let water_cutoff = 0.008;
        let beach_cutoff = 0.015;
        let grass_cutoff = 0.35;
        let mountain_cutoff = 0.5;

        // Creates tiles by row then appends row to 
        for y in 0..ydim 
        {
            let mut tmp_row = Vec::new();
            for x in 0..xdim 
            {
                let v = nmap.get_value(x,y);

                // Iterates over noise map
                // set water
                if v <= water_cutoff {
                    tmp_row.push(Tile::water());
                    self.water_count+=1;

                // set beach
                } else if v > water_cutoff && v <= beach_cutoff  
                {
                    tmp_row.push(Tile::beach());

                // set grass
                } else if v > beach_cutoff && v <= grass_cutoff 
                {
                    tmp_row.push(Tile::grass());
                    self.grass_count+=1;

                // set mountain
                } else if v > grass_cutoff && v <= mountain_cutoff 
                {
                    tmp_row.push(Tile::mountain());
                    self.mountain_count+=1;

                // set tall mountain
                } else {
                    tmp_row.push(Tile::tall_mountain());
                }

            }
            // Appends row to map vector
            self.map.push(tmp_row);
        }
        self.print_stats();
    }

    // Draws the world to an img buffer then saves to filename
    // Useful for debugging and testing
    #[warn(dead_code)]
    pub fn draw_world(&self,filename: String)
    {
        let mut img = RgbImage::new(self.x_dim as u32, self.y_dim as u32);
        for y in 0..self.y_dim
        {
            for x in 0..self.x_dim
            {
                match self.map[y][x].get_ttype()
                {
                    TileType::Beach => img.put_pixel(x as u32,y as u32,Rgb([252,225,149])),
                    TileType::Grass => img.put_pixel(x as u32,y as u32,Rgb([64, 133, 52])),
                    TileType::Mountain => img.put_pixel(x as u32,y as u32,Rgb([127,141,163])),
                    TileType::TallMountain => img.put_pixel(x as u32,y as u32,Rgb([46,45,44])),
                    TileType::Water => img.put_pixel(x as u32, y as u32, Rgb([0,0,255])),
                }
            }
        }
        match img.save(filename + ".png") 
        {
            Ok(_v) => (),
            Err(e) => println!("{:?}",e),
        };
    }
    // Outputs world tile statistics
    fn print_stats(&self) 
    {
        let readable_tt = self.total_tiles.to_formatted_string(&Locale::en);
        println!("Total tiles: {}", readable_tt);

        let water_per = (self.water_count as f32 / self.total_tiles as f32) * 100.0;
        println!("Water tile count: {} \t Water Percentage: {:.2}",
            self.water_count.to_formatted_string(&Locale::en),
            water_per);
        
        let grass_per = (self.grass_count as f32 / self.total_tiles as f32) * 100.0;
        println!("Grass tile count: {} \t Grass Percentage: {:.2}",
            self.grass_count.to_formatted_string(&Locale::en),
            grass_per );
    }

    // Loads a serialized json world for repeating
    // simulations on the same world 
    // From https://docs.serde.rs/serde_json/de/fn.from_reader.html
    pub fn load_world<P: AsRef<Path>>(path: P) -> Result<World,Box<dyn Error>>
    {
        let file = File::open(path)?;
        let reader = BufReader::new(file);

        let u = serde_json::from_reader(reader)?;

        Ok(u)
    }
    // Debug function to output the world in json format
    #[allow(dead_code)]
    pub fn serialize_world(&self,filename: String) 
    {
        // append json file extension
        let json_fn = filename + ".json";
        // let mut file = File::open(filename);
        let serialized = serde_json::to_string_pretty(&self).unwrap();
        fs::write(json_fn, serialized).expect("Unable to write file");
    }

    pub fn check_ttype(&self, x: usize, y: usize) -> TileType 
    {
        self.map[y][x].get_ttype()
    }

    pub fn get_xdim(&self) -> usize 
    {
        self.x_dim
    }
    pub fn get_ydim(&self) -> usize 
    {
        self.y_dim
    }

}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn has_xdim()
    {
        let w = World::new(10,10);
        assert_eq!(w.get_xdim(),10);
    }
    #[test]
    fn has_ydim()
    {
        let w = World::new(10,10);
        assert_eq!(w.get_ydim(),10);
    }
}