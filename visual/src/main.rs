use indicatif::ProgressBar;

const L1_CACHE_SIZE: u32 = 0x400;
const L1_CACHE_ASSOCIATIVITY: u32 = 8;
const L2_CACHE_SIZE: u32 = 0x1000;
const L2_CACHE_ASSOCIATIVITY: u32 = 8;
const NUM_CORES: u32 = 4;
const TOTAL_ITERATIONS: u64 = 0x1000;
use std::num::NonZero;

fn main() {
    println!("Starting Cache Simulation...");

    let l1_cache_size = NonZero::new(L1_CACHE_SIZE).unwrap();
    let l1_cache_associativity = NonZero::new(L1_CACHE_ASSOCIATIVITY).unwrap();
    let l2_cache_size = NonZero::new(L2_CACHE_SIZE).unwrap();
    let l2_cache_associativity = NonZero::new(L2_CACHE_ASSOCIATIVITY).unwrap();

    let mut sim = backend::Simulation::new(
        NonZero::new(NUM_CORES).unwrap(),
        l1_cache_size,
        l1_cache_associativity,
        l2_cache_size,
        l2_cache_associativity,
    )
    .unwrap();

    println!("==============================");
    println!("Cache Configuration:");
    println!("L1 Cache Size: {} bytes", l1_cache_size);
    println!("L1 Cache Associativity: {}-ways", l1_cache_associativity);
    println!("L2 Cache Size: {} bytes", l2_cache_size);
    println!("L2 Cache Associativity: {}-ways", l2_cache_associativity);
    println!("Running Simulation...");
    println!("==============================");

    let total_iterations = TOTAL_ITERATIONS;
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
