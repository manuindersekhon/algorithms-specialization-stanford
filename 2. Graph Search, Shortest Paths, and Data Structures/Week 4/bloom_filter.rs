/**
 * Bloom filter implementation.
 * Value is hashed 3 times using Rust's default hasher.
 */
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

const BLOOM_FILTER_SIZE: usize = 65535;

/// Represents a bloom filter.
struct BloomFilter {
    // Here. bool is used as the smallest
    table: Vec<bool>,
}

impl BloomFilter {
    // Returns a new bloom filter with all bits unset.
    fn new() -> BloomFilter {
        BloomFilter {
            table: vec![false; BLOOM_FILTER_SIZE],
        }
    }

    // Inserts data into bloom filter.
    fn insert(&mut self, data: i32) {
        let hash = self.hash_function(data);

        // Set these bits to 1.
        self.table[hash.0] = true;
        self.table[hash.1] = true;
        self.table[hash.2] = true;
    }

    // Checks if value is present in bloom filter.
    fn check(&self, data: i32) -> bool {
        let hash = self.hash_function(data);

        return self.table[hash.0] && self.table[hash.1] && self.table[hash.2];
    }

    /// Hash with 3 independent hash functions.
    fn hash_function(&self, data: i32) -> (usize, usize, usize) {
        // Hash with Rust default hasher.
        let mut rust_hasher = DefaultHasher::new();
        data.hash(&mut rust_hasher);

        let hash1 = (rust_hasher.finish() % BLOOM_FILTER_SIZE as u64) as usize;

        hash1.hash(&mut rust_hasher);
        let hash2 = (rust_hasher.finish() % BLOOM_FILTER_SIZE as u64) as usize;

        hash2.hash(&mut rust_hasher);
        let hash3 = (rust_hasher.finish() % BLOOM_FILTER_SIZE as u64) as usize;

        (hash1, hash2, hash3)
    }
}

fn main() {
    let mut bloom_filter = BloomFilter::new();

    bloom_filter.insert(3);
    bloom_filter.insert(54);
    bloom_filter.insert(87);
    bloom_filter.insert(32453);
    bloom_filter.insert(23434);
    bloom_filter.insert(23);

    println!("{}", bloom_filter.check(1));
    println!("{}", bloom_filter.check(5));
    println!("{}", bloom_filter.check(3));
    println!("{}", bloom_filter.check(23));
    println!("{}", bloom_filter.check(32));
    println!("{}", bloom_filter.check(544));
}
