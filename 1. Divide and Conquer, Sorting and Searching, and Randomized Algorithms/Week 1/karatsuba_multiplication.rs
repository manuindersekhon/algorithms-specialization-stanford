/**
 * Karatsuba Multiplication.
 *
 * For two numbers x and y,
 * x.y = (10^(n / 2).a + b) . (10^(n / 2).c + d)
**/
use std::cmp::min;

// Multiplies two numbers and return the result.
fn multiply(num1: u128, num2: u128) -> u128 {
    // Get the minimum length of the two numbers.
    let length = min(num1.to_string().len(), num2.to_string().len()) as u32;

    // Multiply it directly if any of the two numbers is single digit.
    if length == 1 {
        return num1 * num2;
    }

    // Split number into two parts.
    let denominator = (10 as i32).pow(length / 2) as u128;
    let a = num1 / denominator;
    let b = num1 % denominator;
    let c = num2 / denominator;
    let d = num2 % denominator;

    // Recursively compute ac, bd and (a + b).(c + b).
    let ac = multiply(a, c);
    let bd = multiply(b, d);

    // This is done so that we don't have to recursively compute ad + bc separately (Gauss trick)
    let aplusb_cplusd = multiply(a + b, c + d);
    let ad_plus_bc = aplusb_cplusd - ac - bd;

    // Apply results into Karatsuba'a equation.
    return 10_u128.pow(length) * ac + 10_u128.pow(length / 2) * ad_plus_bc + bd;
}

fn main() {
    let num1 = 12345678;
    let num2 = 87654321;
    let result = multiply(num1, num2);

    println!("{} * {} = {}", num1, num2, result);
}
