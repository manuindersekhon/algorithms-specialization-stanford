/**
 * You are given as input an unsorted array of n distinct numbers, where n is a power of 2. Give an algorithm that
 * identifies the second-largest number in the array, and that uses at most n + logn - 2 operations.
 */
use std::error::Error;
use std::io;

fn perform_knockout_tournament(elements: &[i32], max_element: i32) -> Vec<i32> {
    // Save those elements that were compared with max element in knockout tournament.
    let mut saved_elements: Vec<i32> = Vec::new();

    let mut remaining_elements = elements.clone().to_vec();
    while remaining_elements.len() > 1 {
        let mut result = Vec::new();

        // Perform knockout in pairs.
        for i in (0..remaining_elements.len()).step_by(2) {
            let first = remaining_elements[i];
            let second = remaining_elements[i + 1];
            result.push(std::cmp::max(first, second));

            if first == max_element || second == max_element {
                saved_elements.push(std::cmp::min(first, second));
            }
        }

        // Perform knockout on remaining elements.
        remaining_elements = result.clone().to_vec();
    }

    return saved_elements;
}

// Helper macro to read input.
macro_rules! read {
    ($out:ident as $type:ty) => {
        let mut inner = String::new();
        io::stdin().read_line(&mut inner).expect("A string");
        let $out = inner.trim().parse::<$type>().expect("Failed to parse");
    };
}

fn main() -> Result<(), Box<dyn Error>> {
    // Input number of elements.
    read!(num_elements as usize);

    // Input list of elements.
    let mut elements = vec![0; num_elements];
    for i in 0..num_elements {
        read!(input as i32);
        elements[i] = input;
    }

    // Find the largest number in the input.
    let max_element = *elements.iter().max().unwrap();

    // Perform the knockout tournament, and get those values that were compared with the max element.
    let compared_values = perform_knockout_tournament(&elements, max_element);

    println!("Second largest value = {}", compared_values.iter().max().unwrap());

    Ok(())
}
