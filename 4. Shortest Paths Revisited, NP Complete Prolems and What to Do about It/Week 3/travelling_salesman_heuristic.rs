/**
 * Travelling salesman problem solved using nearest neighbour heuristics.
 * Time complexity is O(n^2)
 */
use std::{error::Error, fs};

// Coordinates for the city.
#[derive(Debug)]
struct Coordinate {
    x: f64,
    y: f64,
}

impl Coordinate {
    // Return squared euclidean distance.
    fn squared_euclidean_distance(&self, other: &Coordinate) -> f64 {
        (self.x - other.x).powi(2) + (self.y - other.y).powi(2)
    }
}

// Calculate shortest distance using nearest neighbour, rounded down to the nearest integer.
fn travelling_salesman_heuristic(cities: &[Coordinate]) -> u32 {
    // Mark all cities as unvisited.
    let mut visited = vec![false; cities.len()];

    // Starting city.
    let starting_city: usize = 0;
    let mut current_city = starting_city;

    // Maintain total distance covered during the tour.
    let mut total_distance: f64 = 0.0;

    // Loop until we don't visit all cities exactly once.
    loop {
        // Mark this city as visited.
        visited[current_city] = true;

        let mut nearest_neighbour_dist: f64 = f64::MAX;
        let mut nearest_neighbour = current_city;
        let mut visited_new_city = false;

        // Find the nearest neighbour from the current city.
        for city_index in 0..cities.len() {
            // Don't visit same city again.
            if !visited[city_index] {
                visited_new_city = true;
                // Calculate euclidean distance b/w current and this city.
                let dist = cities[city_index].squared_euclidean_distance(&cities[current_city]);
                if dist < nearest_neighbour_dist {
                    nearest_neighbour_dist = dist;
                    nearest_neighbour = city_index;
                }
            }
        }

        if visited_new_city {
            // Get standard euclidean distance value.
            total_distance += nearest_neighbour_dist.sqrt();
            // Go to this city.
            current_city = nearest_neighbour;
        } else {
            // We have visited all cities once.
            break;
        }
    }

    // Calculate one final hop from current city back to the starting city.
    total_distance += cities[current_city]
        .squared_euclidean_distance(&cities[starting_city])
        .sqrt();

    return total_distance.floor() as u32;
}

fn main() -> Result<(), Box<dyn Error>> {
    // Load coordinates from file.
    let file_contents = fs::read_to_string("input_tsp.txt")?;
    let mut file_iter = file_contents.lines();

    // Total number of cities.
    let num_cities = file_iter.next().unwrap().parse::<usize>()?;
    let mut cities = Vec::<Coordinate>::with_capacity(num_cities);

    for line in file_iter {
        let mut row = line.split_whitespace();
        // Drop the city number.
        row.next();

        // Get the coordinates.
        let x = row.next().unwrap().parse::<f64>()?;
        let y = row.next().unwrap().parse::<f64>()?;
        cities.push(Coordinate { x, y });
    }

    println!("{}", travelling_salesman_heuristic(&cities));

    Ok(())
}
