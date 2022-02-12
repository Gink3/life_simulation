
#[derive(Debug)]
enum Gender {
    Male,
    Female,
}

type Message = (usize,String);

#[derive(Debug)]
pub struct Person<'a> {
    Pid: usize,
    Generation: usize,
    Score: i64,
    FirstName: String,
    LastName: String,
    Birthyear: usize,
    Birthday: usize,
    X: usize,
    Y: usize,
    Spouse: &'a Person<'a>,
    Gender: Gender,
}

impl Person<'static> {

}