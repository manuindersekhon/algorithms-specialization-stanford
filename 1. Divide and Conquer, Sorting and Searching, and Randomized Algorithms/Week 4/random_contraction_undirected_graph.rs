/**
 * Karger's Random Contraction Algorithm.
 * Compute minimum number of cuts (crossing edges) in an undirected graph.
 *
 * Logic: Keep removing random edges until two vertices are left.
 * We may not get the correct results first time, hence run it multiple times to compute the lowest value.
 */
use std::collections::HashMap;
use std::io::Error;
use std::{env, fs};

fn compute_min_cut(graph: &mut HashMap<i32, Vec<i32>>) -> usize {
    // Base case: Can't divide graph further.
    if graph.len() == 2 {
        let key = graph.keys().next().unwrap();
        return graph.get(key).unwrap().len();
    }

    // Get a random vertex from graph.
    let keys = graph.keys().map(|val| *val).collect::<Vec<i32>>();
    let vertex_i = keys[random() % keys.len()];

    // Get random vertex on the other end of edge.
    let edges = graph.get(&vertex_i).unwrap().clone();
    let vertex_j = edges[random() % edges.len()];

    // Prepare to remove vertex_i, point all edges connected with vertex_i to vertex_j.
    for vertex in edges {
        // Remove edge between vertex_i and this vertex.
        let index = graph
            .get(&vertex)
            .unwrap()
            .iter()
            .position(|&item| item == vertex_i)
            .unwrap();
        graph.get_mut(&vertex).unwrap().remove(index);

        // Create an edge between this vertex to vertex_j. Check added to prevent self loops.
        if vertex != vertex_j {
            graph.get_mut(&vertex).unwrap().push(vertex_j);
            graph.get_mut(&vertex_j).unwrap().push(vertex);
        }
    }

    // Remove vertex_i.
    graph.remove(&vertex_i);

    // Compute min cut for remaining graph.
    return compute_min_cut(graph);
}

fn main() -> Result<(), Error> {
    // Ensure proper usage.
    let args = env::args().skip(1).collect::<Vec<String>>();
    if args.len() != 1 {
        eprintln!("usage: <filename>");
        std::process::exit(1);
    }

    // Try to open the file
    let filename = &args[0];
    let contents = fs::read_to_string(filename)?;

    // Create an adjacency list.
    let mut adjacency_list: HashMap<i32, Vec<i32>> = HashMap::new();

    // Loop over to contents of file to build our graph.
    for row in contents.split_terminator(['\n']) {
        let row_line = row
            .split_whitespace()
            .map(|item| item.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

        // First number is the vertex, rest are the edges of that vertex.
        adjacency_list.insert(row_line[0], row_line[1..].to_vec());
    }

    let max_iterations = 500;

    let mut min_value = 10000000;
    for index in 0..max_iterations {
        let min_cuts = compute_min_cut(&mut adjacency_list.clone());
        println!("Iteration {} = {} min cuts", index, min_cuts);

        if min_cuts < min_value {
            min_value = min_cuts;
        }
    }

    println!("\nMin Cuts = {}", min_value);

    Ok(())
}

fn random() -> usize {
    unsafe {
        srand(time(0));
        rand()
    }
}

extern "C" {
    fn time(value: u32) -> u32;
    fn srand(seed: u32);
    fn rand() -> usize;
}
