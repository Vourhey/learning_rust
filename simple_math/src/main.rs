use std::io::{self, Write};

fn main() {
    print!("What is the first number? ");
    io::stdout().flush().expect("Didn't flush");
    let mut first_number = String::new();
    io::stdin().read_line(&mut first_number)
        .expect("Failed to read from stdin!");

    print!("What is the second number? ");
    io::stdout().flush().expect("Didn't flush");
    let mut second_number = String::new();
    io::stdin().read_line(&mut second_number)
        .expect("Failed to read from stdin!");

    let f = first_number.trim().parse::<i64>().unwrap();
    let s = second_number.trim().parse::<i64>().unwrap();

    println!("{} + {} = {}", f, s, f + s);
    println!("{} - {} = {}", f, s, f - s);
    println!("{} * {} = {}", f, s, f * s);
    println!("{} / {} = {}", f, s, f / s);
}
