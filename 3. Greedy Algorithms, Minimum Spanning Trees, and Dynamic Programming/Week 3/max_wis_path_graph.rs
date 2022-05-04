/**
 * Calculate maximum weight-independent-set in path graph.
 * Also reconstruct the set from resulting max weight array.
 */
use std::cmp;
use std::convert::TryInto;
use std::fs;
use std::io::Error;

/// Returns the array of vertices included in max wis path graph.
fn get_max_wis_path_graph(weights: &[u64]) -> Vec<bool> {
    let mut max_set: Vec<u64> = vec![0; weights.len()];

    // Base case
    max_set[0] = 0;
    max_set[1] = weights[1];

    // Compute WIS array.
    for i in 2..weights.len() {
        max_set[i] = cmp::max(max_set[i - 1], max_set[i - 2] + weights[i]);
    }

    println!("Max weight = {}", max_set[weights.len() - 1]);

    // Boolean array thats true at places which are included in max WIS set.
    let mut included = vec![false; weights.len()];
    let mut index: i32 = (weights.len() - 1).try_into().unwrap();
    while index >= 1 {
        if max_set[index as usize] > max_set[index as usize - 1] {
            included[index as usize] = true;
            index -= 2;
        } else {
            index -= 1;
        }
    }

    return included;
}

fn main() -> Result<(), Error> {
    // Read input from file.
    let file_contents = fs::read_to_string("max_wis_input.txt")?
        .lines()
        .map(|v| v.parse::<u64>().expect("Failed to parse"))
        .collect::<Vec<u64>>();

    // Vertices start from 1. Put zero at 0th index.
    let mut weights = Vec::<u64>::new();
    weights.push(0);

    for weight in file_contents.iter().skip(1) {
        weights.push(*weight);
    }

    let included = get_max_wis_path_graph(&weights);
    for i in [1, 2, 3, 4, 17, 117, 517, 997].iter() {
        if included[*i as usize] {
            print!("1");
        } else {
            print!("0");
        }
    }

    println!("");

    Ok(())
}
