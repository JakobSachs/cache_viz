use std::{
    num::NonZeroU32,
    sync::{Arc, Mutex},
};

use crate::cache::Cache;

#[derive(Default, Debug)]
pub struct CoreStats {
    read_hit: u32,
    read_miss: u32,
    write_hit: u32,
    write_miss: u32,
}

pub struct Core {
    private_cache: Cache,
    shared_cache: Arc<Mutex<Cache>>,
    pub stats: CoreStats,
}

impl Core {
    pub fn new(
        cache_size: NonZeroU32,
        cache_associativity: NonZeroU32,
        shared_cache: Arc<Mutex<Cache>>,
    ) -> Result<Self, ()> {
        let c = Cache::new(cache_size, cache_associativity)?;
        Ok(Core {
            stats: CoreStats::default(),
            private_cache: c,
            shared_cache,
        })
    }

    pub fn read(&mut self, addr: u32) -> bool {
        match self.private_cache.read(addr) {
            true => self.stats.read_hit += 1,
            false => self.stats.read_miss += 1,
        }
        true
    }

    pub fn write(&mut self, addr: u32, val: u32) -> bool {
        match self.private_cache.write(addr, val) {
            true => self.stats.write_hit += 1,
            false => self.stats.write_miss += 1,
        }
        true
    }
}
