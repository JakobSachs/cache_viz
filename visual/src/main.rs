use std::num::NonZero;
use indicatif::ProgressBar;

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

    println!("==============================");
    println!("Running Simulation...");
    println!("==============================");

    let total_iterations = 0x1000;
    let progress_bar = ProgressBar::new(total_iterations);

    for i in 0..total_iterations {
        progress_bar.inc(1);
        sim.step();
    }

    progress_bar.finish_with_message("Simulation Complete!");

    println!("\n==============================");
    println!("Simulation Complete!");
    println!("==============================");
    println!("Shared Cache Stats: {:?}", sim.shared_stats.lock().unwrap());
    for (i, c) in sim.cores.iter().enumerate() {
        println!("Core {} Stats: {:?}", i + 1, c.stats);
    }
}
