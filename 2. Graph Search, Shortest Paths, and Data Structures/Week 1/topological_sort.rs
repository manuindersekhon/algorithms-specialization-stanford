/**
 * Compute Topological ordering in directed acyclic graph using Depth First Search.
 * Time Complexity: O(m + n)
 */
use std::collections::HashMap;

// Graph representation in adjacency list.
#[derive(Debug)]
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

    // Returns vertices in topological order.
    fn topological_order(&self) -> HashMap<usize, usize> {
        let mut explored = vec![false; self.vertices];
        let mut f_values: HashMap<usize, usize> = HashMap::new();

        // Count F-values from top-down
        let mut current_f_value = self.vertices;

        // Run DFS on all vertices.
        for vertex in 0..self.vertices {
            if !explored[vertex] {
                self.depth_first_search(vertex, &mut explored, &mut f_values, &mut current_f_value);
            }
        }

        return f_values;
    }

    // Run DFS from this vertex
    fn depth_first_search(
        &self,
        vertex: usize,
        explored: &mut [bool],
        f_values: &mut HashMap<usize, usize>,
        current_f_value: &mut usize,
    ) {
        // Mark this vertex as explored.
        explored[vertex] = true;

        // Explore all neighbouring vertices of this vertex.
        for v in self.adj_list.get(vertex).unwrap() {
            if !explored[*v] {
                self.depth_first_search(*v, explored, f_values, current_f_value);
            }
        }

        // We reach the last vertex in this DFS iteration, save F-value for this.
        f_values.insert(vertex, *current_f_value);
        *current_f_value = *current_f_value - 1;
    }
}

fn main() {
    let mut graph = Graph::new(5);
    graph.add_edge(3, 1);
    graph.add_edge(3, 4);
    graph.add_edge(4, 0);
    graph.add_edge(1, 0);
    graph.add_edge(0, 2);

    let mut graph2 = Graph::new(6);
    graph2.add_edge(5, 2);
    graph2.add_edge(5, 0);
    graph2.add_edge(4, 0);
    graph2.add_edge(4, 1);
    graph2.add_edge(2, 3);
    graph2.add_edge(3, 1);

    println!("{:?}", graph.topological_order());
    println!("{:?}", graph2.topological_order());
}
