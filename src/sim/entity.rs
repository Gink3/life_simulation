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

#[allow(dead_code)]
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
    tags: Vec<String>,
}

impl Entity
{
    #[allow(dead_code)]
    pub fn new(id: usize, xdim: usize, ydim: usize) -> Entity
    {
        Entity
        {
            eid: id,
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
            tags: Vec::<String>::new(),
        }
    }
    // Loads a entity from json files in ./data/entity
    pub fn load(id: usize, p: &Path) -> Entity
    {
        let json_string = read_to_string(&p).expect(&("File Not Found ".to_owned() + p.to_str().unwrap()));
        let mut e: Entity = from_str(&json_string).expect("Error while reading");
        e.set_id(id);
        e
    }
    #[allow(dead_code)]
    pub fn load_plant(id: usize, pn: String) -> Entity
    {
        let path_str = "./data/entity/plant/".to_owned() + &pn + &".json".to_owned();
        let path = Path::new(&path_str);
        let json_string = read_to_string(path).expect(&("File not found".to_owned() + path.to_str().unwrap()));
        let mut e: Entity = from_str(&json_string).expect("Error while reading");
        e.set_id(id);
        e
    }
    // Loads animal entities from templates found in
    // ./data/entity/
    // id - Entity Id
    // an - animal name for file
    // i.e. an = rabbit -> ./data/entity/animal/rabbit.json
    #[allow(dead_code)]
    pub fn load_animal(id: usize, an: String) -> Entity
    {
        let path_str = "./data/entity/animal/".to_owned() + &an + &".json".to_owned();
        let path = Path::new(&path_str);
        let json_string = read_to_string(path).expect(&("File not found".to_owned() + path.to_str().unwrap()));
        let mut e: Entity = from_str(&json_string).expect("Error while reading");
        e.set_id(id);
        e
    }
    #[allow(dead_code)]
    pub fn load_adult(id: usize) -> Entity
    {
        let path = Path::new("./data/entity/person/adult.json");
        let json_string = read_to_string(path).expect(&("File not found".to_owned() + path.to_str().unwrap()));
        let mut e: Entity = from_str(&json_string).expect("Error while reading");
        e.set_id(id);
        e
    }
    // Setter functions
    fn set_id(&mut self, id: usize)
    {
        self.eid = id;
    }
    pub fn set_coords(&mut self, xc: usize, yc: usize)
    {
        self.x = xc;
        self.y = yc;
    }

    // Getter functions
    #[allow(dead_code)]
    pub fn get_species(&self) -> &String
    {
        &self.species
    }
    pub fn get_x(&self) -> usize 
    {
        self.x
    }
    pub fn get_y(&self) -> usize 
    {
        self.y
    }
    pub fn get_entitytype(&self) -> &EntityType
    {
        &self.et
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
    #[test]
    fn can_load_berry_bush()
    {
        let e = Entity::load_plant(1, "berry_bush".to_string());
        assert_eq!(e.get_species(), "bush")
    }
    #[test]
    fn can_load_rabbit()
    {
        let e = Entity::load_animal(1, "rabbit".to_string());
        assert_eq!(e.get_species(), "rabbit")
    }
    #[test]
    fn can_load_wolf()
    {
        let e = Entity::load_animal(1, "wolf".to_string());
        assert_eq!(e.get_species(), "wolf")
    }
    #[test]
    fn can_load_adult()
    {
        let e = Entity::load_adult(1);
        assert_eq!(e.get_species(), "human")
    }
}
