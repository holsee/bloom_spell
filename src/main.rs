extern crate bloom_spell;

use bloom_spell::util::bloom_filter::BloomFilter;
use std::hash::{Hash, SipHasher, Hasher};

fn main() {
    let bf = BloomFilter::new(10, 2);
    println!("{:?}", bf);

    let h = BloomFilter::hash(&"wow");

    println!("{:?}", h);
}
