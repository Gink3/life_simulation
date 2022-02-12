
use std::env;

mod sim;
use sim::world::World;


use std::fmt;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    println!("{:?}",args);

    if args.len() != 3 {
        panic!("life_simulation requires 3 arguments")
    }

    let xdim = args[1].parse().unwrap();
    let ydim = args[2].parse().unwrap();

    let w = World::new(xdim,ydim,60);
    w.serialize_world("world_test.json".to_string());
    Ok(())
}
