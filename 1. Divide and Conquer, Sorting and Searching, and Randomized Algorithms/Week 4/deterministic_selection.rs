/**
 * Selection problem solved using median-of-medians technique.
 *
 * Find ith order statistics in an array of distinct numbers in O(n) time.
 */
use std::convert::TryInto;

// Return ith order statistics, or None.
fn deterministic_select(arr: &mut [i32], ith_order: i32) -> Option<i32> {
    // ith order statistics can't be bigger than the array.
    if ith_order > arr.len().try_into().unwrap() || ith_order < 0 {
        return None;
    }

    // Desired element not found.
    if arr.is_empty() {
        return None;
    }

    if arr.len() == 1 {
        return Some(arr[0]);
    }

    // Apply median-of-medians trick. Divide array into chunks of size 5.
    let mut chunks: Vec<Vec<i32>> = arr.chunks_mut(5).map(|chunk| chunk.to_vec()).collect();

    // Sort partitions.
    for chunk in chunks.iter_mut() {
        chunk.sort_unstable();
    }

    // Get medians from sorted chunks.
    let mut medians: Vec<i32> = chunks.iter().map(|chunk| get_median(&chunk)).collect();

    // Find median of medians.
    let final_median_index = (medians.len() / 2) as i32;
    let final_median = deterministic_select(&mut medians, final_median_index).unwrap();

    // Get index of median. We will consider this as pivot index.
    let mut pivot_index: usize = 0;
    for item in arr.iter().enumerate() {
        if *item.1 == final_median {
            pivot_index = item.0;
            break;
        }
    }

    // Use Quick sort partition routine to find the ith minimum.
    arr.swap(0, pivot_index);
    let mut wall = 0;

    for index in 1..arr.len() {
        if arr[index] < arr[0] {
            wall += 1;
            arr.swap(wall, index);
        }
    }

    arr.swap(0, wall);

    if ith_order == wall.try_into().unwrap() {
        return Some(arr[wall]);
    } else if ith_order < wall.try_into().unwrap() {
        return deterministic_select(&mut arr[..wall], ith_order);
    } else {
        return deterministic_select(&mut arr[wall + 1..], ith_order - (wall as i32) - 1);
    }
}

// Get median from sorted array.
fn get_median(arr: &[i32]) -> i32 {
    if arr.len() % 2 != 0 {
        return arr[arr.len() / 2];
    } else {
        return arr[(arr.len() + 1) / 2];
    }
}

fn main() {
    let mut data1 = [10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 12, 11, 14, 13];
    let ith_order_statistics = 5;

    let result = deterministic_select(&mut data1, ith_order_statistics - 1);

    println!("{:?}", result);
}
