/**
 * Travelling salesman problem.
 * Calculate the minimum cost rounded tour that visits every city only once.
 */
use std::error::Error;
use std::fs;

// Point representation in 2D space.
struct Point {
    x: f64,
    y: f64,
}

/// Build distance matrix from city coordinates.
fn build_distance_matrix(coordinates: &[Point]) -> Vec<Vec<f64>> {
    let no_of_points = coordinates.len();
    let mut distance_matrix = vec![vec![0.0; no_of_points]; no_of_points];

    for i in 0..no_of_points {
        for j in 0..no_of_points {
            // Path from city to itself.
            if i == j {
                distance_matrix[i][j] = 0.0;
            } else {
                // Calculate using euclidean distance.
                let one = &coordinates[i];
                let two = &coordinates[j];
                distance_matrix[i][j] = f64::sqrt((one.x - two.x).powi(2) + (one.y - two.y).powi(2));
            }
        }
    }

    distance_matrix
}

/// Calculate travelling salesman route starting from from src. Cities are represented as bits in integer datatype.
/// Rightmost bit represents 1st city, and so on.
fn compute_tsp_route(num_points: usize, distance_matrix: &Vec<Vec<f64>>) -> f64 {
    // Total number of subsets is 2^n.
    let num_subsets = 1_usize << num_points;

    // Create memoization array that will help to lookup results for smaller subsets.
    let mut memo_array = vec![vec![f64::MAX; num_points]; num_subsets];

    // Distance from src to itself using only 1 vertex is 0.
    memo_array[1][0] = 0.0;

    // Iterate over subsets from smallest to largest.
    for subset_size in 2..=num_points {
        // Build all subsets of subset_size.
        for s in combinations(num_points, subset_size).iter() {
            let subset = *s;
            // Subset does not contain src, not needed for our solution.
            if subset & 1 == 0 {
                continue;
            }

            // Calculate distance from src to each destinations present in subset that visits all cities of this subset
            // exactly once.
            for j in 1..num_points {
                // This destination is not present in subset.
                if (1_usize << j) & subset == 0 {
                    continue;
                }

                // Calculate new distance using already computed smaller subsets.
                let mut min_value = f64::MAX;

                // Create copy of subset without destination j (this is subset with 1 smaller size which we computed
                // in previous first loop iteration).
                let prev_subset = subset & !(1_usize << j);

                for k in 0..num_points {
                    // Distance from k to j using j is not valid scenario, or k isn't present in subset.
                    if k == j || (1_usize << k) & subset == 0 {
                        continue;
                    }

                    // Get minimum distance of all possible combinations to visit from src to j.
                    min_value = min_value.min(memo_array[prev_subset][k] + distance_matrix[k][j]);
                }

                // Distance from src to j that visits every city in subset exactly once.
                memo_array[subset][j] = min_value;
            }
        }
    }

    // Compute minimum distance for one last hop back to src.
    let mut tsp_distance = f64::MAX;

    // Calculate the result for the final hop from shortest path back to src itself.
    for i in 1..num_points {
        tsp_distance = tsp_distance.min(memo_array[num_subsets - 1][i] + distance_matrix[i][0]);
    }

    return tsp_distance;
}

/// Choose r cities from n. This function generates C(n, r) with r bits set to 1. Initially starts with set all zeroes (0000).
/// Reccuernce relation is C(n, r) = C(n - 1, r - 1) or C(n - 1, r), i.e., either select current element or not.
fn combinations(n: usize, r: usize) -> Vec<usize> {
    let mut subsets = Vec::<usize>::new();
    generate_combinations(0, 0, n, r, &mut subsets);
    return subsets;
}

fn generate_combinations(mut set: usize, start_index: usize, n: usize, r: usize, subset: &mut Vec<usize>) {
    // Base case: C(n, 0)
    if r == 0 {
        subset.push(set);
        return;
    }

    // Generate combinations.
    for i in start_index..n {
        // Select this bit.
        set = set | (1_usize << i);

        generate_combinations(set, i + 1, n, r - 1, subset);

        // Unselect this bit.
        set = set & !(1_usize << i);
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    // Load city coordinates from file.
    let file_contents = fs::read_to_string("input_tsp.txt")?;
    let mut coordinates = Vec::<Point>::new();
    for line in file_contents.lines().skip(1) {
        let mut iter = line.split_whitespace();
        coordinates.push(Point {
            x: iter.next().unwrap().parse::<f64>()?,
            y: iter.next().unwrap().parse::<f64>()?,
        })
    }

    // Build distance matrix.
    let num_points = coordinates.len();
    let distance_matrix = build_distance_matrix(&coordinates);

    // Calculate travelling salesman route.
    println!("{}", compute_tsp_route(num_points, &distance_matrix).round());

    Ok(())
}
