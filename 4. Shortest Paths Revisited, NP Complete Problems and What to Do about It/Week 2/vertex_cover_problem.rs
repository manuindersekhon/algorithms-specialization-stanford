/**
 * Given graph G = (V, E) and integer K, find the minimum cardinality vertex cover of size K if exists.
 * Explanation: Find subset S that includes at least one endpoint of each edge in G, meaning, vertices in S
 * should be able to directly reach all other vertices in G.
 * 
 ** Same graph can have multiple vertex covers.
 ** If vertex cover exists for size K, it will also exist for size K + 1.
 */
use std::collections::{HashMap, HashSet};

/// Adjacency list representation of undirected graph.
#[derive(Debug, Clone)]
struct Graph {
    graph: HashMap<i32, Vec<i32>>,
}

impl Graph {
    /// Create a new graph.
    fn new() -> Graph {
        Graph { graph: HashMap::new() }
    }

    /// Add undirected edge.
    fn add_edge(&mut self, src: i32, dest: i32) {
        // edge: src -> dest
        if let Some(list) = self.graph.get_mut(&src) {
            list.push(dest);
        } else {
            self.graph.insert(src, Vec::new());
            self.graph.get_mut(&src).unwrap().push(dest);
        }

        // edge: dest -> src
        if let Some(list) = self.graph.get_mut(&dest) {
            list.push(src);
        } else {
            self.graph.insert(dest, Vec::new());
            self.graph.get_mut(&dest).unwrap().push(src);
        }
    }

    /// Delete vertex from graph.
    fn delete_vertex(&mut self, vertex: i32) {
        // Delete all the edges that are pointing in-to this vertex.
        if self.graph.contains_key(&vertex) {
            for v in self.graph.get(&vertex).unwrap().clone().iter() {
                if self.graph.contains_key(&v) {
                    if let Some(index) = self.graph.get(v).unwrap().iter().position(|x| *x == vertex) {
                        self.graph.get_mut(v).unwrap().remove(index);
                        if self.graph.get(v).unwrap().is_empty() {
                            self.graph.remove(v);
                        }
                    }
                }
            }

            // Delete this vertex from graph.
            self.graph.remove(&vertex);
        }
    }

    /// Returns true, if the graph is empty.
    fn is_empty(&self) -> bool {
        self.graph.is_empty()
    }

    /// Returns the pair of vertices connected by a direct edge.
    fn get_uv_pair(&self) -> (i32, i32) {
        let u = self.graph.iter().next().unwrap();
        return (*u.0, *u.1.first().unwrap());
    }
}

/// Returns true, and fills subset S with the result if vertex cover exists. False otherwise.
fn get_vertex_cover(graph: &Graph, k: usize, subset: &mut HashSet<i32>) -> bool {
    // We found a valid vertex cover.
    if k == 0 && graph.is_empty() {
        return true;
    } else if k == 0 && !graph.is_empty() {
        return false;
    }

    let (u, v) = graph.get_uv_pair();

    // Check for vertex cover by deleting u and v from graph.
    let mut graph_u = graph.clone();
    graph_u.delete_vertex(u);
    let result_u = get_vertex_cover(&graph_u, k - 1, subset);
    if result_u {
        subset.insert(u);
        return true;
    }

    let mut graph_v = graph.clone();
    graph_v.delete_vertex(v);
    let result_v = get_vertex_cover(&graph_v, k - 1, subset);
    if result_v {
        subset.insert(v);
        return true;
    }

    result_u || result_v
}

fn main() {
    let mut graph = Graph::new();
    graph.add_edge(1, 6);
    graph.add_edge(1, 7);
    graph.add_edge(2, 7);
    graph.add_edge(2, 3);
    graph.add_edge(3, 6);
    graph.add_edge(3, 8);
    graph.add_edge(3, 9);
    graph.add_edge(4, 7);
    graph.add_edge(4, 10);
    graph.add_edge(5, 10);
    graph.add_edge(5, 7);
    graph.add_edge(6, 7);
    graph.add_edge(7, 8);
    graph.add_edge(9, 10);

    let mut subset = HashSet::<i32>::new();
    let k: usize = 4;
    let result = get_vertex_cover(&graph, k, &mut subset);
    println!("Vertex cover of size {} = {}, {:?}", k, result, subset);
}
