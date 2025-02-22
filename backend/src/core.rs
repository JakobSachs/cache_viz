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
    shared_stats: Arc<Mutex<CoreStats>>,
    pub stats: CoreStats,
}

impl Core {
    pub fn new(
        cache_size: NonZeroU32,
        cache_associativity: NonZeroU32,
        shared_cache: Arc<Mutex<Cache>>,
        shared_stats: Arc<Mutex<CoreStats>>,
    ) -> Result<Self, ()> {
        let c = Cache::new(cache_size, cache_associativity)?;
        Ok(Core {
            stats: CoreStats::default(),
            private_cache: c,
            shared_cache,
            shared_stats,
        })
    }

    pub fn read(&mut self, addr: u32) {
        let local_res = self.private_cache.read(addr);
        match local_res {
            true => self.stats.read_hit += 1,
            false => self.stats.read_miss += 1,
        }
        if !local_res {
            let mut s_c = self.shared_cache.lock().unwrap();
            let mut s_t = self.shared_stats.lock().unwrap();
            match s_c.read(addr) {
                true => s_t.read_hit += 1,
                false => {
                    s_t.read_miss += 1;
                }
            }
        }
    }

    pub fn write(&mut self, addr: u32, val: u32) {
        let local_res = self.private_cache.write(addr, val);
        match local_res {
            true => self.stats.write_hit += 1,
            false => self.stats.write_miss += 1,
        }
        // if we have a local miss, we go to shared
        if !local_res {
            let mut s_c = self.shared_cache.lock().unwrap();
            let mut s_t = self.shared_stats.lock().unwrap();
            match s_c.write(addr, val) {
                true => s_t.write_hit += 1,
                false => {
                    s_t.write_miss += 1;
                }
            }
        }
    }
}
