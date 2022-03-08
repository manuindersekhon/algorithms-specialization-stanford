/**
 * Hash Table for integers with separate chaining.
 */
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

/// Size of hash table. Ideally should be a prime number 1.3 times the size of input we are expecting.
const HASH_TABLE_SIZE: usize = 65535;

/// Represent hash table with separate chaining.
#[derive(Debug)]
struct HashTable {
    table: Vec<Vec<i32>>,
}

impl HashTable {
    /// Returns a new hash table.
    fn new() -> HashTable {
        HashTable {
            table: vec![Vec::<i32>::new(); HASH_TABLE_SIZE],
        }
    }

    /// Inserts the value into hash table.
    fn insert(&mut self, data: i32) {
        // Calculate index to place this value.
        let index = self.hash_function(data);

        // Insert into table, iff not already present.
        if !self.table[index].contains(&data) {
            self.table[index].push(data);
        }
    }

    /// Returns true if value is present in hash table, false otherwise.
    fn contains(&self, data: i32) -> bool {
        let index = self.hash_function(data);
        return self.table[index].contains(&data);
    }

    /// Delete value from hash table.
    fn remove(&mut self, data: i32) {
        // Check if value is present.
        if !self.contains(data) {
            return;
        }

        let hash_index = self.hash_function(data);

        let item_index = self.table[hash_index].iter().position(|x| *x == data).unwrap();
        self.table[hash_index].swap_remove(item_index);
    }

    /// Hash function for 32-bit integers.
    fn hash_function(&self, key: i32) -> usize {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        let value = hasher.finish();
        return (value as usize) % HASH_TABLE_SIZE;
    }
}

fn main() {
    let mut hash_table = HashTable::new();

    println!("contains(3): {:?}", hash_table.contains(3));
    hash_table.insert(3);
    hash_table.insert(4);
    hash_table.insert(6);
    hash_table.insert(123);
    hash_table.insert(543);
    hash_table.insert(5678);
    hash_table.insert(123);
    println!("contains(3): {:?}", hash_table.contains(3));
    println!("contains(1): {:?}", hash_table.contains(1));
    println!("contains(123): {:?}", hash_table.contains(123));

    hash_table.remove(3);
    println!("contains(3): {:?}", hash_table.contains(3));
}
