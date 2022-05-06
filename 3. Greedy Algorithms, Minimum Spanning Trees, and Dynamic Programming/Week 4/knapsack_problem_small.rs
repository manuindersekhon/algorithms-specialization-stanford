/**
 * Compute the optinal solution for knapsack problem.
 */
use std::cmp;
use std::{fs, io::Result};

/// Struct representing knapsack item, first is value (or cost) and second is weight.
#[derive(Debug)]
struct Item(u32, u32);

// Returns the optimal solution.
fn optimal_solution(input: &[Item], capacity: usize) -> u32 {
    // Create a 2d array to calculate optimal solutions for subproblems using bottom-up approach.
    // Here, 2D array is made because optimal solution depends on 2 things, no. of items in subproblem and max weight.
    let mut solution_array: Vec<Vec<u32>> = vec![vec![0; capacity + 1]; input.len() + 1];

    for i in 1..input.len() + 1 {
        for x in 1..capacity + 1 {
            // We can't include this item, because weight is larger than capacity.
            if input[i - 1].1 > x as u32 {
                solution_array[i][x] = solution_array[i - 1][x];
            } else {
                // Try to include this item.
                solution_array[i][x] = cmp::max(
                    solution_array[i - 1][x],
                    solution_array[i - 1][x - input[i - 1].1 as usize] + input[i - 1].0,
                );
            }
        }
    }

    return solution_array[input.len()][capacity];
}

fn main() -> Result<()> {
    // Load onput from file.
    let file_contents = fs::read_to_string("knapsack_small_input.txt")?
        .lines()
        .map(|v| v.to_string())
        .collect::<Vec<String>>();

    let mut first_row = file_contents.first().unwrap().split_whitespace();

    // Maximum capacity of knapsack.
    let capacity = first_row.next().unwrap().parse::<usize>().expect("Failed to parse");

    // Total number of items.
    let _no_of_items = first_row.next().unwrap().parse::<u32>().expect("Failed to parse");

    let mut input: Vec<Item> = Vec::new();
    for line in file_contents.iter().skip(1) {
        let mut row = line.split_whitespace();
        let item = Item(
            row.next().unwrap().parse::<u32>().unwrap(),
            row.next().unwrap().parse::<u32>().unwrap(),
        );
        input.push(item);
    }

    println!("Solution = {}", optimal_solution(&input, capacity));

    Ok(())
}
