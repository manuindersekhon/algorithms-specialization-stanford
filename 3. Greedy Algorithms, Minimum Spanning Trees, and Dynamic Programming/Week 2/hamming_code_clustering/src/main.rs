/**
 * Find the largest value of k such that there is a k-clustering with spacing at least 3.
 * Graph input is so large that distances are given implicitly in Hamming Code.
 */
use std::collections::HashMap;
use std::fs;
use std::io::Error;

mod union_find;
use itertools::Itertools;
use union_find::UnionFind;

/// Returns the max number of clusters with spacing at least 3.
fn get_clusters(hamming_dataset: &Vec<Vec<char>>, vertices: i32) -> i32 {
    // Store mapping of vertex to their hamming codes.
    let mut hash_table = HashMap::<Vec<char>, i32>::new();

    // All vertices are in different clusters initially.
    let mut no_of_clusters = vertices;
    let mut union_find = UnionFind::new();
    for vertex in 0..vertices {
        union_find.add(vertex);
    }

    // Load dataset into hash table and check for spacing 0 (duplicate values).
    for vertex in 0..vertices {
        // Check if this hamming code already belongs to any vertex.
        let hamming_code = hamming_dataset.get(vertex as usize).unwrap();
        if hash_table.contains_key(hamming_code) {
            union_find.union(*hash_table.get(hamming_code).unwrap(), vertex);
            no_of_clusters -= 1;
        } else {
            hash_table.insert(hamming_code.clone(), vertex);
        }
    }

    // Loop over every vertex again and check for spacing 1 and 2.
    for vertex in 0..vertices {
        let hamming_code = hamming_dataset.get(vertex as usize).unwrap();

        // Get all possible permutations for this hamming code with 1 and 2 bits inverted respectively.
        for item in possible_permutations(hamming_code).iter() {
            // This combination is valid.
            if hash_table.contains_key(item) {
                if union_find.find(vertex) != union_find.find(*hash_table.get(item).unwrap()) {
                    // Merge into one cluster.
                    union_find.union(vertex, *hash_table.get(item).unwrap());
                    no_of_clusters -= 1;
                }
            }
        }
    }

    no_of_clusters
}

// Iterator the generates inverted permutations with 1 and 2 bits inverted.
fn possible_permutations(hamming_code: &[char]) -> Vec<Vec<char>> {
    let mut result = Vec::<Vec<char>>::new();

    // Spacing = 1.
    for index in 0..hamming_code.len() {
        let mut copy = hamming_code.to_vec();
        copy[index] = invert_bit(copy[index]);
        result.push(copy);
    }

    // Spacing = 2.
    for item in (0..hamming_code.len()).permutations(2) {
        // Get unique combinations only.
        if item[0] < item[1] {
            let mut copy = hamming_code.to_vec();
            copy[item[0]] = invert_bit(copy[item[0]]);
            copy[item[1]] = invert_bit(copy[item[1]]);
            result.push(copy);
        }
    }

    result
}

// Returns the inverted bit.
fn invert_bit(c: char) -> char {
    match c {
        '0' => '1',
        '1' => '0',
        _ => '1',
    }
}

fn main() -> Result<(), Error> {
    // Read data from file.
    let file_contents = fs::read_to_string("src/clustering_big_input.txt")?
        .lines()
        .map(|v| v.to_string())
        .collect::<Vec<String>>();

    // First line of file is the number of vertices.
    let vertices = file_contents
        .first()
        .unwrap()
        .split_whitespace()
        .next()
        .unwrap()
        .parse::<i32>()
        .expect("Failed to parse vertices");

    // Get hamming codes for vertices from file.
    let mut hamming_dataset = Vec::<Vec<char>>::new();
    for line in file_contents.iter().skip(1) {
        hamming_dataset.push(line.split_whitespace().map(|v| v.chars().next().unwrap()).collect_vec());
    }

    let clusters = get_clusters(&hamming_dataset, vertices);
    println!("{}", clusters);

    Ok(())
}
