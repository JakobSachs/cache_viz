# Cache Simulation Project

This project is a learning exercise to simulate cache behavior in a multi-core processor environment. It models both private and shared caches, allowing for experimentation with different cache configurations and observing their impact on performance metrics such as hit rates.

## Features

- Simulates L1 and L2 cache levels with configurable sizes and associativity.
- Tracks read and write operations, providing statistics on cache hits and misses.
- Uses a simple random access pattern to simulate memory operations across multiple cores.

## Setup

To set up the project, ensure you have Rust installed on your system. You can install Rust using [rustup](https://rustup.rs/).

Clone the repository and navigate to the project directory:

```bash
git clone <repository-url>
cd <project-directory>
```

## Running the Simulation

To run the simulation, use the following command:

```bash
cargo run --release
```

This will execute the simulation and print the cache configuration and statistics to the console.

## Configuration

Cache sizes and associativity can be adjusted in the `visual/src/main.rs` file by modifying the `l1_cache_size`, `l1_cache_associativity`, `l2_cache_size`, and `l2_cache_associativity` variables.

## License

This project is for educational purposes and does not have a specific license.
