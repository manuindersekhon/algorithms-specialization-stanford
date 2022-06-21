/**
 * Floyd Warshall algorithm.
 * Compute all pair shortest paths in O(n^3) in graphs with no negative cycles.
 */
use std::{env, error::Error, fs};

// Graph representation.
#[derive(Clone)]
struct Graph {
    vertices: usize,
    adj_list: Vec<Vec<Edge>>,
}

// Edge representation.
#[derive(Clone)]
struct Edge {
    head: usize,
    cost: i32,
}

impl Graph {
    // Create a new graph with n vertices.
    fn new(vertices: usize) -> Graph {
        Graph {
            vertices,
            adj_list: vec![Vec::<Edge>::new(); vertices],
        }
    }

    // Add a directed edge.
    fn add_edge(&mut self, src: usize, dest: usize, cost: i32) {
        self.adj_list[src].push(Edge { head: dest, cost });
    }

    // Compute all pair shortest paths, and return "shortest shortest distance".
    // Return None if negative cycle exists in graph.
    fn compute_all_pair_shortest_paths(&self) -> Option<i32> {
        // Create a 3D array (i, j, k) that stores all-pair shortest paths with internal nodes upto k.
        let mut lookup_table = vec![vec![vec![0; self.vertices]; self.vertices]; self.vertices];

        // Handle k = 0, i.e, shortest path using 0 internal nodes.
        for i in 0..self.vertices {
            for j in 0..self.vertices {
                // Path from vertex to itself.
                if i == j {
                    lookup_table[i][j][0] = 0;
                }
                // (i, j) has a directed edge.
                else if let Some(edge) = self.adj_list[i].iter().find(|edge| edge.head == j) {
                    lookup_table[i][j][0] = edge.cost;
                }
                // No path between i and j.
                else {
                    lookup_table[i][j][0] = i32::MAX;
                }
            }
        }

        // Save minimum of all shortest paths.
        let mut shortest_shortest_cost = i32::MAX;

        // Fill lookup table. Start with smallest subproblem k = 1, and slowly add more internal nodes to subproblem.
        for k in 1..self.vertices {
            for i in 0..self.vertices {
                for j in 0..self.vertices {
                    lookup_table[i][j][k] = std::cmp::min(
                        // Case 1: K isn't internal to P(i,j). Shortest path is with k - 1 nodes.
                        lookup_table[i][j][k - 1],
                        // Case 2: K is internal to P(i,j). Compute shortest path for both sides of vertex K.
                        // i -> k -> j, where both (i,k) and (k,j) use k - 1 internal nodes.
                        match lookup_table[i][k][k - 1] == i32::MAX || lookup_table[k][j][k - 1] == i32::MAX {
                            true => i32::MAX,
                            false => lookup_table[i][k][k - 1] + lookup_table[k][j][k - 1],
                        },
                    );

                    shortest_shortest_cost = std::cmp::min(shortest_shortest_cost, lookup_table[i][j][k]);
                }
            }
        }

        // Check if graph has a negative cycle.
        for i in 0..self.vertices {
            if lookup_table[i][i][self.vertices - 1] < 0 {
                return None;
            }
        }

        return Some(shortest_shortest_cost);
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    // Sanity check.
    if env::args().len() != 2 {
        eprintln!("usage: ./floyd_warshall <file_name>");
        std::process::exit(1);
    }

    // Try to read from file.
    let filename = env::args().last().expect("Failed to get filename");
    let file_content = fs::read_to_string(filename)?;
    let mut file_content = file_content.lines();

    // Get number of vertices from first line of file.
    let vertices = file_content
        .next()
        .unwrap()
        .split_whitespace()
        .next()
        .unwrap()
        .parse::<usize>()?;

    // Parse file contents into adjacency list representation.
    let mut graph = Graph::new(vertices);
    for line in file_content {
        let mut splitted = line.split_whitespace();
        let src = splitted.next().unwrap().parse::<usize>()? - 1;
        let dest = splitted.next().unwrap().parse::<usize>()? - 1;
        let cost = splitted.next().unwrap().parse::<i32>()?;

        graph.add_edge(src, dest, cost);
    }

    println!("Shortest shortest path: {:?}", graph.compute_all_pair_shortest_paths());

    Ok(())
}
