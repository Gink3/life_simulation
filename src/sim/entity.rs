use std::{path::Path, fs::read_to_string};
use serde::{Serialize, Deserialize};
use serde_json::from_str;

#[derive(Serialize, Deserialize, Debug,Clone)]
enum Size 
{
    XL,
    L,
    M,
    S,
    XS,
}
#[derive(Serialize, Deserialize, Debug,Clone)]
enum Gender 
{
    Male,
    Female,
    None,
}

#[derive(Serialize, Deserialize, Debug,Clone)]
pub enum EntityType
{
    Plant,
    Animal,
    Person,
}


type Message = (usize,String);

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Entity
{
    // Entity init shouldn't change
    eid: usize,
    et: EntityType, 
    size: Size,
    generation: usize,
    species: String,
    first_name: String,
    last_name: String,
    birthyear: usize,
    birthday: usize,

    // Updateable Stats
    x: usize,
    y: usize,
    score: i64,

    // Reproduction
    gender: Gender,

    // Checkable flags
    edible: bool,
    can_move_through: bool,
    can_harvest: bool,
}

impl Entity
{
    pub fn new(id: usize, xdim: usize, ydim: usize) -> Entity
    {
        Entity
        {
            eid: 1,
            et: EntityType::Animal,
            size: Size::M,
            generation: 0,
            species: "human".to_string(),
            first_name: "Taylor".to_string(), 
            last_name: "King".to_string(),
            birthyear: 0,
            birthday: 0,
            x: xdim,
            y: ydim,
            score: 0,
            gender: Gender::Male,
            edible: true,
            can_move_through: false,
            can_harvest: true,
        }
    }
    // Loads a entity from json files in ./data/entity
    pub fn load(id: usize, p: &Path) -> Entity
    {
        let json_string = read_to_string(&p).expect("File Not Found");
        let e: Entity = from_str(&json_string).expect("Error while reading");
        e.set_id(id);
        e
    }
    fn set_id(&mut self, id: usize)
    {
        self.eid = id;
    }
    pub fn set_coords(&mut self, xc: usize, yc: usize)
    {
        self.x = xc;
        self.y = yc;
    }

    pub fn get_x(&self) -> usize 
    {
        self.x
    }
    pub fn get_y(&self) -> usize 
    {
        self.y
    }
    pub fn get_entitytype(&self) -> EntityType
    {
        self.et
    }
}


#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn can_load_entity()
    {
        let filepath = Path::new("./data/test/entity_test.json");
        let e = Entity::load(1, filepath);
        assert_eq!("test",e.first_name);
    }

    #[test]
    fn loads_correct_xcoord()
    {
        let filepath = Path::new("./data/test/entity_test.json");
        let e = Entity::load(1, filepath);
        assert_eq!(101,e.get_x());
    }
    #[test]
    fn loads_correct_ycoord()
    {
        let filepath = Path::new("./data/test/entity_test.json");
        let e = Entity::load(1, filepath);
        assert_eq!(102,e.get_y());
    }
}
