/// Prim's Minimum Spanning Tree.
/// Find the overall cost of the minimum spanning tree. Uses the heap based implementation.

import 'dart:collection';
import 'dart:io';

import 'package:collection/collection.dart';
import 'package:equatable/equatable.dart';

/// Adjacency list representation of a graph.
class Graph {
  final int vertices;
  final List<List<Edge>> adjList;

  static const int INT_MAX = 0x7fffffffffff;

  Graph(this.vertices) : adjList = List.generate(vertices, (_) => <Edge>[]);

  /// Creates an undirected edge between v1 and v2.
  void addEdge(int v1, int v2, int weight) {
    this.adjList[v1].add(Edge(v2, weight));
    this.adjList[v2].add(Edge(v1, weight));
  }

  /// Returns the cost of minimum spanning tree.
  int mstCost() {
    // Mark add vertices as unexplored.
    List<bool> explored = List.filled(this.vertices, false);
    // Min heap.
    final heap = HeapPriorityQueue<HeapNode>((p0, p1) => p0.weight.compareTo(p1.weight));

    // Start with vertex 0, all other vertices will be infinite distance away from vertex 0.
    heap.add(HeapNode(0, 0));
    for (int i = 1; i < this.vertices; i++) {
      heap.add(HeapNode(i, INT_MAX));
    }

    int minimumCost = 0;
    while (heap.isNotEmpty) {
      HeapNode top = heap.removeFirst();

      // This is the closest vertex to already explored MST.
      if (!explored[top.vertex]) {
        explored[top.vertex] = true;
        minimumCost += top.weight;

        // Explore the neighbouring vertices.
        for (final edge in this.adjList[top.vertex]) {
          if (!explored[edge.tailVertex]) {
            heap.add(HeapNode(edge.tailVertex, edge.weight));
          }
        }
      }
    }

    return minimumCost;
  }

  /// O(mn) implementation for computing MST.
  int mstCostBruteForce() {
    final hashset = HashSet<int>()..add(0);
    int minimumCost = 0;

    int i = 0;
    while (i < this.vertices) {
      Edge? cheapestEdge;
      for (int head = 0; head < this.vertices; head++) {
        if (hashset.contains(head)) {
          for (final tail in adjList[head]) {
            if (!hashset.contains(tail.tailVertex)) {
              if (cheapestEdge == null) cheapestEdge = tail;
              if (tail.weight < cheapestEdge.weight) cheapestEdge = tail;
            }
          }
        }
      }

      if (cheapestEdge != null) {
        hashset.add(cheapestEdge.tailVertex);
        minimumCost += cheapestEdge.weight;
      }

      i++;
    }

    return minimumCost;
  }
}

/// Weighted edge representation.
class Edge {
  final int tailVertex;
  final int weight;

  const Edge(this.tailVertex, this.weight);
}

/// Heap node prioritised by minimum weight.
class HeapNode extends Equatable {
  final int vertex;
  final int weight;

  const HeapNode(this.vertex, this.weight);

  @override
  List<Object> get props => [vertex];
}

void main(List<String> arguments) {
  final fileContents = File('assets/input_edges.txt').readAsLinesSync();

  // First line contains number of vertices.
  int vertices = int.parse(fileContents.first.split(' ').first);

  final graph = Graph(vertices);

  // Parse the file and load into graph.
  for (final line in fileContents.sublist(1)) {
    final parsed = line.split(' ').map(int.parse).toList();
    graph.addEdge(parsed[0] - 1, parsed[1] - 1, parsed[2]);
  }

  print('Minimum cost = ${graph.mstCost()}');
  print('Minimum cost (brute force) = ${graph.mstCostBruteForce()}');
}
