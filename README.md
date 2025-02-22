# Cache Simulation Project

This project is a learning exercise to simulate cache behavior in a multi-core processor environment. It models both private and shared caches, allowing for experimentation with different cache configurations and observing their impact on performance metrics such as hit rates.

## Features

- Simulates L1 and L2 cache levels with configurable sizes and associativity.
- Tracks read and write operations, providing statistics on cache hits and misses.
- Uses a simple random access pattern to simulate memory operations across multiple cores.

## Running the Simulation

To run the simulation, use the following command:

```bash
cargo run --release
```

This will execute the simulation and print the cache configuration and statistics to the console.

## Ideas

- Primary next goal is to write a proper graphical visualization for the caches etc
- Implement MOESI cache coherence between local caches
