
// Holds options for simulation details
#[derive(Debug,Clone)]
pub struct Config {
    xdim: usize,
    ydim: usize,
    starting_people: usize,
    load_world: bool,
    world_file: String,
}

impl Config {
    // Config for creating a new world
    pub fn new_world() -> Config {
        Config {
            xdim: 100,
            ydim: 100,
            starting_people: 10,
            load_world: false,
            world_file: String::new(),
        }
    }
    // Config for loading a previous world
    pub fn load_world(filename: String) -> Config {
        Config {
            xdim: 0,
            ydim: 0,
            starting_people: 10,
            load_world: true,
            world_file: filename,
        }
    }
    pub fn get_xdim(&self) -> usize {
        self.xdim
    }
    pub fn get_ydim(&self) -> usize {
        self.ydim
    }
    pub fn get_sp(&self) -> usize {
        self.starting_people
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