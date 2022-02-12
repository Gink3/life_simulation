
#[derive(Debug)]
enum Gender {
    Male,
    Female,
}

type Message = (usize,String);

#[derive(Debug)]
pub struct Person<'a> {
    pid: usize,
    generation: usize,
    score: i64,
    first_name: String,
    last_name: String,
    birthyear: usize,
    birthday: usize,
    x: usize,
    y: usize,
    spouse: &'a Person<'a>,
    gender: Gender,
}

impl Person<'static> {

}