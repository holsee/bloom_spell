extern crate bloom_spell;
use bloom_spell::util::bloom_filter::BloomFilter;

fn main() {
    let bf = {
        let mut bf = BloomFilter::new(1000, 20);
        bf.insert("wow");
        bf.insert("wee");
        bf
    };

    let wow = &bf.maybe_contains("wow");
    println!("wow = {:#?}", wow);

    let foo = &bf.maybe_contains("foo");
    println!("foo = {:#?}", foo);
}
