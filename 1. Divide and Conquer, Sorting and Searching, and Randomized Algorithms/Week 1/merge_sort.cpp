/**
 * Merge sort
 *
 * Divide array into two halves.
 * Sort left half. Sort right half. Merge both halves.
**/

#include <iostream>

// Merge sort
void merge_sort(int *array, size_t length);

// Helper functions to recursively sort and merge.
void sort(int *array, int *temp, size_t start, size_t end);
void merge(int *array, int *temp, size_t start_one, size_t end_one, size_t start_two, size_t end_two);

void merge_sort(int *array, size_t length) {
    // Additional space needed to help merge sorted halves.
    int temp[length];
    sort(array, temp, 0, length - 1);
}

void sort(int *array, int *temp, size_t start, size_t end) {
    if (start >= end) return;

    // Find the middle element
    size_t middle = (start + end) / 2;

    // Sort left half
    sort(array, temp, start, middle);

    // Sort right half
    sort(array, temp, middle + 1, end);

    // Merge both halves.
    merge(array, temp, start, middle, middle + 1, end);
}

void merge(int *array, int *temp, size_t start_one, size_t end_one, size_t start_two, size_t end_two) {
    int index1 = start_one, index2 = start_two, temp_index = start_one;

    // Copy smallest elements first from both sorted arrays.
    while (index1 <= end_one && index2 <= end_two) {
        if (array[index1] < array[index2])
            temp[temp_index++] = array[index1++];
        else
            temp[temp_index++] = array[index2++];
    }

    // Copy remaining elements.
    while (index1 <= end_one) temp[temp_index++] = array[index1++];
    while (index2 <= end_two) temp[temp_index++] = array[index2++];

    // Temp array has the merged sorted values, copy them back into original array.
    for (int index = start_one; index <= end_two; index++)
        array[index] = temp[index];
}

int main(void) {
    const int LENGTH = 10;

    int array[LENGTH] = {10, 9, 8, 8, 3, 0, 4, 3, 2, 1};
    merge_sort(array, LENGTH);

    for (int index = 0; index < LENGTH; index++) {
        std::cout << array[index] << " ";
    }

    std::cout << "\n";
}