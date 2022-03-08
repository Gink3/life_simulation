
use std::env;
use std::time::Instant;
use std::path::PathBuf;
mod sim;
use crate::sim::Sim;

pub mod config;
use config::Config;

use sim::world::World;

fn main() -> std::io::Result<()> {
    // process command line arguments
    let args: Vec<String> = env::args().collect();
    println!("Args -> {:?}",args.len());

    let _s: Sim;

    // Generates worlds to look at and store without running a simulation
    // Useful for testing world generation
    if args.len() >= 3 && args[1] == "--gen-world" {
        let filename = &args[2];
        let c = Config::new_world(PathBuf::from(filename));
        let w = World::new(c.get_xdim(),c.get_ydim());
        w.draw_world(c.get_world_filename().to_path_buf());
        w.serialize_world(filename.to_string());

        return Ok(());
    }
    // benchmarking feature check
    if cfg!(feature = "benchmarking") {
        let start = Instant::now();

        // if load-world argument
        if args.iter().any(|i| i=="load-world") {
            let idx = args.iter().position(|r| r=="load-world").unwrap();
            let filename = &args[idx+1];
            
            let c = Config::load_config(PathBuf::from(filename));
            //Initalizes sim with saved world
            _s = Sim::new(c);

        // Generates new world
        } else {
            let c = Config::new_world(PathBuf::from("./sim_out/world"));
            _s = Sim::new(c);
        }
        
        // Calculate and print time elapsed
        let duration = start.elapsed();
        println!("{:?}",duration);
    // if benchmarking not enabled
    } else {

        // loads world
        if args.iter().any(|i| i=="load-world") {
            let idx = args.iter().position(|r| r=="load-world").unwrap();
            let filename = &args[idx+1];

            _s = Sim::new(Config::load_config(PathBuf::from(filename)));
        // Generates new world
        } else {
            _s = Sim::new(Config::new_world(PathBuf::from("./sim_out/world")));
        }
    }

    _s.print_entity_stats();
    _s.run(1000);
    _s.sim_debug_json();
    
    Ok(())
}
