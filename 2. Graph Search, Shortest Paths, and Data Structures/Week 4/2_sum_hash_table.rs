/**
 * 2-Sum Problem variant.
 * compute the number of target values t in the interval [-10000,10000] (inclusive) such that there are distinct numbers
 * x, y such that x + y = t.
 */
use std::collections::HashSet;
use std::fs;
use std::io::Error;

fn main() -> Result<(), Error> {
    // Load input from file.
    let file_contents = fs::read_to_string("2sum_input.txt")?;

    let mut target_values = 0;
    let mut hashset = HashSet::new();
    let mut seen = HashSet::new();

    for num in file_contents.lines() {
        let number = num.parse::<i64>().unwrap();
        hashset.insert(number);
    }
    println!("File loaded");

    // Loop over interval 't'
    for t in -10000..=10000 {
        println!("Iteration {}", t + 10000);

        for y in hashset.iter() {
            // Compute x = t - y and check if x is present in hashtable.
            let x = t - *y;

            // We have to check for distinct numbers only
            if x != *y && hashset.contains(&x) {
                let seen_x = seen.contains(&x);
                let seen_y = seen.contains(y);
                let seen_t = seen.contains(&t);

                // Check if numbers and target are distinct.
                match seen_x || seen_y || seen_t {
                    true => {}
                    false => {
                        target_values += 1;
                        seen.insert(x);
                        seen.insert(*y);
                        seen.insert(t);
                    }
                }
            }
        }
    }

    println!("Number of target values = {}", target_values);

    Ok(())
}
