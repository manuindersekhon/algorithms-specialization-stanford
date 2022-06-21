/**
 * Knapsack problem solved using heuristics.
 * Correctness depends on the factor epsilon. Less the value, more correct the answer will be, hence less performance.
 */
use std::cmp;
use std::error::Error;
use std::fs;

/// Epsilon factor to treak correctness of algorithm. Guarantees a 70% correct approximation.
const EPSILON: f64 = 0.30;

/// Represents the knapsack item.
#[derive(Debug)]
struct Item {
    value: usize,
    weight: usize,
}

impl Item {
    /// Builds a new item.
    fn new(value: usize, weight: usize) -> Item {
        Item { value, weight }
    }
}

/// Calculate heuristic solution for knapsack items. Item values are reduced by factor of epsilon.
fn knapsack_heuristic(items: &[Item], capacity: usize) -> usize {
    let num_items = items.len();

    let mut v_max = items.first().unwrap().value;
    for item in items.iter() {
        v_max = cmp::max(v_max, item.value);
    }

    // Maximum possible value for entire dataset.
    let max_value = num_items * v_max;

    // Calculate minimum total weight needed to achieve the values incremently using only i items.
    let mut lookup_table = vec![vec![usize::MAX; max_value]; 2];

    // Base case
    lookup_table[num_items % 2][0] = 0;

    // Incremently include new item and recalculate min weight needed to accomodate this item.
    for i in 1..num_items {
        for x in 0..max_value {
            // This item's value is larger than our subproblem constraint.
            if items[i].value > x {
                continue;
            }

            lookup_table[i % 2][x] = cmp::min(
                // Case 1: Don't include ith item.
                lookup_table[(i - 1) % 2][x],
                // Case 2: Try to include ith item.
                items[i].weight + lookup_table[(i - 1) % 2][x - items[i].value],
            );
        }
    }

    // Find the maximum value possible that uses all the items and is in bounds of our capacity.
    let mut answer = 0;
    for x in 1..max_value {
        if lookup_table[num_items % 2][x] <= capacity {
            answer = cmp::max(answer, lookup_table[num_items % 2][x]);
        }
    }

    return answer;
}

fn main() -> Result<(), Box<dyn Error>> {
    // Load input from file.
    let file_contents = fs::read_to_string("knapsack_big_input.txt")?;

    // Get total items and max capacity from first file of file.
    let mut first_row = file_contents.lines().next().unwrap().split_whitespace();
    let capacity = first_row.next().unwrap().parse::<usize>()?;
    let num_items = first_row.next().unwrap().parse::<usize>()?;

    // Load items from file.
    let mut items = Vec::<Item>::new();
    for line in file_contents.lines().skip(1) {
        let mut row = line.split_whitespace();
        items.push(Item::new(
            row.next().unwrap().parse::<usize>()?,
            row.next().unwrap().parse::<usize>()?,
        ));
    }

    // Get the item with max value.
    let mut max_value = items.first().unwrap().value;
    for item in items.iter() {
        max_value = cmp::max(max_value, item.value);
    }

    // Epsilon factor. More the value of this variable, less the accuracy, more performance.
    let m = (EPSILON * max_value as f64) / (num_items as f64);

    // Round down the values of items to closest multiple of m.
    for item in items.iter_mut() {
        item.value = (item.value as f64 / m).floor() as usize;
    }

    // Calculate approximate max value from this knapsack instance.
    let max_value = knapsack_heuristic(&items, capacity);
    println!("Maximum value = {}", max_value);

    Ok(())
}
