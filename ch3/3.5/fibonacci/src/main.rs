fn main() {
    let n: u128 = 20;
    println!("{} Fibonacci is {}", n, slow_fibonacci(n));
}

fn slow_fibonacci(n: u128) -> u128 {
    if n == 1 {
        return 1;
    } else if n <= 0 {
        return 0;
    }
    slow_fibonacci(n - 2) + slow_fibonacci(n - 1)
}
