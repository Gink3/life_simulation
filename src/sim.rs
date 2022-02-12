
// internal modules-
pub mod world;
use crate::sim::world::World;

mod person;
use person::Person;

#[derive(Debug)]
struct Sim<'a> {
    years: usize,
    days: usize,
    score: i64,
    sim_world: World,
    people: Vec<Person<'a>>
}

impl Sim<'_> {
    pub fn new() -> Sim<'static> {
        Sim {
            years: 0,
            days: 0,
            score: 0,
            sim_world: World::new(10,10,40),
            people: Vec::<Person>::new()
        }
    }
}