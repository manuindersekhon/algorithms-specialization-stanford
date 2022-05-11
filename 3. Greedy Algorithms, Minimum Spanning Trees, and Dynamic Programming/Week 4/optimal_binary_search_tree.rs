/**
 * Compute weighted cost of Optimal binary search tree.
 */

/// Returns the weighted cost. Solve smallest subproblems with fewest number of items first.
fn compute_optimal_bst(keys: &[i32], frequency: &[i32]) -> i32 {
    // DP array.
    let mut cost = vec![vec![0; keys.len()]; keys.len()];

    // Loop over every window for i <= j, and compute smallest to largest difference of i...j.
    for offset in 0..keys.len() {
        for i in 0..keys.len() - offset {
            // jth index behaves like a sliding window.
            let j = i + offset;

            // Try for optimal BST considering every element as root r in subset i..j. Hence,
            // reoccurence -> Freq(r) + Cost(i, r - 1) + Cost(r + 1, j)
            let freq_root = frequency[i..=j].iter().fold(0, |a, b| a + b);

            let mut min_cost = i32::MAX;
            for r in i..=j {
                // For out of bound values, consider cost as 0.
                let cost_left_subtree = if (i as i32) > (r as i32) - 1 { 0 } else { cost[i][r - 1] };
                let cost_right_subtree = if r + 1 > j { 0 } else { cost[r + 1][j] };

                min_cost = std::cmp::min(min_cost, freq_root + cost_left_subtree + cost_right_subtree);
            }

            cost[i][j] = min_cost;
        }
    }

    return cost[0][keys.len() - 1];
}

fn main() {
    // Dataset should be in sorted order of their keys.
    let keys = [10, 12, 20];
    let frequency = [34, 8, 50];

    let optimal_bst = compute_optimal_bst(&keys, &frequency);

    println!("{}", optimal_bst);
}
