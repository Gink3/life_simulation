
use std::env;

mod sim;
use sim::world::World;




fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    println!("{:?}",args.len());

    //check argument length
    if args.len() != 3 {
        panic!("life_simulation requires 3 arguments")
    }

    //set x and y dimensions for world
    let xdim = args[args.len()-2].parse().unwrap();
    let ydim = args[args.len()-1].parse().unwrap();

    let w = World::new(xdim,ydim,60);

    w.serialize_world("world_test.json".to_string());
    
    Ok(())
}
