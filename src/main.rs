fn main() {
    let num1 = 1610003; // First number
    let num2 = 713883; // Second number
    let gcd_result = euclidean_algorithm(num1, num2);
    
    println!("The GCD of {} and {} is {}", num1, num2, gcd_result);
}

fn euclidean_algorithm(mut a: i64, mut b: i64) -> i64 {
    while b != 0 {
        let temp = b;//The Euclidean Algorithm can also be used with negative values.// If you just need to work with non-negative integers, use u64.// However, using i64 increases the function's versatility, as it can handle both positive and negative inputs.
        b = a % b;
        a = temp;
    }
    a
}