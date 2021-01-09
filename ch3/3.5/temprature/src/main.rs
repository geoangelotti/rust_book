fn to_celcius(f: f32) -> f32 {
    (f - 32.0) / 1.8
}

fn to_fahrenheit(c: f32) -> f32 {
    c * 1.8 + 32.0
}

fn main() {
    let fahrenheit = 80.0;
    let celcius = 27.0;
    println!(
        "{} Fahrenheit to {} Celcius!",
        fahrenheit,
        to_celcius(fahrenheit)
    );
    println!(
        "{} Celcius to {} Fahrenheit!",
        celcius,
        to_fahrenheit(celcius)
    );
}
