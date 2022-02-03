
enum Gender {
    Male,
    Female,
}

#[derive(Debug)]
struct Person {
    Pid: usize,
    Generation: usize,
    Score: i64,
    FirstName: String,
    LastName: String,
    Birthyear: usize,
    Birthday: usize,
    X: usize,
    Y: usize,
    Spouse: &Person,
    Gender: Gender,
    Msg
}

impl Person {

}