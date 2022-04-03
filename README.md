# life_simulation
These readme instructions are made for someone completely new to understand. I am using this
project to learn rust from the ground up and explain what I am doing along the way.

This simulation looks to simulate human and animal evolution in different enviroments.

## Compile commands
To compile for release
`cargo build --release`

### Compile Options
For runtime statistics
`--features "benchmarking`

## Run commands
run debug
`cargo run`
run release
`cargo run --release`

## File descriptions
sim.rs -> Simulation structs and controller code
config.rs -> simulation config for managing simulation options

## Playing around with the simulation
To play with the simulation options change attributes in the config.rs
Then recompile

### Generating a world
To generate a world without running a simulation use
`--gen-world <file name>`