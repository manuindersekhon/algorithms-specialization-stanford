/**
 * Dijkstra's shortest path algorithm.
 * Compute shortest path with non-negative edge lengths.
 */
use std::{collections::BinaryHeap, io::Error};

const DEFAULT_SHORTEST_DIST: usize = 1000000;

/// Graph representation by adjacency list.
#[derive(Debug)]
struct Graph {
    vertices: usize,
    adj_list: Vec<Vec<Edge>>,
}

/// Represent a weighted edge.
#[derive(Clone, Debug)]
struct Edge {
    // Vertex on tail end of this edge.
    tail: usize,
    // Weight of this vertex.
    weight: usize,
}

/// Heap node to store vertex and its shortest known distance.
#[derive(PartialEq, Eq)]
struct HeapNode {
    distance: usize,
    vertex: usize,
}

// Make it behave as a Min-Heap.
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
    fn new(vertices: usize) -> Graph {
        Graph {
            vertices,
            adj_list: vec![Vec::<Edge>::new(); vertices],
        }
    }

    // Add all edges originating from this vertex.
    fn add_edges(&mut self, vertex: usize, edges: Vec<Edge>) {
        self.adj_list[vertex] = edges;
    }

    // Compute shortest paths from [source] vertex to all other vertices.
    fn shortest_paths(&self, source: usize) -> Vec<usize> {
        // Consider all vertices to be apart.
        let mut shortest_dist = vec![DEFAULT_SHORTEST_DIST; self.vertices];
        let mut explored = vec![false; self.vertices];

        let mut min_heap = BinaryHeap::new();

        // Distance from source to itself in zero.
        shortest_dist[source] = 0;
        min_heap.push(HeapNode {
            vertex: source,
            distance: 0,
        });

        // Keep extracting closest vertex from source.
        while let Some(min_node) = min_heap.pop() {
            // Save its distance if its not already explored.
            if !explored[min_node.vertex] {
                explored[min_node.vertex] = true;
                shortest_dist[min_node.vertex] = min_node.distance;

                // Calculate shortest distance of its neighbouring vertices.
                for edge in &self.adj_list[min_node.vertex] {
                    if !explored[edge.tail] {
                        min_heap.push(HeapNode {
                            vertex: edge.tail,
                            distance: min_node.distance + edge.weight,
                        });
                    }
                }
            }
        }

        return shortest_dist;
    }
}

fn main() -> Result<(), Error> {
    let filename = "dijkstraData.txt";
    let no_of_vertices: usize = 200;
    let mut graph = Graph::new(no_of_vertices);

    // Try to read from file.
    let file_contents = std::fs::read_to_string(filename)?;
    for line in file_contents.lines() {
        let mut vertex: usize = 0;
        let mut edges = Vec::<Edge>::new();

        // Loop over contents from each row.
        for (index, item) in line.split_whitespace().enumerate() {
            // vertex number
            if index == 0 {
                vertex = item.parse::<usize>().unwrap() - 1;
            } else {
                // Weighted edge.
                let edge = item
                    .split(',')
                    .map(|s| s.parse::<usize>().unwrap())
                    .collect::<Vec<usize>>();
                edges.push(Edge {
                    tail: edge[0] - 1,
                    weight: edge[1],
                });
            }
        }

        graph.add_edges(vertex, edges);
    }

    let source_vertex: usize = 1;

    let shortest_paths = graph.shortest_paths(source_vertex - 1);

    for index in [7, 37, 59, 82, 99, 115, 133, 165, 188, 197] {
        println!("distance({},{}) = {}", source_vertex, index, shortest_paths[index - 1]);
    }

    Ok(())
}
