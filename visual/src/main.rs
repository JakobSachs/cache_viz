use std::num::NonZero;

fn main() {
    println!("Hello, world!");

    let mut sim = backend::Simulation::new(
        NonZero::new(4).unwrap(),
        NonZero::new(0x400).unwrap(),
        NonZero::new(8).unwrap(),
        NonZero::new(0x10000).unwrap(),
        NonZero::new(8).unwrap(),
    )
    .unwrap();

    for i in 0..0x100 {
        println!("iter: {}", i);
        sim.step();
    }

    for c in &sim.cores {
        println!("{:?}", c.stats);
    }
}
