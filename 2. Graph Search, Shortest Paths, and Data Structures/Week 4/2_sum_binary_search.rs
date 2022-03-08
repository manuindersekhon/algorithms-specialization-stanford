/**
 * 2-Sum variant with binary search. (Use concept of sliding window to stay in range -10000 to 100000)
 * NOTE: Array should be sorted.
 */
use std::fs;
use std::{collections::HashSet, io::Error};

fn main() -> Result<(), Error> {
    // Load input from file.
    let file_contents = fs::read_to_string("2sum_input.txt")?;

    let mut hashset = HashSet::<i64>::new();

    // Set to store the sums. (sums also have to be unique).
    let mut sums = HashSet::<i64>::new();

    // Load values into hashset for de-duplication.
    for num in file_contents.lines() {
        let number = num.parse::<i64>().unwrap();
        hashset.insert(number);
    }

    let mut array = hashset.iter().map(|val| *val).collect::<Vec<i64>>();
    array.sort_unstable();

    let mut start: usize = 0;
    let mut end = array.len() - 1;

    while start <= end {
        // Compute sum.
        let target = array[start] + array[end];

        // Slide window in range.
        if target > 10000 {
            end -= 1;
        } else if target < -10000 {
            start += 1;
        }
        // Target is in range.
        else {
            sums.insert(target);
            start += 1;
        }
    }

    println!("Target values = {}", sums.len());

    Ok(())
}
