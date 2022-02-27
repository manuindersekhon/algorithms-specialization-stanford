/**
 * Create our own 0-index based Binary Min-Heap (also called priority queue).
 * Supported operations: Insert, Delete, Extract top value, all in O(log n) time.
 */

// Heap implemented using vector.
#[derive(Debug)]
struct Heap {
    array: Vec<i32>,
}

impl Heap {
    // Initialize an empty heap.
    fn new() -> Heap {
        Heap { array: Vec::new() }
    }

    // Inserts an element in a heap.
    fn insert(&mut self, value: i32) {
        // Push element to end of array.
        self.array.push(value);

        // Bubble up this value to satisfy heap constraint.
        self.bubble_up(self.array.len() - 1);
    }

    // Extracts the top element from the heap.
    fn extract(&mut self) -> Option<i32> {
        if self.array.is_empty() {
            return None;
        }

        let top_value = *self.array.first().unwrap();

        // Bring the last element in place of first.
        let last = self.array.len() - 1;
        self.array.swap(last, 0);
        self.array.pop();

        // Heap is empty now.
        if self.array.is_empty() {
            return Some(top_value);
        }

        // Bubble down the top element to correct position.
        self.bubble_down(0);

        Some(top_value)
    }

    // Deletes and returns this value from the heap.
    fn delete(&mut self, value: i32) -> Option<i32> {
        // Find the index of the element to delete.
        let mut index: Option<usize> = None;
        for (i, e) in self.array.iter().enumerate() {
            if *e == value {
                index = Some(i);
                break;
            }
        }

        // Element not found.
        if index == None {
            return None;
        }

        // Swap this with last element.
        let last = self.array.len() - 1;
        self.array.swap(index.unwrap(), last);
        let deleted_element = self.array.pop();

        // Heap is empty now.
        if self.array.is_empty() {
            return deleted_element;
        }

        // Bubble down the top element to correct position.
        self.bubble_down(index.unwrap());

        deleted_element
    }

    // Bubble up the element at [index] until it satisfies heap property.
    fn bubble_up(&mut self, index: usize) {
        let mut child_index = index;
        let mut parent_index = self.get_parent_index(child_index);

        // Keep bubbling up.
        while parent_index != None && self.array[parent_index.unwrap()] > self.array[child_index] {
            self.array.swap(parent_index.unwrap(), child_index);
            child_index = parent_index.unwrap();
            parent_index = self.get_parent_index(child_index);
        }
    }

    // Bubble down the element at [index] until it satisfies heap property.
    fn bubble_down(&mut self, index: usize) {
        let mut parent_index = index;
        let mut child_index = self.get_smallest_child_index(parent_index);

        while child_index != None && self.array[parent_index] > self.array[child_index.unwrap()] {
            self.array.swap(parent_index, child_index.unwrap());
            parent_index = child_index.unwrap();
            child_index = self.get_smallest_child_index(parent_index);
        }
    }

    // Given the child index, returns the parent index or None.
    fn get_parent_index(&self, child_index: usize) -> Option<usize> {
        match child_index {
            0 => None,
            index => Some((index - 1) / 2),
        }
    }

    // Given the parent index, returns the index of smallest child or none.
    fn get_smallest_child_index(&self, parent_index: usize) -> Option<usize> {
        let first_child = match 2 * parent_index + 1 < self.array.len() {
            true => Some(2 * parent_index + 1),
            false => None,
        };

        // Socond child won't exist if first does not.
        if first_child == None {
            return None;
        }

        let second_child = match 2 * parent_index + 2 < self.array.len() {
            true => Some(2 * parent_index + 2),
            false => None,
        };

        if second_child == None {
            return first_child;
        }

        // Return the index of smaller of two children.
        if self.array[first_child.unwrap()] < self.array[second_child.unwrap()] {
            first_child
        } else {
            second_child
        }
    }
}

fn main() {
    let mut min_heap = Heap::new();

    for item in [10, 9, 8, 7, 3, 1] {
        min_heap.insert(item);
    }

    println!("{:?}", min_heap);

    min_heap.extract();
    println!("{:?}", min_heap);

    min_heap.delete(3);
    println!("{:?}", min_heap);
}
