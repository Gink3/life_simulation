
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
    spouse_pid: usize,
    gender: Gender,
}

impl Person {

}