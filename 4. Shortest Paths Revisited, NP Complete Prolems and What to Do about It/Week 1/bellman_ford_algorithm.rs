/**
 * The Bellman Ford Algorithm to compute single source shortest path for generic edge lengths.
 * Intuition: Subset of the shortest path b/w s-v is itself the shortest path.
 * Idea: Artificially restrict the number of edges in the subproblem.
 */
use std::cmp;

/// Graph representation
#[derive(Debug, Clone)]
struct Graph {
    // Total number of vertices from 0..=n-1
    vertices: usize,
    // Adjacency list representation of Graph.
    adj_list: Vec<Vec<Edge>>,
    // Reversed graph. Created to calculate the in-degree of vertices.
    rev_adj_list: Vec<Vec<Edge>>,
    // Lookup table built to calculate shortest single source paths.
    lookup_table: Vec<Vec<LookupValue>>,
    // True if cycle exists.
    cycle_exists: bool,
}

/// Edge representation.
#[derive(Debug, Clone)]
struct Edge {
    // Head vertex of this edge.
    head: i32,
    // Cost (or edge length) of this edge.
    cost: i32,
}

/// Value of shortest-path lookup table.
#[derive(Debug, Clone)]
struct LookupValue {
    // Total cost of the shortest path.
    cost: i32,
    // Remember the 2nd-to-last vertex when this cost was changed. Default is None.
    vertex: Option<i32>,
}

impl LookupValue {
    // Create a new empty lookup value.
    fn identity() -> LookupValue {
        LookupValue {
            cost: i32::MAX,
            vertex: None,
        }
    }
}

impl Graph {
    // Initialize empty graph with n vertices.
    fn new(vertices: usize) -> Graph {
        Graph {
            vertices,
            adj_list: vec![Vec::<Edge>::new(); vertices],
            rev_adj_list: vec![Vec::<Edge>::new(); vertices],
            lookup_table: vec![vec![LookupValue::identity(); vertices]; 2],
            cycle_exists: false,
        }
    }

    // Add a directed edge in this graph.
    fn add_edge(&mut self, src: i32, dest: i32, cost: i32) {
        self.adj_list[src as usize].push(Edge { head: dest, cost });
        self.rev_adj_list[dest as usize].push(Edge { head: src, cost });
    }

    // Build the shortest path from single vertex to all vertices.
    fn build_lookup_table(&mut self, src: i32) {
        // Distance from source to itself is zero when using 0 edges. For others it infinite be default.
        self.lookup_table[0][src as usize] = LookupValue { cost: 0, vertex: None };

        // Track if shortest path of any vertex changes when subproblem is increased.
        let mut is_shortest_path_changed = false;

        // Increase the number of subproblems, i.e, add more edges one at a time.
        for i in 1..self.vertices {
            is_shortest_path_changed = false;

            // Recompute the shortest path when new edge is added.
            for vertex in 0..self.vertices {
                // Case 1: Length of src-vertex is already the shortest path.
                let mut min_value = self.lookup_table[(i - 1) % 2][vertex].clone();

                // Case 2: Loop over in-degree of this vertex to check if shortest path changes when
                // one more edge is added.
                for edge in self.rev_adj_list[vertex].iter() {
                    match self.lookup_table[(i - 1) % 2][edge.head as usize].cost {
                        i32::MAX => {}
                        val => {
                            // Get the minimum of all in-degree candidates.
                            if val + edge.cost < min_value.cost {
                                min_value = LookupValue {
                                    cost: val + edge.cost,
                                    vertex: Some(edge.head),
                                };
                                is_shortest_path_changed = true;
                            }
                        }
                    }
                }

                // Update the new shortest path.
                self.lookup_table[i % 2][vertex] = min_value;
            }

            // Shortest path is not changed after adding the new edge, we found the answer.
            if !is_shortest_path_changed {
                break;
            }
        }

        // We found a shortest path at i = n iterations, means there is a cycle.
        if is_shortest_path_changed {
            self.cycle_exists = true;
        }
    }

    /// Get actual shortest path.
    fn get_shortest_paths(&self) -> Vec<Vec<i32>> {
        let mut shortest_paths = vec![Vec::<i32>::new(); self.vertices];
        let index = (self.vertices - 1) % 2;

        for vertex in 0..self.vertices {
            let mut last_hop_vertex = self.lookup_table[index][vertex].vertex;
            while last_hop_vertex != None {
                shortest_paths[vertex].push(last_hop_vertex.unwrap());
                last_hop_vertex = self.lookup_table[index][last_hop_vertex.unwrap() as usize].vertex;
            }

            shortest_paths[vertex].reverse();
            shortest_paths[vertex].push(vertex as i32);
        }

        return shortest_paths;
    }
}

fn main() {
    let mut graph = Graph::new(5);
    graph.add_edge(0, 1, -1);
    graph.add_edge(0, 2, 4);
    graph.add_edge(1, 2, 3);
    graph.add_edge(1, 3, 2);
    graph.add_edge(1, 4, 2);
    graph.add_edge(3, 1, 1);
    graph.add_edge(3, 2, 5);
    graph.add_edge(4, 3, -3);

    graph.build_lookup_table(0);
    println!("Graph 1");
    println!("Cycle exists? {}", graph.cycle_exists);
    let shortest_paths = graph.get_shortest_paths();
    for item in shortest_paths.iter().enumerate() {
        println!("0->{}:  {:?}", item.0, item.1);
    }

    let mut graph2 = Graph::new(8);
    graph2.add_edge(0, 1, 4);
    graph2.add_edge(0, 2, 4);
    graph2.add_edge(2, 4, 4);
    graph2.add_edge(2, 5, -2);
    graph2.add_edge(3, 0, 3);
    graph2.add_edge(3, 2, 2);
    graph2.add_edge(4, 3, 1);
    graph2.add_edge(4, 6, -2);
    graph2.add_edge(5, 1, 3);
    graph2.add_edge(5, 4, -3);
    graph2.add_edge(6, 5, 2);
    graph2.add_edge(6, 7, 2);
    graph2.add_edge(7, 4, -2);

    graph2.build_lookup_table(0);
    println!("Graph 2");
    println!("Cycle exists? {}", graph2.cycle_exists);
}
