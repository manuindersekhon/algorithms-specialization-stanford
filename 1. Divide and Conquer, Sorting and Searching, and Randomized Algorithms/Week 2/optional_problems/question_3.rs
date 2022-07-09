/**
 * You are given a sorted (from smallest to largest) array A of n distinct integers which can be positive, negative,
 * or zero. You want to decide whether or not there is an index i such that A[i] = i. Design the fastest algorithm
 * that you can for solving this problem. (Can be solved in O(log n))
 */
use std::error::Error;
use std::io;

// Helper macro to read input.
macro_rules! read {
    ($out:ident as $type:ty) => {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("failed to read input");
        let $out = input.trim().parse::<$type>().expect("failed to parse");
    };
}

// Returns that index where a[i] = i, None otherwise.
fn find_result(elements: &[i32]) -> Option<usize> {
    let mut start_index = 0_i32;
    let mut end_index = elements.len() as i32 - 1;

    // Perform binary search on array.
    while start_index <= end_index {
        let middle_index: i32 = (start_index + end_index) / 2 as i32;

        if elements[middle_index as usize] < middle_index {
            start_index = middle_index + 1;
        } else if elements[middle_index as usize] > middle_index {
            end_index = middle_index - 1;
        } else {
            return Some(middle_index as usize);
        }
    }

    return None;
}

fn main() -> Result<(), Box<dyn Error>> {
    // Input number of elements.
    read!(num_elements as usize);

    // Input numbers.
    let mut elements: Vec<i32> = vec![0; num_elements];
    for i in 0..num_elements {
        read!(input as i32);
        elements[i] = input;
    }

    let result_index = find_result(&elements);
    println!("Result = {:?}", result_index);

    Ok(())
}
