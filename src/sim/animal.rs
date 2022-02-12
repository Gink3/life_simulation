
#[derive(Debug,Clone)]
enum Size {
    XL,
    L,
    M,
    S,
    XS,
}

#[derive(Debug,Clone)]
pub struct Animal {
    species: String,
    health: i32,
    hunger: i32,
    size: Size
    
}

impl Animal {

}