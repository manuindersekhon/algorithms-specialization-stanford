/// Kruskal's minimum spanning tree.
/// Uses union find data structure to check for cycles in constant time.
use std::fs;
use std::io::Error;

mod union_find;

// Edge representation.
#[derive(Debug)]
struct Edge {
    head: i32, // Head vertex
    tail: i32, // Tail vertex
    cost: i32, // Cost of this edge.
}

fn main() -> Result<(), Error> {
    let mut union_find = union_find::UnionFind::new();

    // Read input from file.
    let file_contents = fs::read_to_string("input_edges.txt")?
        .lines()
        .map(|v| v.to_string())
        .collect::<Vec<String>>();

    // Get vertices from file.
    let vertices = file_contents[0]
        .split_whitespace()
        .next()
        .expect("Failed to get vertices")
        .parse::<i32>()
        .expect("Failed to parse vertices");

    // Load edges into array.
    let mut edges = Vec::<Edge>::new();
    for line in file_contents.iter().skip(1) {
        let parsed = line
            .split_whitespace()
            .map(|v| v.parse::<i32>().expect("Failed to parse"))
            .collect::<Vec<i32>>();

        edges.push(Edge {
            head: parsed[0],
            tail: parsed[1],
            cost: parsed[2],
        });
    }

    // Load union find with all vertices.
    for i in 1..=vertices {
        union_find.add(i);
    }

    // Sort edges by edge cost.
    edges.sort_unstable_by(|a, b| a.cost.cmp(&b.cost));

    // Run Kruskal on graph.
    let mut minimum_cost = 0;
    for edge in edges.iter() {
        // This edge connects different disjoint sets.
        if union_find.find(edge.head) != union_find.find(edge.tail) {
            minimum_cost += edge.cost;
            union_find.union(edge.head, edge.tail);
        }
    }

    println!("Minimum cost = {}", minimum_cost);

    Ok(())
}
