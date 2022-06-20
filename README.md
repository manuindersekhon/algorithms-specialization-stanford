# Stanford's Algorithms Specialization

This repository contains the solutions for problems sets and algorithms taught in [Stanford's Algorithms Specialization](https://www.coursera.org/specializations/algorithms) at Coursera.

Most of the problems sets are written in `Rust` (as a part of learning Rust itself) and some of them are written in `C++` and `Dart`.

### Topics and Design paradigms covered are:

1. [Divide and Conquer, Sorting and Searching, and Randomized Algorithms](https://github.com/ManuSekhon/algorithms-specialization-stanford/tree/main/1.%20Divide%20and%20Conquer%2C%20Sorting%20and%20Searching%2C%20and%20Randomized%20Algorithms)
1. [Graph Search, Shortest Paths, and Data Structures](https://github.com/ManuSekhon/algorithms-specialization-stanford/tree/main/2.%20Graph%20Search%2C%20Shortest%20Paths%2C%20and%20Data%20Structures)
1. [Greedy Algorithms, Minimum Spanning Trees, and Dynamic Programming](https://github.com/ManuSekhon/algorithms-specialization-stanford/tree/main/3.%20Greedy%20Algorithms%2C%20Minimum%20Spanning%20Trees%2C%20and%20Dynamic%20Programming)
1. [Shortest Paths Revisited, NP Complete Prolems and What to Do about It](https://github.com/ManuSekhon/algorithms-specialization-stanford/tree/main/4.%20Shortest%20Paths%20Revisited%2C%20NP%20Complete%20Prolems%20and%20What%20to%20Do%20about%20It)

Specific details are covered in each subfolder.

### Running the files

Problem sets are tested on below versions at the time of writing.

Rust: `rustc 1.60.0 (7737e0b5c 2022-04-04)`

Dart: `2.17.3 (stable)`

C++: `Apple clang version 13.1.6 (clang-1316.0.21.2.5)`

Files can be compiled and run by following the below steps.

```bash
# Rust
$ rustc -C debuginfo=0 -C opt-level=3 filename.rs
$ ./filename

# Dart single file.
$ dart --enable-asserts filename.dart

# Dart project
$ dart run

# C++
$ clang++ -Wall -Werror -Wno-unused-parameter -std=c++17 filename.cpp -o filename
$ ./filename
```
