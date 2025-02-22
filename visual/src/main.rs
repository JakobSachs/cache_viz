use indicatif::ProgressBar;
use std::num::NonZero;

fn main() {
    println!("Starting Cache Simulation...");

    let l1_cache_size = NonZero::new(0x400).unwrap();
    let l1_cache_associativity = NonZero::new(8).unwrap();
    let l2_cache_size = NonZero::new(0x1000).unwrap();
    let l2_cache_associativity = NonZero::new(8).unwrap();

    let mut sim = backend::Simulation::new(
        NonZero::new(4).unwrap(),
        l1_cache_size,
        l1_cache_associativity,
        l2_cache_size,
        l2_cache_associativity,
        NonZero::new(4).unwrap(),
        NonZero::new(0x400).unwrap(),
        NonZero::new(8).unwrap(),
        NonZero::new(0x1000).unwrap(),
        NonZero::new(8).unwrap(),
    )
    .unwrap();

    println!("==============================");
    println!("Cache Configuration:");
    println!("L1 Cache Size: {} bytes", l1_cache_size);
    println!("L1 Cache Associativity: {}", l1_cache_associativity);
    println!("L2 Cache Size: {} bytes", l2_cache_size);
    println!("L2 Cache Associativity: {}", l2_cache_associativity);
    println!("Running Simulation...");
    println!("==============================");

    let total_iterations = 0x1000;
    let progress_bar = ProgressBar::new(total_iterations);

    for _ in 0..total_iterations {
        progress_bar.inc(1);
        sim.step();
    }

    progress_bar.finish_with_message("Simulation Complete!");

    println!("\n==============================");
    println!("Simulation Complete!");
    println!("==============================\n");
    println!("Stats: \n");
    println!(
        "Shared Cache:\n==============================\n{}",
        sim.shared_stats.lock().unwrap()
    );
    println!("==============================");
    for (i, c) in sim.cores.iter().enumerate() {
        println!("Core {} Stats:\n{}", i + 1, c.stats);
    }
    println!("==============================");
}
