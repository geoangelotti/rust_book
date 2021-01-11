fn main() {
    let n: u128 = 20;
    println!("{} Fibonacci is {}", n, slow_fibonacci(n));
    println!("{} Fibonacci is {}", n, faster_fibonacci(n));
}

fn slow_fibonacci(n: u128) -> u128 {
    if n == 1 {
        return 1;
    } else if n <= 0 {
        return 0;
    }
    slow_fibonacci(n - 2) + slow_fibonacci(n - 1)
}

fn faster_fibonacci(n: u128) -> u128 {
    let mut a: u128 = 0;
    let mut b: u128 = 1;
    for _ in 0..(n) {
        let c: u128 = a + b;
        a = b;
        b = c;
    }
    return a;
}
