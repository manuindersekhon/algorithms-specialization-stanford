/**
 * Sequence alignment problem.
 * Calculate similarity measure between two strings, and compute the string itself.
 */
use std::{cmp::min, env, process};

/// Returns lookup table with minimum penalties.
fn get_lookup_table(
    string1: &[char],
    string2: &[char],
    gap_penalty: usize,
    mismatch_penalty: usize,
) -> Vec<Vec<usize>> {
    // Create the lookup table.
    let mut lookup_table: Vec<Vec<usize>> = vec![vec![0; string2.len() + 1]; string1.len() + 1];

    // Fill 0th row and 0th column with gap penalties, because (i, 0) means match string1 with 0 characters
    // of string2 i.e. gaps.
    for i in 0..=string1.len() {
        lookup_table[i][0] = i * gap_penalty;
    }
    for i in 0..=string2.len() {
        lookup_table[0][i] = i * gap_penalty;
    }

    // Loop over all the characters of strings to calculate penalties.
    for i in 1..=string1.len() {
        for j in 1..=string2.len() {
            // Case 1: character matched with the character.
            let mut case1: usize = 0 + lookup_table[i - 1][j - 1];
            if string1[i - 1] != string2[j - 1] {
                case1 = mismatch_penalty + lookup_table[i - 1][j - 1];
            }

            // Case 2: X(i) is matched with the gap in Y.
            let case2: usize = gap_penalty + lookup_table[i - 1][j];

            // Case 2: Y(j) is matched with the gap in X.
            let case3: usize = gap_penalty + lookup_table[i][j - 1];

            lookup_table[i][j] = min(min(case1, case2), case3);
        }
    }

    return lookup_table;
}

fn main() {
    let args = env::args();

    // Sanity check
    if args.len() != 5 {
        eprintln!("usage: ./sequence_alignment <string1> <string2> <gap_penalty> <mismatch_penalty>");
        process::exit(1);
    }

    // Get data from command line arguments.
    let args = args.map(|v| v).collect::<Vec<String>>();
    let string1 = args.get(1).unwrap().chars().collect::<Vec<char>>();
    let string2 = args.get(2).unwrap().chars().collect::<Vec<char>>();
    let gap_penalty = args.get(3).unwrap().parse::<usize>().expect("Failed to parse");
    let mismatch_penalty = args.get(4).unwrap().parse::<usize>().expect("Failed to parse");

    let lookup_table = get_lookup_table(&string1, &string2, gap_penalty, mismatch_penalty);

    println!("Penalty = {}", lookup_table.last().unwrap().last().unwrap());
}
