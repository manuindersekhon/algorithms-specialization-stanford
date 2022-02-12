/**
 * Counting inversions.
 * Inversion occurs when i < j, and A[i] > A[j].
*/

#include <iostream>

typedef unsigned long long _bigint;

// Helper functions to recursively sort and count inversions.
_bigint sort_and_count(int *array, int *temp, size_t start, size_t end);
_bigint count_split_inversions(int *array, int *temp, size_t start_one, size_t end_one, size_t start_two, size_t end_two);

_bigint inversion_count(int *array, size_t length) {
    // Additional space needed to help merge sorted halves.
    int temp[length];
    return sort_and_count(array, temp, 0, length - 1);
}

_bigint sort_and_count(int *array, int *temp, size_t start, size_t end) {
    if (start >= end) return 0;

    // Find the middle element
    size_t middle = (start + end) / 2;

    // Count left inversions
    _bigint left_inversions = sort_and_count(array, temp, start, middle);

    // Count right inversions.
    _bigint right_inversions = sort_and_count(array, temp, middle + 1, end);

    // Count split inversions.
    _bigint split_inversions = count_split_inversions(array, temp, start, middle, middle + 1, end);

    return left_inversions + right_inversions + split_inversions;
}

_bigint count_split_inversions(int *array, int *temp, size_t start_one, size_t end_one, size_t start_two, size_t end_two) {
    int index1 = start_one, index2 = start_two, temp_index = start_one;

    _bigint inversion_count = 0;

    // Copy smallest elements first from both sorted arrays.
    while (index1 <= end_one && index2 <= end_two) {
        if (array[index1] < array[index2])
            temp[temp_index++] = array[index1++];
        else {
            temp[temp_index++] = array[index2++];
            inversion_count += (end_one - index1 + 1);
        }
    }

    // Copy remaining elements.
    while (index1 <= end_one) temp[temp_index++] = array[index1++];
    while (index2 <= end_two) temp[temp_index++] = array[index2++];

    // Temp array has the merged sorted values, copy them back into original array.
    for (int index = start_one; index <= end_two; index++)
        array[index] = temp[index];

    return inversion_count;
}

int main(void) {
    const int LENGTH = 100000;

    int *array = (int *)calloc(LENGTH, sizeof(int));
    for (int i = 0; i < LENGTH; i++) {
        std::cin >> array[i];
    }

    std::cout << inversion_count(array, LENGTH) << "\n";
}