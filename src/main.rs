
use std::env;
use std::time::{Duration, Instant};

mod sim;
use crate::sim::Sim;

pub mod config;
use config::Config;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    //println!("{:?}",args.len());

    if cfg!(feature = "benchmarking") {
        let start = Instant::now();
        let s = Sim::new(Config::new());
        let duration = start.elapsed();
        println!("{:?}",duration);
    } else {
        let s = Sim::new(Config::new());
    }


    //s.sim_debug_json();
    
    Ok(())
}
