/**
 * Computational Geometry: Closest pair in 2D plane.
 */

import 'dart:math';

// Class representing a Point in 2D plane.
class Point {
  final int x;
  final int y;

  Point(this.x, this.y);

  @override
  String toString() => 'Point($x, $y)';
}

// Class that maintains a pair of points.
class Pair {
  final Point a;
  final Point b;

  Pair(this.a, this.b);

  // Returns the euclidean distance between a and b.
  double get distance {
    return sqrt(pow(a.x - b.x, 2) + pow(a.y - b.y, 2));
  }

  @override
  String toString() => 'Pair($a, $b)';
}

// Returns the closest pair in the dataset. Points_x and Points_y are copies of same dataset sorted by x and y coordinates.
// Divide and conquer runs on points_x only. Points_y is only used for computing split pair subroutine.
Pair closestPair(List<Point> pointsX, List<Point> pointsY) {
  // Fallback to brute force method for datasets that contain 2 or 3 elements (1 element can't make a pair).
  if (pointsX.length <= 3) {
    return bruteForce(pointsX);
  }

  // Split X array into two halves.
  int middle = pointsX.length ~/ 2;
  List<Point> leftHalf = pointsX.sublist(0, middle);
  List<Point> rightHalf = pointsX.sublist(middle);

  // Find closest pair in left half of dataset.
  Pair p1q1 = closestPair(leftHalf, pointsY);

  // Find closest pair in right half of dataset.
  Pair p2q2 = closestPair(rightHalf, pointsY);

  // Find minimum euclidean distance between above two pairs.
  double delta = min(p1q1.distance, p2q2.distance);

  // If the closest pair is made up of one point from left and other from right, find that pair.
  Pair? p3q3 = closestSplitPair(pointsX, pointsY, delta);

  // We found such split pair that is smaller than the other top two.
  if (p3q3 != null) return p3q3;

  // Return the best of left or right pairs.
  return delta == p1q1.distance ? p1q1 : p2q2;
}

// Check if there exists a pair p,q such that one lies in left half and other in right half.
Pair? closestSplitPair(List<Point> pointsX, List<Point> pointsY, double delta) {
  // Find median point from sorted x array.
  Point middlePoint = pointsX[pointsX.length ~/ 2];

  // Get the subset such that x lies b/w [middle_x - delta, middle_x + delta], sorted by y coordinates.
  List<Point> subsetY = pointsY.where((p) => p.x > middlePoint.x - delta && p.x < middlePoint.x + delta).toList();

  double best = delta;
  Pair? minPair;

  for (int i = 0; i < subsetY.length - 1; i++) {
    // Try to find the closest pair from i to at most its 7 next positions,
    // because pair has to be b/w (2-Delta x Delta) region.
    for (int j = 1; j < min(7, subsetY.length - i - 1); j++) {
      double distance = Pair(subsetY[i], subsetY[i + j]).distance;
      if (distance < best) {
        best = distance;
        minPair = Pair(subsetY[i], subsetY[i + j]);
      }
    }
  }

  return minPair;
}

// Returns the closest pair using O(n^2) brute force approach.
Pair bruteForce(List<Point> points) {
  // Maintain minimum distance and closest pair found.
  double? minDistance;
  late Pair minPair;

  for (int i = 0; i < points.length; i++) {
    for (int j = 0; j < points.length; j++) {
      if (i != j) {
        double distance = Pair(points[i], points[j]).distance;
        if (minDistance == null || distance < minDistance) {
          // We found the new closest pair.
          minDistance = distance;
          minPair = Pair(points[i], points[j]);
        }
      }
    }
  }

  return minPair;
}

void main() {
  const LENGTH = 100;

  List<Point> points = [];
  for (int i = 0; i < LENGTH; i++) {
    points.add(Point(Random().nextInt(1000), Random().nextInt(1000)));
  }

  List<Point> pointsX = [...points]..sort((one, two) => one.x.compareTo(two.x));
  List<Point> pointsY = [...points]..sort((one, two) => one.y.compareTo(two.y));

  Pair pair1 = closestPair(pointsX, pointsY);
  Pair pair2 = bruteForce(points);

  print(pair1);
  print(pair2);
}
