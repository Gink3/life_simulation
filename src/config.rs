
// Holds options for simulation details
#[derive(Debug,Copy,Clone)]
pub struct Config {
    xdim: usize,
    ydim: usize,
    food_scarcity: usize,
}

impl Config {
    pub fn new() -> Config {
        Config {
            xdim: 10,
            ydim: 10,
            food_scarcity: 40,
        }
    }
    pub fn get_xdim(self) -> usize {
        self.xdim
    }
    pub fn get_ydim(self) -> usize {
        self.ydim
    }
    pub fn get_fs(self) -> usize {
        self.food_scarcity
    }
}