/**
 * Union Find Data Structure.
 * Uses union by rank and leverages path-compression for optimizations.
 */
use std::collections::HashMap;

// Node for the disjoint set.
#[derive(Debug)]
struct Node {
    // Max number of hops from leaf to this node.
    rank: usize,
    // Parent of this node.
    parent: i32,
}

impl Node {
    // Create a new set node.
    fn new(rank: usize, parent: i32) -> Node {
        Node { rank, parent }
    }
}

// Union Find data structure.
#[derive(Debug)]
pub struct UnionFind {
    disjoint_sets: HashMap<i32, Node>,
}

impl UnionFind {
    // Returns an empty union find data structure.
    pub fn new() -> UnionFind {
        UnionFind {
            disjoint_sets: HashMap::new(),
        }
    }

    // Adds an object to union find. New object will have rank 0 and it will be its own parent.
    pub fn add(&mut self, item: i32) {
        self.disjoint_sets.insert(item, Node::new(0, item));
    }

    // Do a union by rank of two disjoint sets.
    pub fn union(&mut self, x: i32, y: i32) {
        // Find the parent of both items.
        let parent_x = self.find(x);
        let parent_y = self.find(y);

        // They already belong to same set.
        if parent_x == parent_y {
            return;
        }

        // Parent_y has less height that parent_x. Attach y set to x.
        if self.disjoint_sets.get(&parent_x).unwrap().rank > self.disjoint_sets.get(&parent_y).unwrap().rank {
            self.disjoint_sets.get_mut(&parent_y).unwrap().parent = parent_x;
        }
        // Parent_x has less height that parent_y.
        else if self.disjoint_sets.get(&parent_x).unwrap().rank < self.disjoint_sets.get(&parent_y).unwrap().rank {
            self.disjoint_sets.get_mut(&parent_x).unwrap().parent = parent_y;
        }
        // Same height. Adjust ranks.
        else {
            self.disjoint_sets.get_mut(&parent_y).unwrap().parent = parent_x;
            self.disjoint_sets.get_mut(&parent_x).unwrap().rank += 1;
        }
    }

    // Returns the parent of [x].
    pub fn find(&mut self, x: i32) -> i32 {
        if self.disjoint_sets.get(&x).unwrap().parent != x {
            // Apply path compression.
            self.disjoint_sets.get_mut(&x).unwrap().parent = self.find(self.disjoint_sets.get(&x).unwrap().parent);
        }

        // This is the leader of this disjoint set.
        return self.disjoint_sets.get(&x).unwrap().parent;
    }
}
