/**
 * Build huffman tree using heaps.
 * Compute the maximum and minimum length of the codeword in resulting tree.
 */
use std::{collections::BinaryHeap, collections::VecDeque, fs, io::Error};

/// Tree node in Huffman tree.
#[derive(Debug, Eq, PartialEq)]
struct TreeNode {
    weight: u64,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

impl TreeNode {
    // Build a new tree node with no children.
    fn new(weight: u64) -> TreeNode {
        TreeNode {
            weight,
            left: None,
            right: None,
        }
    }
}

// Behave as min heap.
impl Ord for TreeNode {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.weight.cmp(&other.weight)
    }
}
impl PartialOrd for TreeNode {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(other.cmp(&self))
    }
}

// Returns the root of the Huffman Tree.
fn build_tree(weights: &[u64]) -> TreeNode {
    // Build a min heap from weights. We will use it to build tree using bottom-up approach.
    let mut heap = BinaryHeap::<TreeNode>::new();
    for weight in weights {
        heap.push(TreeNode::new(*weight));
    }

    while heap.len() > 1 {
        // Merge the top 2 smallest weights into one.
        let left = heap.pop().unwrap();
        let right = heap.pop().unwrap();
        heap.push(TreeNode {
            weight: left.weight + right.weight,
            left: Some(Box::new(left)),
            right: Some(Box::new(right)),
        });
    }

    // Last remaining node is the root of huffman tree.
    return heap.pop().unwrap();
}

// Get the minimum and maximum coding length of codeword.
fn get_min_max_encoding_length(root: &TreeNode) -> (usize, usize) {
    // Prepare for level order traversal of huffman tree.
    let mut queue = VecDeque::<&TreeNode>::new();

    // Maintain min and max levels of tree.
    let mut min_level: usize = 0;
    let mut max_level: usize = 0;

    // Store nodes per level, so that we know in next iteration how many nodes to process.
    queue.push_back(root);
    let mut nodes_per_level: usize = 1;

    while !queue.is_empty() {
        let mut temp = nodes_per_level;
        nodes_per_level = 0;

        // Loop over nodes at this level.
        while temp > 0 {
            let top = queue.pop_front().unwrap();
            temp -= 1;

            match (&top.left, &top.right) {
                (Some(left), Some(right)) => {
                    queue.push_back(left.as_ref());
                    queue.push_back(right.as_ref());
                    nodes_per_level += 2;
                }
                (None, None) => {
                    if min_level == 0 {
                        min_level = max_level;
                    }
                }
                (_, _) => {}
            }
        }

        max_level += 1;
    }

    (min_level, max_level - 1)
}

fn main() -> Result<(), Error> {
    let weights = fs::read_to_string("huffman_input.txt")?
        .lines()
        .map(|v| v.parse::<u64>().expect("Failed to parse"))
        .collect::<Vec<u64>>();

    let huffman_tree = build_tree(&weights[1..]);

    println!("min, max: {:?}", get_min_max_encoding_length(&huffman_tree));

    Ok(())
}
