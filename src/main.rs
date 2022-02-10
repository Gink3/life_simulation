
use std::env;

mod sim;
use sim::world::World;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    println!("{:?}",args);

    let xdim = args[1].parse().unwrap();
    let ydim = args[2].parse().unwrap();

    let w = World::new(xdim,ydim,60);
    w.serialize_world("world_test.json".to_string());
    Ok(())
}
