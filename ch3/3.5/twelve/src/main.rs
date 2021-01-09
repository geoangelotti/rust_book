fn main() {
    let days = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eight", "ninth",
        "tenth", "eleventh", "twelfth",
    ];
    let offerings = [
        "a Partridge in a Pear Tree",
        "Two Turtle Doves",
        "Three French Hens",
        "Four Calling Birds",
        "Five Golden Rings",
        "Six Geese a Laying",
        "Seven Swans a Swimming",
        "Eight Maids a Milking",
        "Nine Ladies Dancing",
        "Ten Lords a Leaping",
        "Eleven Pipers Piping",
        "12 Drummers Drumming",
    ];
    for i in 0..days.len() {
        println!();
        println!("On the {} day of Christmas", days[i]);
        println!("my true love sent to me:");
        for n in (0..i + 1).rev() {
            if n == 0 {
                if n == i {
                    println!("{}", offerings[n])
                } else {
                    println!("and {}", offerings[n])
                }
            } else {
                println!("{}", offerings[n])
            }
        }
    }
}
