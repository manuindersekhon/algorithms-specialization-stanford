/**
 * You are a given a unimodal array of n distinct elements, meaning that its entries are in increasing order up until
 * its maximum element, after which its elements are in decreasing order. Give an algorithm to compute the maximum
 * element that runs in O(log n) time.
 */
use std::error::Error;
use std::io;

// Helper function to read input.
macro_rules! read {
    ($out:ident as $type:ty) => {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        let $out = input.trim().parse::<$type>().unwrap();
    };
}

fn find_max_element(elements: &[i32]) -> Option<i32> {
    let mut start_index: i32 = 0;
    let mut end_index: i32 = elements.len() as i32 - 1;

    // Do a binary search on input.
    while start_index <= end_index {
        let middle_index = ((start_index + end_index) / 2) as i32;

        if middle_index - 1 >= start_index && elements[middle_index as usize] < elements[middle_index as usize - 1] {
            end_index = middle_index - 1;
        } else if middle_index + 1 <= end_index && elements[middle_index as usize] < elements[middle_index as usize + 1] {
            start_index = middle_index + 1;
        } else {
            return Some(elements[middle_index as usize]);
        }
    }

    // Array is not a unimodel array.
    return None;
}

fn main() -> Result<(), Box<dyn Error>> {
    // Input number of elements.
    read!(num_elements as usize);

    // Input elements.
    let mut elements = vec![0; num_elements];
    for i in 0..num_elements {
        read!(input as i32);
        elements[i] = input;
    }

    // Find maximum element.
    let max_element = find_max_element(&elements);
    println!("Max element = {:?}", max_element);

    Ok(())
}
