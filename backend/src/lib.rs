mod cache;
mod core;

use std::borrow::BorrowMut;
use std::num::NonZeroU32;
use std::sync::{Arc, Mutex};

use rand::{thread_rng, Rng};

use crate::cache::Cache;
use crate::core::Core;

#[derive(Default)]
struct Stats {}

pub struct Simulation {
    pub cores: Vec<Core>,
    shared_cache: Arc<Mutex<Cache>>,
    cycle: u32,
    stats: Stats,
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

        let cores: Vec<Core> = (0..num_cores.get())
            .map(|_| Core::new(l1_cache_size, l1_cache_associativity, shared_cache.clone()))
            .collect::<Result<_, ()>>()?;

        Ok(Simulation {
            cores,
            shared_cache,
            cycle: 0,
            stats: Stats::default(),
        })
    }

    pub fn step(&mut self) {
        let mut rng = thread_rng();

        for core in &mut self.cores {
            let addr: u32 = rng.random::<u32>() >> 24;
            let val: u32 = rng.random();
            let is_write: bool = rng.random();
            let res = match is_write {
                true => core.write(addr, val),
                false => core.read(addr),
            };
        }
    }
}
