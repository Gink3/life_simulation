
mod sim;
use sim::world::World;

fn main() -> std::io::Result<()> {
    let w = World::new(10,10,60);
    w.serialize_world("world_test.json".to_string());
    Ok(())
}
