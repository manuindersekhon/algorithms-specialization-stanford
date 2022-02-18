/**
 * Selection Problem.
 * Find ith order statistics in O(n) time. (ith minimum element in array)
 *
 * Use the variation of quick sort, this helps give us correct position of pivot.
 */
use std::collections::HashSet;
use std::env;
use std::process;

// Return ith minimum element in array, or None if not found.
fn select(array: &mut [i32], ith_order: i32) -> Option<i32> {
    // Ith min element not found.
    if array.is_empty() {
        return None;
    }

    // Get random pivot element, and bring it to the start.
    array.swap(0, random() % array.len() as usize);
    let mut wall = 0;

    // Maintain set of unique elements before the pivot.
    let mut unique_items = HashSet::new();

    for index in 1..array.len() {
        // bring smaller elements to left of the wall.
        if array[index] <= array[0] {
            if array[index] != array[0] {
                unique_items.insert(array[index]);
            }
            wall += 1;
            array.swap(wall, index);
        }
    }

    // Bring pivot to its rightful position.
    array.swap(0, wall);

    // Get the real ith order statistics by ignoring the duplicate elements.
    let real_ith_order = unique_items.len() as i32;

    // Pivot itself is the desired value.
    if ith_order == real_ith_order {
        return Some(array[wall]);
    } else if ith_order < real_ith_order {
        return select(&mut array[..wall], ith_order);
    } else {
        return select(&mut array[wall + 1..], ith_order - real_ith_order - 1);
    }
}

fn main() {
    // Ensure proper usage.
    let args = env::args().skip(1).collect::<Vec<String>>();
    if args.len() != 1 {
        eprintln!("Usage: ./randomized_selection <ith order statistics>");
        process::exit(1);
    }

    let ith_order = (*args.first().unwrap()).parse::<i32>().unwrap();
    assert!(ith_order > 0);

    // Test cases
    let mut data1 = [2, 1, 3, 6, 5, 8, 0];
    let mut data2 = [2, 2, 3, 4, 8, 2];
    let mut data3 = [2];
    let mut data4 = [2, 2, 2, 2, 2, 2, 2];

    let result1 = select(&mut data1, ith_order - 1);
    println!("result1 = {:?}", result1);

    let result2 = select(&mut data2, ith_order - 1);
    println!("result2 = {:?}", result2);

    let result3 = select(&mut data3, ith_order - 1);
    println!("result3 = {:?}", result3);

    let result4 = select(&mut data4, ith_order - 1);
    println!("result4 = {:?}", result4);
}

// Use random number from C (Rust doesn't have a builtin one)
extern "C" {
    fn srand(seed: u32);
    fn time(value: i32) -> u32;
    fn rand() -> usize;
}
fn random() -> usize {
    unsafe {
        srand(time(0));
        rand()
    }
}
