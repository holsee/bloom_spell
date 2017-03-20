use super::bitvec_rs::BitVec;
use super::fnv::FnvHasher;

use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

#[derive(Debug)]
pub struct BloomFilter {
    bits: BitVec,
    hash_count: u8
}

impl BloomFilter {

//    pub fn from(expected_inserts: usize, fpr: f64) -> BloomFilter {
//        let m: usize = ceil((-1.0 * (expected_inserts as f64) * ln(fpr))
//                            / powf(2.0.ln(), 2.0)) as usize;
//
//        let k: u8 = ceil(((m as f64) /
//            (expected_inserts as f64)) * ln(2.0)) as u8;
//
//        return BloomFilter::new(m, k)
//    }

    pub fn new(size: usize, hash_count: u8) -> BloomFilter {
        BloomFilter {
            bits: BitVec::from_elem(size, false),
            hash_count: hash_count
        }
    }

    pub fn insert<T: Hash>(&mut self, item: T) {
        let hash_values = self.compute_indices(item);

        for index in hash_values {
            self.bits.set(index as usize, true);
        }
    }

    pub fn maybe_contains<T: Hash>(&self, item: T) -> bool {
        let indices = self.compute_indices(item);
        indices.into_iter().all(|i| self.bits.get(i as usize).unwrap() )
    }

    pub fn compute_indices<T: Hash>(&self, item: T) -> Vec<f64> {
        // Hash value with 2 hash functions
        let mut fnv = FnvHasher::default();
        item.hash(&mut fnv);
        let hash_a: f64 = fnv.finish() as f64;

        let mut sip = DefaultHasher::default();
        item.hash(&mut sip);
        let hash_b: f64 = sip.finish() as f64;

        // Produce multiple hashes and convert to indices
        let size = self.bits.len() as f64;
        let hash_range = 0..self.hash_count;

        hash_range.into_iter()
            .map(|i| (hash_a + (i as f64) * hash_b) % size )
            .collect()
    }
}

