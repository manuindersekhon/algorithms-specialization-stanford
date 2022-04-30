/**
 * Compute maximum spacing of 4-clustering for given dataset.
 */
use std::fs;
use std::io::Error;

mod union_find;

// Edge representation.
#[derive(Debug)]
struct Edge {
    head: i32,
    tail: i32,
    edge_cost: i32,
}

impl Edge {
    // Create a new edge.
    fn new(head: i32, tail: i32, edge_cost: i32) -> Edge {
        Edge { head, tail, edge_cost }
    }
}

fn max_spacing_k_clustering(no_of_nodes: i32, edges: &[Edge]) -> i32 {
    const TARGET_CLUSTERS: i32 = 4;

    // Keep every node as its own cluster.
    let mut current_clusters: i32 = no_of_nodes;

    // Load every cluster in union find as disjoint set.
    let mut union_find = union_find::UnionFind::new();
    for node in 1..=no_of_nodes {
        union_find.add(node);
    }

    let mut iterator = edges.iter().peekable();

    // Keep merging clusters until we hit the target.
    while current_clusters != TARGET_CLUSTERS && iterator.peek().is_some() {
        if let Some(edge) = iterator.next() {
            // This edge connects different clusters, merge both clusters.
            if union_find.find(edge.head) != union_find.find(edge.tail) {
                union_find.union(edge.head, edge.tail);
                current_clusters -= 1;
            }
        }
    }

    // Find the next cluster to be merged. This will be the max spacing.
    let mut max_spacing = -1;
    while iterator.peek().is_some() {
        if let Some(edge) = iterator.next() {
            if union_find.find(edge.head) != union_find.find(edge.tail) {
                max_spacing = edge.edge_cost;
                break;
            }
        }
    }

    return max_spacing;
}

fn main() -> Result<(), Error> {
    let file_contents = fs::read_to_string("clustering_small_input.txt")?
        .lines()
        .map(|v| v.to_string())
        .collect::<Vec<String>>();

    // Get total number of nodes from file.
    let no_of_nodes = file_contents
        .first()
        .expect("Failed to get nodes")
        .parse::<i32>()
        .expect("Failed to parse nodes");

    // Load edges into array and sort by edge cost.
    let mut edges = Vec::<Edge>::new();
    for line in file_contents.iter().skip(1) {
        let parsed = line
            .split_whitespace()
            .map(|v| v.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        edges.push(Edge::new(parsed[0], parsed[1], parsed[2]));
    }
    edges.sort_unstable_by(|a, b| a.edge_cost.cmp(&b.edge_cost));

    let max_spacing = max_spacing_k_clustering(no_of_nodes, &edges);
    println!("Max spacing of 4-clustering = {}", max_spacing);

    Ok(())
}
