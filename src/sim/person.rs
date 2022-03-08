
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
    message_log: Vec<Message>,
}

impl Person {
    pub fn new(l: usize, xc: usize, yc: usize) -> Person 
    {
        Person {
            pid: l,
            generation: 0,
            score: 0,
            first_name: "Taylor".to_string(), 
            last_name: "King".to_string(),
            birthyear: 0,
            birthday: 0,
            x: xc,
            y: yc,
            spouse_pid: -1,
            gender: Gender::Male,
            message_log: Vec::<Message>::new(),
        }
    }
    #[warn(dead_code)]
    pub fn is_at(&self, xc: usize, yc: usize) -> bool
    {
        if self.x == xc && self.y == yc 
        {
            return true
        }
        false
    }
    pub fn get_x(&self) -> usize
    {
        self.x
    }
    pub fn get_y(&self) -> usize
    {
        self.y
    }
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn has_correct_xcoord()
    {
        let p = Person::new(0,1,1);
        assert_eq!(p.get_x(),1);
    }
    #[test]
    fn has_correct_ycoord()
    {
        let p = Person::new(0,1,1);
        assert_eq!(p.get_y(),1);
    }

    #[test]
    fn has_position()
    {
        let p = Person::new(0,1,1);
        assert!(p.is_at(1, 1));
    }

}