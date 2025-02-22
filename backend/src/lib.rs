mod cache;
mod core;

use core::CoreStats;
use std::num::NonZeroU32;
use std::sync::{Arc, Mutex};

use rand::{rng, Rng};

use crate::cache::Cache;
use crate::core::Core;

pub struct Simulation {
    pub cores: Vec<Core>,
    shared_cache: Arc<Mutex<Cache>>,
    pub shared_stats: Arc<Mutex<CoreStats>>,
    cycle: u32,
}

impl Simulation {
    pub fn new(
        num_cores: NonZeroU32,
        l1_cache_size: NonZeroU32,
        l1_cache_associativity: NonZeroU32,
        l2_cache_size: NonZeroU32,
        l2_cache_associativity: NonZeroU32,
    ) -> Result<Self, ()> {
        let shared_cache = Arc::new(Mutex::new(Cache::new(
            l2_cache_size,
            l2_cache_associativity,
        )?));

        let shared_stats = Arc::new(Mutex::new(CoreStats::default()));

        let cores: Vec<Core> = (0..num_cores.get())
            .map(|_| {
                Core::new(
                    l1_cache_size,
                    l1_cache_associativity,
                    shared_cache.clone(),
                    shared_stats.clone(),
                )
            })
            .collect::<Result<_, ()>>()?;

        Ok(Simulation {
            cores,
            shared_cache,
            cycle: 0,
            shared_stats,
        })
    }

    pub fn step(&mut self) {
        self.cycle += 1;
        let mut rng = rng();

        for core in &mut self.cores {
            let addr: u32 = rng.random::<u32>() >> 20;
            let val: u32 = rng.random();
            let is_write: bool = rng.random();
            let _ = match is_write {
                true => core.write(addr, val),
                false => core.read(addr),
            };
        }
    }
}
