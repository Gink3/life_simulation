
use std::env;

mod sim;
use sim::world::World;
use crate::sim::Sim;

pub mod config;
use config::Config;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    println!("{:?}",args.len());


    let s = Sim::new(Config::new());

    s.sim_debug_ron("sim_test.ron".to_string());
    
    Ok(())
}
