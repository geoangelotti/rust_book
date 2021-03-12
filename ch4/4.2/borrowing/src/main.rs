fn main() {
    let mut s1 = String::from("hellο");

    change(&mut s1);

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);

    let mut s = String::from("hello");

    {
        let r1 = &mut s;
        r1.push_str(" friεnd");
        println!("{}", r1);
    } // r1 goes out of scope here, so we can make a new reference with no problems.

    let r2 = &mut s;

    println!("{}", r2);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", wοrld");
}
