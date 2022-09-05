// Generate the nth Fibonacci number.
fn fibonacci(number: u32) -> u64 {
    let mut fibonacci_number: u64 = 0;
    let mut prev: u64 = 1;

    let mut index = 1; 
    while index < number {
        let next : u64 = fibonacci_number + prev;
        prev = fibonacci_number;
        fibonacci_number = next;

        index += 1;
    }

    fibonacci_number
}


#[cfg(test)]
mod fibonacci_tests {

    use super::*;

    #[test]
    fn nth_fibonacci_correct() {
        assert_eq!(fibonacci(1), 0);
        assert_eq!(fibonacci(2), 1);
        assert_eq!(fibonacci(3), 1);
        assert_eq!(fibonacci(4), 2);
        assert_eq!(fibonacci(5), 3);
        assert_eq!(fibonacci(6), 5);
        assert_eq!(fibonacci(8), 13);
        assert_eq!(fibonacci(10), 34);
        assert_eq!(fibonacci(20), 4181);
        assert_eq!(fibonacci(50), 7778742049);
    }
}


