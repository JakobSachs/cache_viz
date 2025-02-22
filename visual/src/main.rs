use indicatif::ProgressBar;
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

    println!("==============================");
    println!("Running Simulation...");
    println!("==============================");

    let total_iterations = 0x10000;
    let progress_bar = ProgressBar::new(total_iterations);

    for _ in 0..total_iterations {
        progress_bar.inc(1);
        sim.step();
    }

    progress_bar.finish_with_message("Simulation Complete!");

    println!("\n==============================");
    println!("Simulation Complete!");
    println!("==============================");
    println!("Shared Cache Stats:\n{}", sim.shared_stats.lock().unwrap());
    for (i, c) in sim.cores.iter().enumerate() {
        println!("Core {} Stats:\n{}", i + 1, c.stats);
    }
}
