/**
 * Karatsuba Multiplication.
 *
 * For two numbers x and y,
 * x.y = (10^(n / 2).a + b) . (10^(n / 2).c + d)
**/

#include <iostream>
#include <math.h>

// For ease of use
typedef unsigned long long _bigint;

// Returns the length of an integer (multiplied with bool to handle 0 condition).
size_t len(_bigint num) {
    return ((bool)num * (int)log10(num) + 1);
}

// Multiplies two numbers and return the result.
unsigned long long multiply(_bigint num1, _bigint num2) {
    // Get the minimum length of the two numbers.
    int length = std::min(len(num1), len(num2));

    // Multiply it directly if any of the two numbers is single digit.
    if (length == 1) return num1 * num2;

    // Split number into two parts.
    _bigint denominator = pow(10, length / 2);
    _bigint a = num1 / denominator,
        b = num1 % denominator,
        c = num2 / denominator,
        d = num2 % denominator;

    // Recursively compute ac, bd and (a + b).(c + b).
    _bigint ac = multiply(a, c);
    _bigint bd = multiply(b, d);

    // This is done so that we don't have to recursively compute ad + bc separately (Gauss trick)
    _bigint aplusb_cplusd = multiply(a + b, c + d);
    _bigint ad_plus_bc = aplusb_cplusd - ac - bd;

    // Apply results into Karatsuba'a equation.
    return pow(10, length) * ac + pow(10, length / 2) * ad_plus_bc + bd;
}

int main(void) {
    _bigint num1 = 1234, num2 = 5678;
    std::cout << multiply(num1, num2) << "\n";
}