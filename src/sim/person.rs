
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
enum Gender {
    Male,
    Female,
}

type Message = (usize,String);

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Person {
    pid: usize,
    generation: usize,
    score: i64,
    first_name: String,
    last_name: String,
    birthyear: usize,
    birthday: usize,
    x: usize,
    y: usize,
    spouse_pid: isize,
    gender: Gender,
}

impl Person {
    pub fn new(l: usize) -> Person {
        Person {
            pid: l,
            generation: 0,
            score: 0,
            first_name: "Taylor".to_string(), 
            last_name: "King".to_string(),
            birthyear: 0,
            birthday: 0,
            x: 0,
            y: 0,
            spouse_pid: -1,
            gender: Gender::Male,
        }
    }
}