/**
 * Calculate minimum weighted sum of completion times for scheduled jobs.
 * Jobs are scheduled once by difference of weight and length, and once by ratio.
 */
use std::fs;
use std::io::Error;

/// Represent a Job, with its length (completion time) and weight (priority).
struct Job {
    weight: i64,
    length: i64,
}
impl Job {
    fn new(weight: i64, length: i64) -> Job {
        Job { weight, length }
    }

    // Returns score by difference.
    fn score_by_diff(&self) -> i64 {
        self.weight - self.length
    }

    // Returns scrore by ratio.
    fn score_by_ratio(&self) -> f64 {
        self.weight as f64 / self.length as f64
    }
}

/// Returns minimim weighted completion times.
fn completion_times(jobs: &[Job]) -> i64 {
    let mut weighted_sum = 0;
    let mut last_completion_time = 0;

    for job in jobs.iter() {
        // This job's completion time.
        let comp_time = last_completion_time + job.length;
        weighted_sum += job.weight * comp_time;
        last_completion_time = comp_time;
    }

    return weighted_sum;
}

fn main() -> Result<(), Error> {
    // Try to open the file.
    let file_contents = fs::read_to_string("input_jobs.txt")?;

    // Jobs represented by their (weight, length)
    let mut jobs = Vec::new();

    // Read jobs from file.
    for line in file_contents.lines().skip(1) {
        let values = line
            .split_whitespace()
            .map(|v| v.parse::<i64>().unwrap())
            .collect::<Vec<i64>>();
        jobs.push(Job::new(values[0], values[1]));
    }

    // Case 1: Sort by decreasing order of difference, tie breaker is job weights.
    jobs.sort_by(|a, b| b.score_by_diff().cmp(&a.score_by_diff()).then(b.weight.cmp(&a.weight)));
    println!("When sorted by difference: {:?}", completion_times(&jobs));

    // Case 2: Sort by decreasing order of ratio.
    jobs.sort_by(|a, b| b.score_by_ratio().partial_cmp(&a.score_by_ratio()).unwrap());
    println!("When sorted by ratio: {:?}", completion_times(&jobs));

    Ok(())
}
