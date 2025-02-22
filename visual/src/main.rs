use std::num::NonZero;
use std::io::{self, Write};

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
    let bar_width = 50;

    for i in 0..total_iterations {
        if i % (total_iterations / bar_width) == 0 {
            print!("\r[");
            let pos = bar_width * i / total_iterations;
            for _ in 0..pos {
                print!("=");
            }
            for _ in pos..bar_width {
                print!(" ");
            }
            print!("] {}%", (i * 100) / total_iterations);
            io::stdout().flush().unwrap();
        }
        if i % 100 == 0 {
            println!("Iteration: {:04}", i);
        }
        sim.step();
    }

    println!("\n\n==============================");
    println!("Simulation Complete!");
    println!("==============================");
    println!("Shared Cache Stats: {:?}", sim.shared_stats.lock().unwrap());
    for (i, c) in sim.cores.iter().enumerate() {
        println!("Core {} Stats: {:?}", i + 1, c.stats);
    }
}
