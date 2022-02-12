
// internal modules-
pub mod world;
use crate::sim::world::World;

mod person;
use person::Person;

mod plant;
use plant::Plant;

mod animal;
use animal::Animal;

#[derive(Debug)]
struct Sim<'a> {
    years: usize,
    days: usize,
    score: i64,
    sim_world: World,
    people: Vec<Person<'a>>,
    plants: Vec<Plant>,
    animals: Vec<Animal>,
}

impl Sim<'_> {
    pub fn new() -> Sim<'static> {
        Sim {
            years: 0,
            days: 0,
            score: 0,
            sim_world: World::new(10,10,40),
            people: Vec::<Person>::new(),
            plants: Vec::<Plant>::new(),
            animals: Vec::<Animal>::new(),
        }
    }
}