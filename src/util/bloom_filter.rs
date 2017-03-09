use super::bitvec_rs::BitVec;
use std::hash::{Hash, SipHasher, Hasher};

#[derive(Debug)]
pub struct BloomFilter {
    bits: BitVec,
    hash_count: u8
}

impl BloomFilter {
    pub fn new(size: usize, hash_count: u8) -> BloomFilter {
        BloomFilter {
            bits: BitVec::from_elem(16, false),
            hash_count: hash_count
        }
    }

    pub fn hash<T: Hash>(t: &T) -> u64 {
        let mut s = SipHasher::new();
        t.hash(&mut s);
        s.finish()
    }

    // TODO: implement Insert & Insert Many

    // TODO: implement Contains

    // TODO: read from file

    // TODO: dump filter to file
}

