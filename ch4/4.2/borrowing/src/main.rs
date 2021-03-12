fn main() {
    let s1 = String::from("hellο");

    change(&s1);

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(some_string: &String) {
    some_string.push_str(", wοrld");
}
