/**
 * Breadth first search.
 * Compute shortest path in an undirected graph in O(m + n) time.
 */
use std::collections::VecDeque;

#[derive(Debug)]
// Graph representation in adjacency list.
struct Graph {
    // Number of vertices.
    vertices: usize,
    // Adjacency list from vertex (0..n).
    adj_list: Vec<Vec<usize>>,
}

impl Graph {
    // Create a new graph with [vertices].
    fn new(vertices: usize) -> Graph {
        Graph {
            vertices,
            adj_list: vec![Vec::<usize>::new(); vertices],
        }
    }

    // Creates an edge from v1 to v2.
    fn add_edge(&mut self, v1: usize, v2: usize) {
        self.adj_list[v1].push(v2);
    }

    // Returns shortest path to all vertices starting from [start_vertex].
    fn shortest_path(&self, start_vertex: usize) -> Vec<usize> {
        // Maintain distance of every vertex.
        let mut dist = vec![0; self.vertices];
        dist[start_vertex] = 0;

        // Mark start_vertex as explored.
        let mut explored = vec![false; self.vertices];
        explored[start_vertex] = true;

        let mut queue = VecDeque::new();
        queue.push_back(start_vertex);

        // Start exploring, starting from start_vertex.
        while !queue.is_empty() {
            // Get the topmost element from queue.
            if let Some(top_vertex) = queue.pop_front() {
                // Explore all neighbouring vertices from this vertex.
                for v in self.adj_list.get(top_vertex).unwrap() {
                    // This is not yet explored
                    if !explored[*v] {
                        explored[*v] = true;
                        dist[*v] = dist[top_vertex] + 1;

                        // Mark this vertex explore its neighbours.
                        queue.push_back(*v);
                    }
                }
            }
        }

        return dist;
    }
}

fn main() {
    let mut graph = Graph::new(6);

    graph.add_edge(0, 1);
    graph.add_edge(1, 0);
    graph.add_edge(1, 2);
    graph.add_edge(2, 1);
    graph.add_edge(2, 3);
    graph.add_edge(3, 2);
    graph.add_edge(3, 5);
    graph.add_edge(5, 3);
    graph.add_edge(5, 4);
    graph.add_edge(4, 5);
    graph.add_edge(4, 0);
    graph.add_edge(0, 4);
    graph.add_edge(2, 4);
    graph.add_edge(4, 2);
    graph.add_edge(2, 5);
    graph.add_edge(5, 2);

    println!("ShortestPath(0): {:?}", graph.shortest_path(0));
    println!("ShortestPath(2): {:?}", graph.shortest_path(2));
}
