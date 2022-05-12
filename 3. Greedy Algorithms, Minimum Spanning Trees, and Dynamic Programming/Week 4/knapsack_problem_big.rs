/**
 * Knapsack problem. Find the subset that has maximum value, and in limits of total knapsack capacity.
 * It uses a very large input file. Hence as optimisation, we use 2xW array for memoization.
 */
use std::{cmp, error::Error, fs};

/// Struct to store Knapsack item.
#[derive(Debug)]
struct Item {
    value: i32,
    weight: i32,
}

// Returns the optimal solution.
fn optimal_solution(input: &[Item], capacity: usize) -> i32 {
    // Create a 2d array to calculate optimal solutions for subproblems using bottom-up approach.
    // Here, 2D array is made because optimal solution depends on 2 things, no. of items in subproblem and max weight.
    let mut solution_array: Vec<Vec<i32>> = vec![vec![0; capacity + 1]; 2];

    for i in 1..input.len() + 1 {
        for x in 1..capacity + 1 {
            // We can't include this item, because weight is larger than capacity.
            if input[i - 1].weight > x as i32 {
                solution_array[i % 2][x] = solution_array[(i - 1) % 2][x];
            } else {
                // Try to include this item.
                solution_array[i % 2][x] = cmp::max(
                    solution_array[(i - 1) % 2][x],
                    solution_array[(i - 1) % 2][x - input[i - 1].weight as usize] + input[i - 1].value,
                );
            }
        }

        println!("Iteration {}", i);
    }

    return solution_array[input.len() % 2][capacity];
}

fn main() -> Result<(), Box<dyn Error>> {
    // Load input from file.
    let file_contents = fs::read_to_string("knapsack_big_input.txt")?
        .lines()
        .map(|v| v.to_string())
        .collect::<Vec<String>>();

    let mut first_row = file_contents.first().unwrap().split_whitespace();

    // Maximum capacity of knapsack.
    let capacity = first_row.next().unwrap().parse::<usize>()?;

    // Total number of items.
    let _no_of_items = first_row.next().unwrap().parse::<u32>()?;

    let mut input: Vec<Item> = Vec::new();
    for line in file_contents.iter().skip(1) {
        let mut row = line.split_whitespace();
        let item = Item {
            value: row.next().unwrap().parse::<i32>().unwrap(),
            weight: row.next().unwrap().parse::<i32>().unwrap(),
        };
        input.push(item);
    }

    println!("Solution = {}", optimal_solution(&input, capacity));

    Ok(())
}
