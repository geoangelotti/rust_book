fn main() {
    let n: u64 = 20;
    println!("{} Fibonacci is {}", n, slow_fibonacci(n));
}

fn slow_fibonacci(n: u64) -> u64 {
    if n == 1 {
        return 1;
    } else if n <= 0 {
        return 0;
    }
    slow_fibonacci(n - 2) + slow_fibonacci(n - 1)
}
