
// internal modules-
pub mod world;
use crate::sim::world::World;

#[derive(Debug)]
struct Sim {
    Years: usize,
    Days: usize,
    Score: i64,
    SimWorld: World,
}

impl Sim {
    pub fn new() -> Sim {
        Sim {
            Years: 0,
            Days: 0,
            Score: 0,
            SimWorld: World::new(10,10,40),
        }
    }
}