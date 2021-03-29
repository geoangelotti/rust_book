fn main() {
    let s = String::from("hello wοrld");

    let hello = &s[..5];
    let world = &s[6..];
    println!("{} {}", hello, world);
}
