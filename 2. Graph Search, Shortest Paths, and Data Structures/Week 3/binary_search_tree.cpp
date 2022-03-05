#include <iostream>
#include <queue>
#include <list>

using namespace std;

// Node of the tree.
typedef struct node {
    int key;
    string value;
    struct node *left;
    struct node *right;

    // Size of the subtree rooted at this node.
    int size;
} node;

// Class representing the Binary Search Tree.
class BST {
private:
    // Root of the tree.
    node *root;

    // Size of the subtree rooted at this node.
    int size(node *tree);

    // Delete the minimum node from tree and return its parent node.
    node *delete_min(node *tree);

    // Helper methods for recursion.
    node *min(node *tree);
    node *max(node *tree);
    node *insertRecursively(node *node, int key, string value);
    node *delete_node(node *tree_node, int key);
    int rank(node *tree_node, int key);
    int select(node *tree_node, int ith_order);

public:
    // Insert a key, value pair in the tree.
    void insert(int key, string value);

    // Returns the list of level order traversal for this tree.
    list<int> level_order();

    // Return size of the tree.
    int size();

    // Helper function to print level order traverval of tree.
    void print_level_order();

    // Return the minimum key in the tree, -1 if not found.
    int min();

    // Return the maximum key in the tree, -1 if not found.
    int max();

    // Return the floor of the given key, -1 if not found.
    // Floor -> Largest element smaller than or equal to arg_key.
    int floor(int arg_key);

    // Return the ceil of the given key, -1 if not found.
    // Ceil -> Smallest element greater than or equal to arg_key.
    int ceil(int arg_key);

    // Delete a node in a tree.
    void delete_node(int key);

    // Returns rank of the key, or -1 of key not found. Rank of k is equal to number of keys less than k.
    int rank(int key);

    // Returns the ith order statistic value, or -1 of value not found.
    int select(int ith_order);
};

void BST::insert(int key, string value) {
    this->root = BST::insertRecursively(this->root, key, value);
}

int BST::size(node *tree) {
    return tree == NULL ? 0 : tree->size;
}

int BST::size() {
    return this->size(this->root);
}

void BST::delete_node(int key) {
    this->root = BST::delete_node(this->root, key);
}

int BST::rank(int key) {
    return this->rank(this->root, key);
}

int BST::select(int ith_order) {
    return this->select(this->root, ith_order);
}

int BST::select(node *tree_node, int ith_order) {
    // Ith order statistic not found.
    if (tree_node == NULL) return -1;

    // Check the current order statistic of current node.
    int current_order = 1 + this->size(tree_node->left);

    if (ith_order == current_order) return tree_node->key;
    else if (ith_order < current_order) return select(tree_node->left, ith_order);
    else return select(tree_node->right, ith_order - current_order);
}

int BST::rank(node *tree_node, int key) {
    // We reached here, that means key not found.
    if (tree_node == NULL) return -1;

    if (key == tree_node->key) {
        return this->size(tree_node->left);
    }
    else if (key < tree_node->key) {
        return rank(tree_node->left, key);
    }
    else {
        return this->size(tree_node->left) + 1 + rank(tree_node->right, key);
    }
}

node *BST::delete_node(node *tree_node, int key) {
    // No tree exists, or desired key is not found.
    if (tree_node == NULL) return NULL;

    // Reach the desired node.
    if (key < tree_node->key) {
        tree_node->left = delete_node(tree_node->left, key);
    }
    else if (key > tree_node->key) {
        tree_node->right = delete_node(tree_node->right, key);
    }
    else {
        // We found the node to delete.
        node *t = tree_node;

        // If left child does not exists, return the right child as it will also satisfy the BST property.
        // Similarly in case of the right child.
        if (t->left == NULL) return t->right;
        if (t->right == NULL) return t->left;

        // Find the successor of this node (minimum node in its right subtree).
        node *s = this->min(t->right);

        // Create copy of above node as we will delete it next.
        node *successor = (node *)malloc(sizeof(node));
        successor->key = s->key;
        successor->left = s->left;
        successor->right = s->right;

        // Delete the minimum node from right subtree.
        t->right = this->delete_min(t->right);

        // Place successor at the desired position.
        successor->left = t->left;
        successor->right = t->right;

        free(t);
        tree_node = successor;
    }

    tree_node->size = 1 + this->size(tree_node->left) + this->size(tree_node->right);
    return tree_node;
}

node *BST::delete_min(node *tree_node) {
    if (tree_node == NULL) return NULL;
    if (tree_node->left == NULL) return tree_node->right;

    tree_node->left = delete_min(tree_node->left);
    return tree_node;
}

// Just works like an event loop.
list<int> BST::level_order() {
    if (this->root == NULL) return list<int>();

    list<int> result;

    queue<node *> queue;
    queue.push(this->root);

    while (!queue.empty()) {
        // Get the topmost element from queue.
        node *top = queue.front();
        queue.pop();

        if (top == NULL) continue;

        result.push_back(top->key);

        // Mark next level of node to be traversed.
        queue.push(top->left);
        queue.push(top->right);
    }

    return result;
}

void BST::print_level_order() {
    list<int> result = this->level_order();
    for (int &item : result) {
        cout << item << " ";
    }
    cout << "\n";
}

node *BST::insertRecursively(node *tree_node, int key, string value) {
    // Insert the new node at tree rooted at this node.
    if (tree_node == NULL) {
        node *new_node = (node *)malloc(sizeof(node));
        new_node->key = key;
        new_node->value = value;
        new_node->size = 1;
        new_node->left = new_node->right = NULL;
        return new_node;
    }

    if (key < tree_node->key) {
        node *new_node = insertRecursively(tree_node->left, key, value);
        tree_node->left = new_node;
    }
    else if (key > tree_node->key) {
        node *new_node = insertRecursively(tree_node->right, key, value);
        tree_node->right = new_node;
    }
    else {
        // If key is already present in the tree, then replace it with new value.
        tree_node->value = value;
    }

    // Increase the size of the tree rooted at tree_node by 1.
    tree_node->size = 1 + this->size(tree_node->left) + this->size(tree_node->right);

    return tree_node;
}

int BST::min() {
    node *tree_node = this->min(this->root);
    return tree_node == NULL ? -1 : tree_node->key;
}

int BST::max() {
    node *tree_node = this->max(this->root);
    return tree_node == NULL ? -1 : tree_node->key;
}

node *BST::min(node *tree) {
    if (tree == NULL) return NULL;
    if (tree->left == NULL) return tree;

    return min(tree->left);
}

node *BST::max(node *tree) {
    if (tree == NULL) return NULL;
    if (tree->right == NULL) return tree;

    return max(tree->right);
}

int BST::floor(int arg_key) {
    node *tree_node = this->root;
    int floor = -1;

    // Keep going until there are no more valid nodes to check for floor.
    while (tree_node != NULL) {
        // Exact match found.
        if (tree_node->key == arg_key) {
            floor = tree_node->key;
            break;
        }
        // Current key is greater, hence can't be the floor. We have to search left for smaller nodes.
        else if (tree_node->key > arg_key) {
            tree_node = tree_node->left;
        }
        // Current key is smaller, this can be our floor. But also look to the right subtree,
        // as there may be a greater value than this.
        else if (tree_node->key < arg_key) {
            floor = tree_node->key;
            tree_node = tree_node->right;
        }
    }

    return floor;
}

int BST::ceil(int arg_key) {
    node *tree_node = this->root;
    int ceil = -1;

    while (tree_node != NULL) {
        if (tree_node->key == arg_key) {
            ceil = tree_node->key;
            break;
        }
        else if (tree_node->key < arg_key) {
            tree_node = tree_node->right;
        }
        else if (tree_node->key > arg_key) {
            ceil = tree_node->key;
            tree_node = tree_node->left;
        }
    }

    return ceil;
}

int main(void) {
    BST tree = BST();
    cout << "Size of tree: " << tree.size() << "\n";

    tree.insert(5, "");
    tree.insert(7, "");
    tree.insert(6, "");
    tree.insert(3, "");
    tree.insert(8, "");
    tree.insert(4, "");
    tree.insert(2, "");

    cout << "Size of tree: " << tree.size() << "\n";
    tree.print_level_order();

    cout << "Min: " << tree.min() << "\n";
    cout << "Max: " << tree.max() << "\n";

    cout << "floor(1): " << tree.floor(1) << ", ceil(1): " << tree.ceil(1) << "\n";
    cout << "floor(7): " << tree.floor(7) << ", ceil(4): " << tree.ceil(4) << "\n";
    cout << "floor(9): " << tree.floor(9) << ", ceil(9): " << tree.ceil(9) << "\n";

    cout << "Rank(6): " << tree.rank(6) << "\n";
    cout << "Rank(1): " << tree.rank(1) << "\n";
    cout << "Rank(2): " << tree.rank(2) << "\n";
    cout << "Rank(3): " << tree.rank(3) << "\n";

    cout << "Select(0): " << tree.select(0) << "\n";
    cout << "Select(1): " << tree.select(1) << "\n";
    cout << "Select(2): " << tree.select(2) << "\n";
    cout << "Select(3): " << tree.select(3) << "\n";
    cout << "Select(4): " << tree.select(4) << "\n";
    cout << "Select(5): " << tree.select(5) << "\n";
    cout << "Select(6): " << tree.select(6) << "\n";
    cout << "Select(7): " << tree.select(7) << "\n";
    cout << "Select(8): " << tree.select(8) << "\n";

    tree.delete_node(2);
    cout << "Deleting 2... \n";
    tree.print_level_order();

    tree.delete_node(5);
    cout << "Deleting 5... \n";
    tree.print_level_order();
}