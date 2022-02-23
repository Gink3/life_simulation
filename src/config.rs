
// Holds options for simulation details
#[derive(Debug,Copy,Clone)]
pub struct Config {
    xdim: usize,
    ydim: usize,
    starting_people: usize,
}

impl Config {
    pub fn new() -> Config {
        Config {
            xdim: 10000,
            ydim: 10000,
            starting_people: 10,
        }
    }
    pub fn get_xdim(self) -> usize {
        self.xdim
    }
    pub fn get_ydim(self) -> usize {
        self.ydim
    }
    pub fn get_sp(self) -> usize {
        self.starting_people
    }
}