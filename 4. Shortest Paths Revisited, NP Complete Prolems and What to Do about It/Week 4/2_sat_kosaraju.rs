/**
 * 2-SAT problem reduced to Strongly Connected Components and solved using Kosaraju's Two-Pass algorithm.
 * Idea: Every clause in 2-SAT can be represented as a graph with 2 vertices and 2 directed edges.
 * Time complexity: O(m + n), where m are vertices and n is number of edges.
 */
use std::collections::LinkedList;
use std::{error::Error, fs};

/// 2-SAT
#[derive(Debug)]
struct TwoSat {
    /// Total number of variables.
    num_variables: usize,
    /// Directed graph representation for 2-SAT dataset.
    graph: Graph,
}

impl TwoSat {
    /// Open the file and try to load it into directed graph.
    fn create_graph_from_file(filename: &str) -> Result<TwoSat, Box<dyn Error>> {
        let file_contents = fs::read_to_string(filename)?;
        let mut file_iter = file_contents.lines();

        // Total number of variables and clauses in 2-SAT problem. (1 indexed)
        let num_variables = file_iter.next().unwrap().parse::<usize>()?;
        let mut graph = Graph::new(num_variables);

        // Convert clauses into directed edges of graph.
        for line in file_iter {
            let mut row = line.split_whitespace();
            let mut x = row.next().unwrap().parse::<i32>()?;
            let mut y = row.next().unwrap().parse::<i32>()?;

            // Convert clause (x, y) into vertices of graph.
            // x OR y === (~x OR y) AND (x OR ~y). Hence there's a directed edge between ~x -> y, and ~y -> x.
            let mut x_not: i32 = 0;
            let mut y_not: i32 = 0;

            // +ve variable will map to vertices 1..=n, and -ve variables will map to n + 1..=2n.
            if x > 0 {
                x_not = x + num_variables as i32;
            } else {
                x_not = x.abs();
                x = x.abs() + num_variables as i32;
            }
            if y > 0 {
                y_not = y + num_variables as i32;
            } else {
                y_not = y.abs();
                y = y.abs() + num_variables as i32;
            }

            graph.add_edge(x_not as usize, y as usize);
            graph.add_edge(y_not as usize, x as usize);
        }

        return Ok(TwoSat { num_variables, graph });
    }

    /// True, it this satisfies 2-SAT property.
    fn is_satisfiable(&self) -> bool {
        // Get strongly connected components of this graph.
        let scc = self.graph.strongly_connected_components();

        // For satisfibility, x and ~x should not be in same SCC.
        for index in 1..=self.num_variables {
            if scc[index] == scc[index + self.num_variables] {
                return false;
            }
        }

        return true;
    }
}

/// Graph representation using adjacency list.
#[derive(Debug)]
struct Graph {
    // Total number of vertices in range 1..2n + 1, 0th vertex is void.
    vertices: usize,
    // Adjacency list from vertex 1..2n + 1.
    adj_list: Vec<LinkedList<usize>>,
    // Reversed adjacency list of same graph (used to compute finishing times)
    rev_adj_list: Vec<LinkedList<usize>>,
}

impl Graph {
    /// Return an empty graph.
    fn new(n: usize) -> Graph {
        let vertices = 2 * n + 1;

        Graph {
            vertices,
            adj_list: vec![LinkedList::new(); vertices],
            rev_adj_list: vec![LinkedList::new(); vertices],
        }
    }

    /// Creates a directed edge src and dest.
    fn add_edge(&mut self, src: usize, dest: usize) {
        // Edge src -> dest in forward graph.
        self.adj_list[src].push_back(dest);

        // Edge dest -> src in reverse graph.
        self.rev_adj_list[dest].push_back(src);
    }

    /// Run Depth first search subroutine on graph and fill vector in topological order.
    fn depth_first_search(
        graph: &Vec<LinkedList<usize>>,
        start_vertex: usize,
        explored: &mut [bool],
        topological_order: &mut Vec<usize>,
    ) {
        // Stack to perform DFS.
        let mut stack = Vec::<usize>::new();

        // Mark this vertex as explored.
        explored[start_vertex] = true;
        stack.push(start_vertex);

        'dfs_loop: while !stack.is_empty() {
            // Get the top vertex from stack.
            if let Some(&top) = stack.last() {
                // Iterate over all the neighbours of this vertex, and check if we can dig deeper.
                for neighbour_v in graph[top].iter() {
                    // Go to unvisited neighbour.
                    if !explored[*neighbour_v] {
                        explored[*neighbour_v] = true;

                        // Try to dig deeper from this neighbour.
                        stack.push(*neighbour_v);
                        continue 'dfs_loop;
                    }
                }

                // Can't dig deeper. This is the farthest vertex in topological order.
                topological_order.push(top);

                // Backtrack.
                stack.pop();
            }
        }
    }

    /// Compute Strongly Connected Components, and return SCC leaders of vertices.
    fn strongly_connected_components(&self) -> Vec<usize> {
        // Store vertices as increasing order of their finishing times.
        let mut finish_times = Vec::<usize>::with_capacity(self.vertices);

        // Mark all vertices as explored.
        let mut explored = vec![false; self.vertices];

        // Run DFS on reversed graph to compute finish times.
        for v in 1..self.vertices {
            if !explored[v] {
                Graph::depth_first_search(&self.rev_adj_list, v, &mut explored, &mut finish_times);
            }
        }

        let mut explored = vec![false; self.vertices];
        let mut ssc_list = vec![0_usize; self.vertices];

        // Run DFS on forward graph in topological order to compute SCCs.
        for v in finish_times.iter().rev() {
            // Found a new SCC.
            if !explored[*v] {
                let mut finish_times = Vec::new();
                Graph::depth_first_search(&self.adj_list, *v, &mut explored, &mut finish_times);

                // All vertices explored by above DFS will have their leader as [v] in the SCC.
                for w in finish_times.iter() {
                    ssc_list[*w] = *v;
                }
            }
        }

        return ssc_list;
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    // Dataset
    let filenames = [
        "2sat_dataset/2sat1.txt",
        "2sat_dataset/2sat2.txt",
        "2sat_dataset/2sat3.txt",
        "2sat_dataset/2sat4.txt",
        "2sat_dataset/2sat5.txt",
        "2sat_dataset/2sat6.txt",
    ];

    // Loop over dataset and check if it satisfies 2-SAT.
    for filename in filenames.iter() {
        // Load file into directed graph.
        let two_sat_instance = TwoSat::create_graph_from_file(filename)?;

        // Check if it satifies 2-SAT clauses.
        let result = two_sat_instance.is_satisfiable();

        println!("{} -> {}", filename, result);
    }

    Ok(())
}
