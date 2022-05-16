/**
 * Johnson's algorithm to compute All pair shortest paths.
 * Idea: Convert negative edge weights to non-negative weights so that we can run Dijkstra N times to compute APSP.
 */
use std::collections::BinaryHeap;
use std::{cmp, env, error::Error, fs};

// Graph representation.
#[derive(Clone)]
struct Graph {
    vertices: usize,
    adj_list: Vec<Vec<Edge>>,
    // Maintain reverse graph, needed for Bellman-Ford.
    rev_adj_list: Vec<Vec<Edge>>,
}

// Edge representation.
#[derive(Clone)]
struct Edge {
    head: usize,
    cost: i32,
}

// Binary heap node for Dijkstra algorithm.
#[derive(PartialEq, Eq)]
struct HeapNode {
    distance: i32,
    vertex: usize,
}

// Make it behave as min heap.
impl Ord for HeapNode {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.distance.cmp(&self.distance)
    }
}
impl PartialOrd for HeapNode {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(&other))
    }
}

impl Graph {
    // Build a new graph with vertices 1..n. 0th vertex will be the dummy one to compute vertex weights.
    fn new(vertices: usize) -> Graph {
        Graph {
            vertices,
            adj_list: vec![Vec::<Edge>::new(); vertices + 1],
            rev_adj_list: vec![Vec::<Edge>::new(); vertices + 1],
        }
    }

    // Add a directed edge.
    fn add_edge(&mut self, src: usize, dest: usize, cost: i32) {
        self.adj_list[src].push(Edge { head: dest, cost });
        self.rev_adj_list[dest as usize].push(Edge { head: src, cost });
    }

    // Compute All-Pair Shortest paths, and return the shortest-shortest-distance.
    fn compute_all_pair_shortest_paths(&mut self) -> Option<i32> {
        // Connect a dummy vertex 0 to all vertices to ensure reachibility.
        for i in 1..=self.vertices {
            self.add_edge(0, i, 0);
        }

        // Run Bellman-Ford from dummy vertex.
        let bellman_ford_result = self.run_bellman_ford(0);

        // Negative cycle is the graph.
        if bellman_ford_result.is_none() {
            return None;
        }
        let vertex_weights = bellman_ford_result.unwrap();

        // Reweight all the edges (u,v) by factor, C(e) + Pu - Pv.
        for vertex in 1..=self.vertices {
            for edge in self.adj_list[vertex].iter_mut() {
                edge.cost = edge.cost + vertex_weights[vertex] - vertex_weights[edge.head];
            }
        }

        // Run Dijkstra's SSSP for all vertices.
        let mut all_pair_shortest_paths = vec![Vec::<i32>::new(); self.vertices + 1];
        for vertex in 1..=self.vertices {
            all_pair_shortest_paths[vertex] = self.run_dijkstra(vertex);
        }

        // Reset edge lengths to its original weights, and store min result.
        let mut shortest_shortest_dist = i32::MAX;
        for u in 1..=self.vertices {
            for v in 1..=self.vertices {
                all_pair_shortest_paths[u][v] = all_pair_shortest_paths[u][v] - vertex_weights[u] + vertex_weights[v];
                shortest_shortest_dist = cmp::min(shortest_shortest_dist, all_pair_shortest_paths[u][v]);
            }
        }

        return Some(shortest_shortest_dist);
    }

    // Run Bellman Ford from source vertex, return shortest paths or None if cycle exists.
    fn run_bellman_ford(&self, src: usize) -> Option<Vec<i32>> {
        // Build lookup table. All vertices are infinite distance apart by default.
        let mut lookup_table = vec![vec![i32::MAX; self.vertices + 1]; 2];

        // Distance from source to itself is 0.
        lookup_table[0][src] = 0;

        // Track shortest path change when subproblem size is increased.
        let mut is_shortest_path_changed = false;

        // Increase number of edges and recompute shortest paths.
        for i in 1..=self.vertices {
            is_shortest_path_changed = false;
            for vertex in 1..=self.vertices {
                // Case 1: Length of src-vertex is already the shortest path.
                let mut min_value = lookup_table[(i - 1) % 2][vertex];

                // Check in-degree of vertex to find a less cost path.
                for edge in self.rev_adj_list[vertex].iter() {
                    match lookup_table[(i - 1) % 2][edge.head] {
                        i32::MAX => {}
                        val => {
                            if val + edge.cost < min_value {
                                min_value = val + edge.cost;
                                is_shortest_path_changed = true;
                            }
                        }
                    }
                }

                // Update the new shortest path for src-vertex.
                lookup_table[i % 2][vertex] = min_value;
            }

            // Shortest path is not changed for next iteration. We found the solution.
            if !is_shortest_path_changed {
                break;
            }
        }

        // Found the shortest path at i = n iterations. There is a cycle.
        if is_shortest_path_changed {
            return None;
        }

        // Return the shortest path array.
        return Some(lookup_table[self.vertices % 2].clone());
    }

    // Run Dijkstra from src vertex. Return Single source shortest paths from this vertex.
    fn run_dijkstra(&self, src: usize) -> Vec<i32> {
        // Consider all vertices to be apart.
        let mut shortest_dist = vec![i32::MAX; self.vertices + 1];
        let mut explored = vec![false; self.vertices + 1];

        let mut min_heap = BinaryHeap::<HeapNode>::new();

        // Distance from source to itself is 0.
        min_heap.push(HeapNode {
            vertex: src,
            distance: 0,
        });

        while let Some(min_node) = min_heap.pop() {
            // Save its distance if its not already explored.
            if !explored[min_node.vertex] {
                explored[min_node.vertex] = true;
                shortest_dist[min_node.vertex] = min_node.distance;

                // Recalculate shortest distance of its neighbouring vertices.
                for edge in self.adj_list[min_node.vertex].iter() {
                    if !explored[edge.head] {
                        min_heap.push(HeapNode {
                            vertex: edge.head,
                            distance: min_node.distance + edge.cost,
                        });
                    }
                }
            }
        }

        return shortest_dist;
    }
}

// Heap node representation for Dijkstra single source shortest path algorithm.

fn main() -> Result<(), Box<dyn Error>> {
    // Sanity check.
    if env::args().len() != 2 {
        eprintln!("usage: ./johnson_algorithm <file_name>");
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
        let src = splitted.next().unwrap().parse::<usize>()?;
        let dest = splitted.next().unwrap().parse::<usize>()?;
        let cost = splitted.next().unwrap().parse::<i32>()?;
        graph.add_edge(src, dest, cost);
    }

    println!("{:?}", graph.compute_all_pair_shortest_paths());

    Ok(())
}
