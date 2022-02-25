use serde::{Serialize, Deserialize};

// Holds options for simulation details
#[derive(Serialize, Deserialize, Debug,Clone)]
pub struct Config {
    xdim: usize,
    ydim: usize,
    load_world: bool,
    world_file: String,
    init_pl: usize,         // initial plant count
    init_an: usize,         // initial animal count
    init_pe: usize,         // initial person count
}

impl Config {
    // Config for creating a new world
    // Change settings here
    pub fn new_world(filename: String) -> Config {
        Config {
            xdim: 100,
            ydim: 100,
            load_world: false,
            world_file: filename,
            init_pl: 10,
            init_an: 10,
            init_pe: 10,
        }
    }
    // Config for loading a previous world
    pub fn load_world(filename: String) -> Config {
        Config {
            xdim: 0,
            ydim: 0,
            load_world: true,
            world_file: filename,
            init_pl: 10,
            init_an: 10,
            init_pe: 10,
        }
    }
    pub fn get_xdim(&self) -> usize {
        self.xdim
    }
    pub fn get_ydim(&self) -> usize {
        self.ydim
    }
    // Get initial plants
    pub fn get_init_pl(&self) -> usize {
        self.init_pl
    }
    // Get initial animals
    pub fn get_init_an(&self) -> usize {
        self.init_an
    }
    // Get initial people
    pub fn get_init_pe(&self) -> usize {
        self.init_pe
    }
    pub fn will_load_world(&self) -> bool {
        self.load_world
    }
    // Ran into a move issue, used this stack overflow issue to change to a borrow
    // https://stackoverflow.com/questions/28158738/cannot-move-out-of-borrowed-content-cannot-move-out-of-behind-a-shared-referen
    pub fn get_world_filename(&self) -> &String {
        &self.world_file
    }
}