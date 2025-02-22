use std::num::NonZero;

fn main() {
    println!("Starting Cache Simulation...");

    let mut sim = backend::Simulation::new(
        NonZero::new(4).unwrap(),
        NonZero::new(0x400).unwrap(),
        NonZero::new(8).unwrap(),
        NonZero::new(0x1000).unwrap(),
        NonZero::new(8).unwrap(),
    )
    .unwrap();

    for i in 0..0x1000 {
        if i % 100 == 0 {
            println!("Iteration: {:04}", i);
        }
        sim.step();
    }

    println!("\nSimulation Complete!");
    println!("Shared Cache Stats: {:?}", sim.shared_stats.lock().unwrap());
    for (i, c) in sim.cores.iter().enumerate() {
        println!("Core {} Stats: {:?}", i + 1, c.stats);
    }
}
