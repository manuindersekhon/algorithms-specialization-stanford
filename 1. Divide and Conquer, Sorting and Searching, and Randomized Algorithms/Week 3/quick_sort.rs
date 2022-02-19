/**
 * Quick Sort, example of randomized algorithms.
 */
use std::io::stdin;

// Pivot type to choose for partition.
enum PivotType {
    First,
    Last,
    MedianOfThree,
}

// Sorts the array using quick sort, and returns number of comparisons done.
fn quick_sort(arr: &mut [i32], pivot_type: &PivotType) -> i32 {
    // Base case, array with zero or one item is already sorted.
    if arr.len() <= 1 {
        return 0;
    }

    // Choose first item as pivot index.
    let pivot_index: usize = get_pivot_index(&arr, &pivot_type);

    // Bring pivot to first place of array.
    arr.swap(0, pivot_index);

    // Start of the wall.
    let mut wall: usize = 0;

    for index in 1..arr.len() {
        if arr[index] < arr[0] {
            // Move wall to make more space.
            wall += 1;
            // Bring element to left of wall.
            arr.swap(index, wall);
        }
    }

    // Bring pivot to its rightful position.
    arr.swap(0, wall);

    // Sort left half of wall.
    let left_comparisons: i32 = quick_sort(&mut arr[..wall], &pivot_type);

    // Sort right half of wall.
    let right_comparisons: i32 = quick_sort(&mut arr[wall + 1..], &pivot_type);

    // Number of comparisons are always equal to len - 1.
    return left_comparisons + right_comparisons + (arr.len() as i32 - 1);
}

// Get the pivot index based on the type we need. (first element, last element, or median of three rule).
fn get_pivot_index(arr: &[i32], pivot_type: &PivotType) -> usize {
    match pivot_type {
        PivotType::First => 0,
        PivotType::Last => arr.len() - 1,
        PivotType::MedianOfThree => {
            let middle_index = arr.len() / 2;

            // For element count less then 3, we can't compute median of three.
            if arr.len() < 3 {
                return middle_index;
            }

            if arr[0] < arr[middle_index] && arr[middle_index] < *arr.last().unwrap() {
                // Middle element is the median
                return middle_index;
            } else if arr[middle_index] < arr[0] && arr[0] < *arr.last().unwrap() {
                // First element is the median
                return 0;
            } else {
                // Last element is the median
                return arr.len() - 1;
            }
        }
    }
}

fn main() {
    let length: usize = 10000;
    let mut input_array: Vec<i32> = Vec::with_capacity(length);

    // Load dataset.
    for _ in 0..length {
        let mut input_buffer: String = String::new();
        stdin()
            .read_line(&mut input_buffer)
            .expect("Failed to read value");

        // Parse into to integer.
        input_array.push(
            input_buffer
                .trim()
                .parse()
                .expect("Failed to convert to int"),
        );
    }

    // Sort the array.
    let comparisons1: i32 = quick_sort(&mut input_array.clone(), &PivotType::First);
    let comparisons2: i32 = quick_sort(&mut input_array.clone(), &PivotType::Last);
    let comparisons3: i32 = quick_sort(&mut input_array.clone(), &PivotType::MedianOfThree);

    // Print the sorted result.
    println!("Sorting done");
    println!(
        "Comparisons1 = {}, Comparisons1 = {}, Comparisons1 = {}",
        comparisons1, comparisons2, comparisons3
    );
}
