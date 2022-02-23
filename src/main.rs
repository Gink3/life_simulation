
use std::env;
use std::time::Instant;

mod sim;
use crate::sim::Sim;

pub mod config;
use config::Config;

fn main() -> std::io::Result<()> {
    // process command line arguments
    let args: Vec<String> = env::args().collect();
    println!("Args -> {:?}",args.len());

    let s: Sim;

    // benchmarking feature check
    if cfg!(feature = "benchmarking") {
        let start = Instant::now();

        // if load-world argument
        if args.iter().any(|i| i=="load-world") {
            let idx = args.iter().position(|r| r=="load-world").unwrap();
            let filename = &args[idx+1];
            
            //Initalizes sim with saved world
            s = Sim::new(Config::load_world(filename.to_string()));

        // Generates new world
        } else {
            s = Sim::new(Config::new_world());
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

            s = Sim::new(Config::load_world(filename.to_string()));
        // Generates new world
        } else {
            s = Sim::new(Config::new_world());
        }
    }

    // TODO
    // Simulation run code here

    //s.sim_debug_json();
    
    Ok(())
}
