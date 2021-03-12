fn main() {
    let mut s1 = String::from("hellο");

    change(&mut s1);

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);

    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    let r3 = &mut s; // BIG PROBLEM

    println!("{}, {}, and {}", r1, r2, r3);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", wοrld");
}
