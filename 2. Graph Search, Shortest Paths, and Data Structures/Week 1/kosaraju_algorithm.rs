/**
 * Kosaraju's Two-Pass algorithm.
 * Compute strongly connected components in directed graph in O(m + n) time.
 */
use std::collections::LinkedList;
use std::fs;
use std::io::Error;

#[derive(Debug)]
struct Graph {
    // Number of vertices.
    vertices: usize,
    // Adjacency list from vertex (0..n).
    adj_list: Vec<LinkedList<usize>>,
    // Reversed adjacency list of same graph (used to compute finishing times)
    rev_adj_list: Vec<LinkedList<usize>>,
}

impl Graph {
    // Create a new graph with [vertices].
    fn new(vertices: usize) -> Graph {
        Graph {
            vertices,
            adj_list: vec![LinkedList::<usize>::new(); vertices],
            rev_adj_list: vec![LinkedList::<usize>::new(); vertices],
        }
    }

    // Creates an edge from v1 to v2, and reverse edge from v2 to v1
    fn add_edge(&mut self, v1: usize, v2: usize) {
        self.adj_list[v1].push_back(v2);
        self.rev_adj_list[v2].push_back(v1);
    }

    // Compute Strongly Connected Components, and returns size of all SCCs.
    fn strongly_connected_components(&self) -> Vec<(usize, usize)> {
        // Store vertices as increasing order of finish times.
        let mut finish_times: Vec<usize> = Vec::with_capacity(self.vertices);

        // Mark all vertices as unexplored.
        let mut explored = vec![false; self.vertices];

        // Run DFS on reversed graph to compute finish times.
        for v in 0..self.vertices {
            if !explored[v] {
                self.dfs_reversed_graph(v, &mut explored, &mut finish_times);
            }
        }

        // Mark all vertices as unexplored again.
        let mut explored = vec![false; self.vertices];

        // Maintain list of SCC's (leader, its size).
        let mut scc_list = Vec::<(usize, usize)>::new();

        // Run DFS on forward graph with decreasing order of finish times.
        for v in finish_times.iter().rev() {
            if !explored[*v] {
                let mut size_of_this_scc: usize = 0;
                self.dfs_forward_graph(*v, &mut size_of_this_scc, &mut explored);
                scc_list.push((*v, size_of_this_scc));
            }
        }

        return scc_list;
    }

    // DFS subroutine on forward graph to compute SCC.
    fn dfs_forward_graph(&self, vertex: usize, size: &mut usize, explored: &mut [bool]) {
        // Stack to perform DFS.
        let mut stack = Vec::<usize>::new();

        // Mark this vertex as explored.
        explored[vertex] = true;
        stack.push(vertex);
        *size = *size + 1;

        'dfs_loop: while !stack.is_empty() {
            // let mut next_vertex_found = false;
            if let Some(&top) = stack.last() {
                for v in &self.adj_list[top] {
                    // Mark next neighbour to explore next.
                    if !explored[*v] {
                        explored[*v] = true;
                        stack.push(*v);
                        *size = *size + 1;
                        continue 'dfs_loop;
                    }
                }

                // Can't dig deeper.
                stack.pop();
            }
        }
    }

    // DFS subroutine on the reversed graph to compute finishing times.
    fn dfs_reversed_graph(
        &self,
        vertex: usize,
        explored: &mut [bool],
        finish_times: &mut Vec<usize>,
    ) {
        // Stack to perform DFS.
        let mut stack = Vec::<usize>::new();

        // Mark this vertex as explored.
        explored[vertex] = true;
        stack.push(vertex);

        'dfs_loop: while !stack.is_empty() {
            // Get the top vertex in stack.
            if let Some(&top) = stack.last() {
                for v in &self.rev_adj_list[top] {
                    // Mark next neighbour to explore next.
                    if !explored[*v] {
                        explored[*v] = true;
                        stack.push(*v);
                        continue 'dfs_loop;
                    }
                }

                // Can't dig deeper.
                finish_times.push(top);
                stack.pop();
            }
        }
    }
}

fn main() -> Result<(), Error> {
    let filename = "scc_input_graph.txt";
    let no_of_vertices: usize = 875714;

    let mut graph = Graph::new(no_of_vertices);

    // Try to read the graph from file.
    let file_content = fs::read_to_string(filename)?;
    for line in file_content.lines() {
        let vertices = line
            .split_whitespace()
            .map(|v| v.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();

        graph.add_edge(vertices[0] - 1, vertices[1] - 1);
    }

    let mut result = graph.strongly_connected_components();

    // Sort descending by SCC size.
    result.sort_unstable_by(|a, b| b.1.cmp(&a.1));

    println!("{:?}", &result[..5]);

    Ok(())
}
