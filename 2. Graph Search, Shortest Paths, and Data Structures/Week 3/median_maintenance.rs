/**
 * Median Maintenance problem.
 * Maintain two heaps, min and max to compute medians of running numbers in O(log n) time.
 */
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::fs;
use std::io::Error;

fn main() -> Result<(), Error> {
    let filename = "median_input.txt";

    // Maintain to heaps to accomodate min and max values.
    let mut min_heap = BinaryHeap::<Reverse<i32>>::new();
    let mut max_heap = BinaryHeap::<i32>::new();

    let mut median_sum = 0;

    // Try to read from file.
    let file_contents = fs::read_to_string(filename)?;
    for number in file_contents.lines() {
        let num = number.parse::<i32>().unwrap();

        // Push element to correct heap.
        if max_heap.is_empty() {
            max_heap.push(num);
        } else if num < *max_heap.peek().unwrap() {
            max_heap.push(num);
        } else {
            min_heap.push(Reverse(num));
        }

        // Resize heaps to maintain equal size for both heaps.
        if (max_heap.len() as isize) - (min_heap.len() as isize) > 1 {
            let max_top = max_heap.pop().unwrap();
            min_heap.push(Reverse(max_top));
        } else if (min_heap.len() as isize) - (max_heap.len() as isize) > 1 {
            max_heap.push(min_heap.pop().unwrap().0);
        }

        if min_heap.len() > max_heap.len() {
            median_sum += min_heap.peek().unwrap().0;
        } else {
            median_sum += *max_heap.peek().unwrap();
        }
    }

    println!("{}", median_sum);

    Ok(())
}
