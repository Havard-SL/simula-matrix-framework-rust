pub fn factorial(n: usize) -> usize {
    let mut result = 1;

    for i in 1..=n {
        result *= i
    }

    result
}