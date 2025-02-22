use std::{
    hint::assert_unchecked,
    num::{NonZero, NonZeroU32, NonZeroUsize},
};

use rand::Rng;

// #[derive(Clone, Default)]
// pub enum LineState {
//     MODIFIED,
//     OWNED,
//     EXCLUSIVE,
//     SHARED,
//     #[default]
//     INVALID,
// }

#[derive(Clone, Default)]
pub struct CacheLine {
    // state: LineState,
    data: u32,
    tag: u32,
    lru: u32,
}

pub struct Cache {
    size: NonZeroU32,
    associativity: NonZeroU32,
    set_count: u32,
    sets: Vec<Vec<CacheLine>>,
}

impl Cache {
    pub fn new(size: NonZeroU32, associativity: NonZeroU32) -> Result<Self, ()> {
        if associativity >= size {
            return Err(());
        }
        let sets = size.get() / (associativity.get() * 4); // recall we have 4 bytes/line

        return Ok(Cache {
            size,
            associativity,
            set_count: sets,
            sets: vec![vec![CacheLine::default(); associativity.get() as usize]; sets as usize],
        });
    }

    pub fn update_lru(&mut self) {}

    pub fn read(&mut self, addr: u32) -> bool {
        let (_, idx, tag) = self.destruct_addr(addr);
        assert!(idx < self.set_count);

        let s = self.sets.get_mut(idx as usize).unwrap();
        let tar = s.iter_mut().find(|l| l.tag == tag);
        if tar.is_some() {
            let target = tar.unwrap();
            target.lru = 0;
            // tag is already set

            self.update_lru();
            return true;
        }

        // find line with hightest lru counter to evic
        let max_lru = s
            .iter()
            .fold(0u32, |max, l| if l.lru > max { l.lru } else { max });
        let target_line = s.iter_mut().find(|l| l.lru == max_lru).unwrap();

        target_line.data = 0xFFFFFFFF;
        target_line.lru = 0;
        target_line.tag = tag;

        self.update_lru();
        return false;
    }

    pub fn write(&mut self, addr: u32, val: u32) -> bool {
        let (_, idx, tag) = self.destruct_addr(addr);
        assert!(idx < self.set_count);

        let s = self.sets.get_mut(idx as usize).unwrap();
        let tar = s.iter_mut().find(|l| l.tag == tag);
        if let Some(target) = tar {
            target.data = val;
            target.lru = 0;
            self.update_lru();
            return true;
        }

        // Cache miss: find line with highest LRU counter to evict
        let max_lru = s.iter().fold(0u32, |max, l| if l.lru > max { l.lru } else { max });
        let target_line = s.iter_mut().find(|l| l.lru == max_lru).unwrap();

        target_line.data = val;
        target_line.lru = 0;
        target_line.tag = tag;

        self.update_lru();
        return false;
    }

    /// Destructures a given address into its constituent parts: Offset, Index, Tag
    /// based on the Caches Configuration
    ///
    /// The offset is always a 2 bits, since we settled on u32 for our cacheline-datatype
    /// Index is log2(num_sets) bits wide, to address the set the line belongs to
    /// Tag is 32 - 2 - log2(num_sets) bits wide (duh)
    pub fn destruct_addr(&self, addr: u32) -> (u32, u32, u32) {
        let offset = addr & 0b11;
        let index_width = if self.set_count > 1 {
            31 - self.set_count.leading_zeros() // log2(num_sets)
        } else {
            0 // Fully associative cache (1 set)
        };
        let index = (addr >> 2) & ((1 << (index_width)) - 1);
        let tag = addr >> (2 + index_width);
        (offset, index, tag)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn cache_destruct_addr() {
        let c = Cache::new(32.try_into().unwrap(), 2.try_into().unwrap()).unwrap();

        let addr: u32 = 0x0000_abcf;
        let (offset, index, tag) = c.destruct_addr(addr);

        assert_eq!(offset, 0b11);
        assert_eq!(index, 0b11);
        assert_eq!(tag, 0xabc);
    }
}
