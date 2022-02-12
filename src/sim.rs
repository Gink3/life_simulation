
// internal modules-
pub mod world;
use crate::sim::world::World;

mod person;
use person::Person;

#[derive(Debug)]
struct Sim<'a> {
    Years: usize,
    Days: usize,
    Score: i64,
    SimWorld: World,
    People: Vec<Person<'a>>
}

impl Sim<'_> {
    pub fn new() -> Sim<'static> {
        Sim {
            Years: 0,
            Days: 0,
            Score: 0,
            SimWorld: World::new(10,10,40),
            People: Vec::<Person>::new()
        }
    }
}